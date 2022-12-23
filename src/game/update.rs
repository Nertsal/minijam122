use super::*;

const GRAVITY: Vec3<f32> = vec3(0.0, 0.0, -9.8);
const DRAG: f32 = 1.0;

impl Game {
    pub fn update(&mut self, delta_time: Time) {
        // Movement
        self.player.velocity *= Coord::ONE
            - Coord::new(if self.player.velocity.len() > Coord::ONE {
                DRAG
            } else {
                2.0
            }) * delta_time;
        self.player.velocity += GRAVITY.map(Coord::new) * delta_time;
        self.player.position += self.player.velocity * delta_time;

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
            let bounciness = if v.len().raw() > 0.1 { 0.5 } else { 0.0 };
            if len < player.radius {
                let n = v.normalize_or_zero();
                player.velocity -= n * Vec3::dot(n, player.velocity) * Coord::new(1.0 + bounciness);
                player.position += n * (player.radius - len);
            }
        }

        // Camera interpolation
        let interpolation = 1.0 / 0.5;
        let target_pos = self.player.position.map(Coord::as_f32) + vec3(0.0, 0.0, 1.0);
        let pos = self.camera.pos;
        self.camera.pos += (target_pos - pos) * interpolation * delta_time.as_f32();

        // Update control
        match &mut self.control {
            Control::Disabled => {
                if self.player.velocity.len().as_f32() < 0.01 {
                    self.control = Control::Direction;
                }
            }
            Control::Direction => {}
            Control::Power { time, .. } => {
                *time += delta_time;
            }
        }
    }
}
