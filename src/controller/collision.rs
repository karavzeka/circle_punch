use game::Player;

use std::collections::HashMap;
use cgmath::{Vector2, Angle, MetricSpace, InnerSpace};

pub struct CollisionController {
    players_collisions: HashMap<String, PlayerCollision>
}

impl CollisionController {
    pub fn new() -> CollisionController {
        CollisionController {
            players_collisions: HashMap::new()
        }
    }

    /// Check is there collision between players.
    /// If the collision occurs, the velocity modification vectors are calculated.
    pub fn detect_player_vs_player(&mut self, player_1: &Player, player_2: &Player) {
        let distance2 = player_1.body.pos.distance2(player_2.body.pos);
        if distance2 <= (player_1.body.r + player_2.body.r).powi(2) {
            // Collision occurred
            let norm_to_player_2 = (player_1.body.pos - player_2.body.pos).normalize();
            let velocity_distinction = player_2.body.velocity - player_1.body.velocity;
            let angle = velocity_distinction.angle(norm_to_player_2);
            let power = velocity_distinction.magnitude() * angle.cos().abs();

            let recoil_velocity_1 = norm_to_player_2 * power;
            let recoil_velocity_2 = recoil_velocity_1 * (-1.0);

            self.players_collisions.insert(player_1.id.clone(), PlayerCollision {recoil_velocity: recoil_velocity_1});
            self.players_collisions.insert(player_2.id.clone(), PlayerCollision {recoil_velocity: recoil_velocity_2});
        }
    }

    /// Apply collision for a player
    pub fn apply_for_player(&mut self, player: &mut Player) {
        if self.players_collisions.contains_key(&player.id) {
            {
                let collision = self.players_collisions.get(&player.id).unwrap();
                collision.apply(player);
            }
            self.players_collisions.remove(&player.id);
        }
    }
}

//trait CollisionInstant {
//    fn apply(&self);
//}

/// Player - player collision
struct PlayerCollision {
    recoil_velocity: Vector2<f32>,
}
impl PlayerCollision {
    pub fn apply(&self, player: &mut Player) {
        player.body.velocity += self.recoil_velocity;
    }
}
