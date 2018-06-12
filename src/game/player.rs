extern crate websocket;

use super::command::{PlayerCmd, Position};

use std::net::TcpStream;
use std::cmp::Ordering;
use cgmath::{Point2, Vector2, Zero};
use chrono::prelude::{DateTime, Utc};
use chrono::Duration;

pub const DEFAULT_RADIUS: f32 = 16.0;
pub const DEFAULT_MASS: f32 = 100.0;
pub const DEFAULT_HEALTH: f32 = 100.0;
pub const HEALING: f32 = 0.005;
pub const ATTACK_DELAY: i64 = 2000;
pub const MOVE_FORCE: f32 = 30000.0;
pub const FRICTION_FORCE: f32 = 15000.0;
pub const MAX_VELOCITY: f32 = 100.0;
pub const RESTITUTION: f32 = 0.8;

pub struct Player {
    pub id: String,
    pub sender: websocket::sender::Writer<TcpStream>,

    pub body: PlayerBody,

    pub health_max: f32,
    pub health: f32,
    pub is_health_changed: bool,
    pub is_dead: bool,

    healing_counter: u8,

    attack_delay: i64,
    last_attack: DateTime<Utc>,

    // Actions
    pub move_to: Vector2<i16>,
    pub attack: bool,
}

impl Player {
    pub fn new(id: String, sender: websocket::sender::Writer<TcpStream>) -> Player {
        Player {
            id,
            sender,
            body: PlayerBody {
                pos: Point2::new(0.0, 0.0),
                velocity: Vector2::zero(),
                acceleration: Vector2::zero(),
                r: DEFAULT_RADIUS,
                mass: DEFAULT_MASS,
                inv_mass: 1.0 / DEFAULT_MASS,
            },
            health_max: DEFAULT_HEALTH,
            health: DEFAULT_HEALTH,
            is_health_changed: true,
            is_dead: false,
            healing_counter: 0,

            attack_delay: ATTACK_DELAY,
            last_attack: Utc::now().checked_sub_signed(Duration::seconds(ATTACK_DELAY)).unwrap(),

            // Actions
            move_to: Vector2::zero(),
            attack: false,
        }
    }

    pub fn set_position(&mut self, x: f32, y: f32) {
        self.body.pos.x = x;
        self.body.pos.y = y;
    }

    /// Sets the unit vector of the direction of motion
    pub fn set_move_to(&mut self, x: i16, y: i16) {
        match x.cmp(&0) {
            Ordering::Less => self.move_to.x = -1,
            Ordering::Greater => self.move_to.x = 1,
            Ordering::Equal => self.move_to.x = 0,
        }
        match y.cmp(&0) {
            Ordering::Less => self.move_to.y = -1,
            Ordering::Greater => self.move_to.y = 1,
            Ordering::Equal => self.move_to.y = 0,
        }
    }

    pub fn attack(&mut self) {
        let now = Utc::now();
        if now.signed_duration_since(self.last_attack).ge(&Duration::milliseconds(self.attack_delay)) {
            self.attack = true;
            self.last_attack = now;
        }
    }

    /// Take away some health
    pub fn lost_health(&mut self, value: f32) {
        self.is_health_changed = true;

        self.health -= value;
        if self.health <= 0.0 {
            self.is_dead = true;
            self.health = 0.0;
        }
    }

    /// Update player state
    pub fn update(&mut self, dt: f32) {
        // Update moving
        self.update_velocity(dt);
        self.body.pos.x += self.body.velocity.x * dt;
        self.body.pos.y += self.body.velocity.y * dt;

        // Update health
        if self.health < self.health_max {
            self.health += HEALING;
            self.healing_counter += 1;
            if self.healing_counter >= 100 {
                self.is_health_changed = true;
                self.healing_counter = 0;
            }
        }

        // Update attacking
        if self.attack {
            self.attack = false;
        }
    }

    /// Aggregates some factors and calculates current velocity
    fn update_velocity(&mut self, dt: f32) {
        let mut total_force = Vector2::zero();

        let vel_abs_x = self.body.velocity.x.abs();
        let vel_abs_y = self.body.velocity.y.abs();

        // Applying friction.
        if vel_abs_x > 0.0 {
            if self.body.velocity.x > 0.0 {
                total_force.x -= FRICTION_FORCE;
            } else {
                total_force.x += FRICTION_FORCE;
            }
        }
        if vel_abs_y > 0.0 {
            if self.body.velocity.y > 0.0 {
                total_force.y -= FRICTION_FORCE;
            } else {
                total_force.y += FRICTION_FORCE;
            }
        }

        // Applying move force
        if !self.move_to.is_zero() {
            // If velocity >= max velocity then movie force = friction force.
            if vel_abs_x < MAX_VELOCITY {
                total_force.x += self.move_to.x as f32 * MOVE_FORCE;
            } else {
                total_force.x += self.move_to.x as f32 * FRICTION_FORCE;
            }
            if vel_abs_y < MAX_VELOCITY {
                total_force.y += self.move_to.y as f32 * MOVE_FORCE;
            } else {
                total_force.y += self.move_to.y as f32 * FRICTION_FORCE;
            }
        }

        self.body.velocity += total_force * dt / self.body.mass;

        // Disable the noise of velocity near zero velocity which friction force makes.
        if self.move_to.is_zero() && vel_abs_x < 5.0 && vel_abs_y < 5.0 {
            self.body.velocity = Vector2::zero();
        }
    }

    /// Generates the command to send it to client
    pub fn generate_cmd(&mut self) -> PlayerCmd {
        let player = PlayerCmd {
            player_id: self.id.clone(),
            it_is_you: false,
            position: Position {
                x: self.body.pos.x,
                y: self.body.pos.y,
            },
            health_max: match self.is_health_changed {
                true => Some(self.health_max),
                false => None
            },
            health: match self.is_health_changed {
                true => Some(self.health),
                false => None
            },
        };
        self.is_health_changed = false;
        player
    }
}

pub struct PlayerBody {
    pub pos: Point2<f32>,
    pub velocity: Vector2<f32>,
    pub acceleration: Vector2<f32>,
    pub r: f32,
    pub mass: f32,
    pub inv_mass: f32,
}