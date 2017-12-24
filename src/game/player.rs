extern crate websocket;

use super::Arena;
use super::command::{PlayerCmd, Position};

use std::net::TcpStream;
use std::cmp::Ordering;
use cgmath::{Point2, Vector2, Zero};

pub const DEFAULT_RADIUS: f32 = 16.0;
pub const DEFAULT_MASS: f32 = 100.0;
pub const MOVE_ACCELERATION: f32 = 1000.0;
pub const FRICTION_ACCELERATION: f32 = 400.0;
pub const MAX_VELOCITY: f32 = 100.0;

pub struct Player {
    pub id: String,
    pub sender: websocket::sender::Writer<TcpStream>,

    pub body: PlayerBody,

    // Actions
    pub move_to: Vector2<i16>
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
            },
//            mass: PLAYER_RADIUS * PLAYER_RADIUS,
            // Actions
            move_to: Vector2::zero(),
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

    /// Reset move vector
    fn reset_move_to(&mut self) {
        self.move_to.x = 0;
        self.move_to.y = 0;
    }

    /// Update player state
    pub fn update(&mut self, dt: f32) {
        self.update_velocity(dt);
        self.body.pos.x += self.body.velocity.x * dt;
        self.body.pos.y += self.body.velocity.y * dt;

        self.reset_move_to();
    }

    fn update_velocity(&mut self, dt: f32) {
        // Applying friction
        if self.body.velocity.x.abs() > 0.0 {
            if self.body.velocity.x > 0.0 {
                self.body.velocity.x -= FRICTION_ACCELERATION * dt;
                if self.body.velocity.x < 0.0 {
                    self.body.velocity.x = 0.0;
                }
            } else {
                self.body.velocity.x += FRICTION_ACCELERATION * dt;
                if self.body.velocity.x > 0.0 {
                    self.body.velocity.x = 0.0;
                }
            }
        }

        if self.body.velocity.y.abs() > 0.0 {
            if self.body.velocity.y > 0.0 {
                self.body.velocity.y -= FRICTION_ACCELERATION * dt;
                if self.body.velocity.y < 0.0 {
                    self.body.velocity.y = 0.0;
                }
            } else {
                self.body.velocity.y += FRICTION_ACCELERATION * dt;
                if self.body.velocity.y > 0.0 {
                    self.body.velocity.y = 0.0;
                }
            }
        }

        // Applying acceleration
        self.body.velocity.x += self.move_to.x as f32 * MOVE_ACCELERATION * dt;
        self.body.velocity.y += self.move_to.y as f32 * MOVE_ACCELERATION * dt;
        if self.body.velocity.x > MAX_VELOCITY {
            self.body.velocity.x = MAX_VELOCITY;
        } else if self.body.velocity.x < -MAX_VELOCITY {
            self.body.velocity.x = -MAX_VELOCITY
        }
        if self.body.velocity.y > MAX_VELOCITY {
            self.body.velocity.y = MAX_VELOCITY;
        } else if self.body.velocity.y < -MAX_VELOCITY {
            self.body.velocity.y = -MAX_VELOCITY
        }
    }

    pub fn generate_cmd(&self) -> PlayerCmd {
        PlayerCmd {
            player_id: self.id.clone(),
            it_is_you: false,
            position: Position {
                x: self.body.pos.x,
                y: self.body.pos.y,
            }
        }
    }
}

pub struct PlayerBody {
    pub pos: Point2<f32>,
    pub velocity: Vector2<f32>,
    pub acceleration: Vector2<f32>,
    pub r: f32,
    pub mass: f32,
}