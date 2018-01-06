use game::{Player, RESTITUTION, Wall};
use std::f32;
use std::cmp::Ordering;
use std::collections::HashMap;
use cgmath::{Vector2, MetricSpace, InnerSpace};
use cgmath::num_traits::clamp;

pub struct CollisionController {
    players_collisions: HashMap<String, Vec<Collision>>
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
            let percent = 0.2; // 20% .. 80%
            let slop = 0.01;
            let relative_correction = percent * normal * (0.0f32.max(penetration - slop) / inv_mass_sum);

            let collision_1 = Collision {
                recoil_velocity: recoil_velocity_1,
                relative_pos_correction: relative_correction
            };
            let collision_2 = Collision {
                recoil_velocity: recoil_velocity_2,
                relative_pos_correction: relative_correction * (-1.0)
            };

            self.add_collision(player_1.id.clone(), collision_1);
            self.add_collision(player_2.id.clone(), collision_2);
        }
    }

    pub fn detect_player_vs_wall(&mut self, player: &Player, walls: &Vec<Wall>) {
        // We need collision only with the closest wall
        let mut closest_collision: Option<Collision> = None;
        let mut wall_distance = f32::MAX;

        for wall in walls.iter() {
            let n = player.body.pos - wall.get_center();
            let mut closest = n.clone();
            let half_edge = wall.edge_size / 2.0;

            closest.x = clamp(closest.x, -half_edge, half_edge);
            closest.y = clamp(closest.y, -half_edge, half_edge);

            let mut inside = false;

            if n == closest {
                inside = true;

                // Find closest axis
                if n.x.abs() > n.y.abs() {
                    if closest.x > 0.0 {
                        closest.x = half_edge;
                    } else {
                        closest.x = -half_edge;
                    }
                } else {
                    if closest.y > 0.0 {
                        closest.y = half_edge;
                    } else {
                        closest.y = -half_edge;
                    }
                }
            }

            let distance2 = (n - closest).magnitude2();

            if distance2 > player.body.r.powi(2) && !inside {
                continue;
            }

            match distance2.partial_cmp(&wall_distance) {
                Some(Ordering::Less) => {
                    wall_distance = distance2;
                }
                _ => {
                    // Exists closest wall
                    continue;
                }
            }

            let penetration = player.body.r - distance2.sqrt();

            let impulse_normal;
            if n.y.abs() < closest.y {
                if n.x > 0.0 {
                    impulse_normal = Vector2::new(1.0, 0.0);
                } else {
                    impulse_normal = Vector2::new(-1.0, 0.0);
                }
            } else if n.x.abs() < closest.x {
                if n.y > 0.0 {
                    impulse_normal = Vector2::new(0.0, 1.0);
                } else {
                    impulse_normal = Vector2::new(0.0, -1.0);
                }
            } else {
                impulse_normal = (player.body.pos - (wall.get_center() + closest)).normalize();
            }

            let velocity_along_normal = player.body.velocity.dot(impulse_normal);
            let impulse_scalar = (-(1.0 + RESTITUTION) * velocity_along_normal) / player.body.inv_mass;
            let impulse = impulse_scalar * impulse_normal;
            let recoil_velocity = impulse * player.body.inv_mass;

            // Penetration correction
            let percent = 0.2; // 20% .. 80%
            let slop = 0.01;
            let relative_correction = percent * impulse_normal * (0.0f32.max(penetration - slop) / player.body.inv_mass);

            let collision = Collision {
                recoil_velocity,
                relative_pos_correction: relative_correction
            };

            closest_collision = Some(collision);
        }

        match closest_collision {
            Some(collision) => {
                self.add_collision(player.id.clone(), collision);
            }
            None => {}
        }
    }

    fn add_collision(&mut self, player_id: String, collision: Collision) {
        let collisions = self.players_collisions.entry(player_id).or_insert(Vec::new());
        collisions.push(collision);
    }

    /// Apply collision for a player
    pub fn apply_for_player(&mut self, player: &mut Player) {
        if self.players_collisions.contains_key(&player.id) {
            {
                let collisions = self.players_collisions.get(&player.id).unwrap();
                for collision in collisions {
                    collision.apply(player);
                }
            }
            self.players_collisions.remove(&player.id);
        }
    }
}

/// Player - player collision
struct Collision {
    recoil_velocity: Vector2<f32>,
    relative_pos_correction: Vector2<f32>,
}

impl Collision {
    /// Modifies the properties of the player using his collision
    pub fn apply(&self, player: &mut Player) {
        player.body.velocity += self.recoil_velocity;
        player.body.pos += self.relative_pos_correction * player.body.inv_mass;
    }
}
