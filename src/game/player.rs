extern crate websocket;

use super::Arena;
use super::command::{PlayerCmd, Position};

use std::net::TcpStream;
use std::cmp::Ordering;
use cgmath::{Point2, Vector2, Zero};

pub const PLAYER_RADIUS: f32 = 16.0;
pub const MOVE_ACCELERATION: f32 = 1000.0;
pub const FRICTION_ACCELERATION: f32 = 400.0;
pub const MAX_VELOCITY: f32 = 100.0;

pub struct Player {
    pub id: String,
    pub sender: websocket::sender::Writer<TcpStream>,

    pub pos: Point2<f32>,
    pub velocity: Vector2<f32>,
    pub acceleration: Vector2<f32>,
    pub r: f32,
//    pub mass: f32,
    // Actions
    pub move_to: Vector2<i16>
}

impl Player {
    pub fn new(id: String, sender: websocket::sender::Writer<TcpStream>) -> Player {
        Player {
            id,
            sender,
            pos: Point2::new(0.0, 0.0),
            velocity: Vector2::zero(),
            acceleration: Vector2::zero(),

            r: PLAYER_RADIUS,
//            mass: PLAYER_RADIUS * PLAYER_RADIUS,
            // Actions
            move_to: Vector2::zero(),
        }
    }

    pub fn set_position(&mut self, x: f32, y: f32) {
        self.pos.x = x;
        self.pos.y = y;
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
//        println!("move_to {:?}", self.move_to);
//        println!("velocity {:?}", self.velocity);
        self.pos.x += self.velocity.x * dt;
        self.pos.y += self.velocity.y * dt;

        self.reset_move_to();
    }

    fn update_velocity(&mut self, dt: f32) {
        // Applying friction
        if self.velocity.x.abs() > 0.0 {
            if self.velocity.x > 0.0 {
                self.velocity.x -= FRICTION_ACCELERATION * dt;
                if self.velocity.x < 0.0 {
                    self.velocity.x = 0.0;
                }
            } else {
                self.velocity.x += FRICTION_ACCELERATION * dt;
                if self.velocity.x > 0.0 {
                    self.velocity.x = 0.0;
                }
            }
        }

        if self.velocity.y.abs() > 0.0 {
            if self.velocity.y > 0.0 {
                self.velocity.y -= FRICTION_ACCELERATION * dt;
                if self.velocity.y < 0.0 {
                    self.velocity.y = 0.0;
                }
            } else {
                self.velocity.y += FRICTION_ACCELERATION * dt;
                if self.velocity.y > 0.0 {
                    self.velocity.y = 0.0;
                }
            }
        }

        // Applying acceleration
        self.velocity.x += self.move_to.x as f32 * MOVE_ACCELERATION * dt;
        self.velocity.y += self.move_to.y as f32 * MOVE_ACCELERATION * dt;
        if self.velocity.x > MAX_VELOCITY {
            self.velocity.x = MAX_VELOCITY;
        } else if self.velocity.x < -MAX_VELOCITY {
            self.velocity.x = -MAX_VELOCITY
        }
        if self.velocity.y > MAX_VELOCITY {
            self.velocity.y = MAX_VELOCITY;
        } else if self.velocity.y < -MAX_VELOCITY {
            self.velocity.y = -MAX_VELOCITY
        }
    }

    pub fn generate_cmd(&self) -> PlayerCmd {
        PlayerCmd {
            player_id: self.id.clone(),
            it_is_you: false,
            position: Position {
                x: self.pos.x,
                y: self.pos.y,
            }
        }
    }
}