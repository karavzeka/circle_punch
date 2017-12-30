use game::{Player, RESTITUTION};

use std::collections::HashMap;
use cgmath::{Vector2, Angle, MetricSpace, InnerSpace, Zero};

pub struct CollisionController {
    players_collisions: HashMap<String, PlayerVsPlayerCollision>
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
        let total_radius = player_1.body.r + player_2.body.r;

        if distance2 <= total_radius.powi(2) {
            // Collision occurred
            let distance = distance2.sqrt();
            let penetration = total_radius - distance;
            let normal = (player_1.body.pos - player_2.body.pos).normalize();

            let relative_velocity = player_1.body.velocity - player_2.body.velocity;
            let velocity_along_normal = relative_velocity.dot(normal);
            if velocity_along_normal > 0.0 {
                return;
            }

            let inv_mass_sum = player_1.body.inv_mass + player_2.body.inv_mass;
            let impulse_scalar = (-(1.0 + RESTITUTION) * velocity_along_normal) / inv_mass_sum;
            let impulse = impulse_scalar * normal;

            let recoil_velocity_1 = impulse * player_1.body.inv_mass;
            let recoil_velocity_2 = impulse * player_2.body.inv_mass * (-1.0);

            // Penetration correction
            let percent = 0.05; // 20% .. 80%
            let slop = 0.01;
            let correction = percent * normal * (0.0f32.max(penetration - slop) / inv_mass_sum);

            let collision_1 = PlayerVsPlayerCollision {
                recoil_velocity: recoil_velocity_1,
                position_correction: correction
            };
            let collision_2 = PlayerVsPlayerCollision {
                recoil_velocity: recoil_velocity_2,
                position_correction: correction * (-1.0)
            };

            self.players_collisions.insert(player_1.id.clone(), collision_1);
            self.players_collisions.insert(player_2.id.clone(), collision_2);
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

/// Player - player collision
struct PlayerVsPlayerCollision {
    recoil_velocity: Vector2<f32>,
    position_correction: Vector2<f32>,
}

impl PlayerVsPlayerCollision {
    /// Modifies the properties of the player using his collision
    pub fn apply(&self, player: &mut Player) {
        player.body.velocity += self.recoil_velocity;
        player.body.pos += self.position_correction;
    }
}
