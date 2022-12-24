use super::*;

const GRAVITY: Vec3<f32> = vec3(0.0, 0.0, -9.8);

impl Game {
    pub fn update(&mut self, delta_time: Time) {
        // Movement
        self.player.velocity += GRAVITY.map(Coord::new) * delta_time;
        self.player.position += self.player.velocity * delta_time;

        let mut drag = 0.0;

        // Check finish
        if !self.player.finished {
            let finish = vec3(3.911, -4.034, 0.076).map(Coord::new); // Hardcode
            if (self.player.position - finish).len() < self.player.radius {
                self.player.finished = true;
            }
        }

        // Player-Water collisions
        let player = &mut self.player;
        if player.position.z - player.radius < Coord::new(-5.1) {
            self.player_death();
        }

        // Player-Level collisions
        let player = &mut self.player;
        let level = &self.assets.level;
        if let Some(v) = level
            .iter()
            .flat_map(|mesh| mesh.data.chunks(3))
            .map(|tri| {
                util::vector_from_triangle(
                    [tri[0].a_pos, tri[1].a_pos, tri[2].a_pos].map(|pos| (pos.extend(1.0)).xyz()),
                    player.position.map(Coord::as_f32),
                )
                .map(Coord::new)
            })
            .filter(|v| v.len() < player.radius)
            .min_by_key(|v| v.len())
        {
            let len = v.len();
            let bounciness = if player.velocity.len().raw() > 0.1 {
                0.5
            } else {
                0.0
            };
            if len < player.radius {
                let n = v.normalize_or_zero();
                player.velocity -= n * Vec3::dot(n, player.velocity) * Coord::new(1.0 + bounciness);
                player.position += n * (player.radius - len);

                drag = if n.xy() != Vec2::ZERO {
                    0.2
                } else if self.player.velocity.len() > Coord::ONE {
                    1.0
                } else {
                    2.0
                }
            }
        }

        self.player.velocity *= Coord::ONE - Coord::new(drag) * delta_time;

        // Camera interpolation
        let interpolation = 1.0 / 0.5;
        let target_pos = self.player.position.map(Coord::as_f32) + vec3(0.0, 0.0, 1.0);
        let pos = self.camera.pos;
        self.camera.pos += (target_pos - pos) * interpolation * delta_time.as_f32();

        // Check delayed input
        if let Some(time) = &mut self.delayed_input {
            *time -= delta_time;
            if *time <= Time::ZERO {
                self.delayed_input = None;
                self.control();
            }
        }

        // Update control
        match &mut self.control {
            Control::Disabled => {
                if self.player.velocity.len().as_f32() < 0.01 {
                    self.control = Control::Direction;
                }
            }
            _ if self.player.velocity.len().as_f32() > 0.01 => {
                self.control = Control::Disabled;
            }
            Control::Direction => {}
            Control::Power { time, .. } | Control::Precision { time, .. } => {
                *time += delta_time;
            }
            Control::Hitting { time, hit } => {
                *time += delta_time / Time::new(0.3);
                if *time >= Time::ONE {
                    self.player.velocity += *hit;
                    self.player.last_shot = self.player.position;
                    self.player.fatigue = (self.player.fatigue + r32(0.2)).min(R32::ONE);
                    self.player.hits += 1;
                    self.control = Control::Disabled;
                }
            }
        }
    }

    pub fn player_death(&mut self) {
        self.player.position = self.player.last_shot;
        self.player.velocity = Vec3::ZERO;
        self.player.fatigue = R32::ZERO;
        self.player.deaths += 1;
    }
}
