use super::*;

const GRAVITY: Vec3<f32> = vec3(0.0, 0.0, -9.8);

impl Game {
    pub fn update(&mut self, delta_time: Time) {
        // Movement
        self.player.velocity += GRAVITY.map(Coord::new) * delta_time;
        self.player.position += self.player.velocity * delta_time;

        // Player-Level collisions
        let player = &mut self.player;
        let level = &self.assets.level;
        for tri in level.iter().flat_map(|mesh| mesh.data.chunks(3)) {
            let v = util::vector_from_triangle(
                [tri[0].a_pos, tri[1].a_pos, tri[2].a_pos].map(|pos| (pos.extend(1.0)).xyz()),
                player.position.map(Coord::as_f32),
            )
            .map(Coord::new);
            let len = v.len();
            if len < player.radius {
                let n = v.normalize_or_zero();
                player.velocity -= n * Vec3::dot(n, player.velocity);
                player.position += n * (player.radius - len);
            }
        }
    }
}
