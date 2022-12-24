use super::*;

impl Game {
    pub fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        self.render
            .draw_world(&self.player, &self.control, framebuffer);

        // UI
        let camera = geng::Camera2d {
            center: Vec2::ZERO,
            rotation: 0.0,
            fov: 10.0,
        };
        let framebuffer_size = framebuffer.size().map(|x| x as f32);
        let camera_aabb = AABB::point(camera.center).extend_symmetric(
            vec2(
                camera.fov * framebuffer_size.x / framebuffer_size.y,
                camera.fov,
            ) / 2.0,
        );

        if self.player.finished {
            self.geng.default_font().draw_with_outline(
                framebuffer,
                &camera,
                "YOU FINISHED!",
                camera_aabb.center() + vec2(0.0, 2.0),
                geng::TextAlign::CENTER,
                1.0,
                Rgba::BLACK,
                0.02,
                Rgba::WHITE,
            );
            self.geng.default_font().draw_with_outline(
                framebuffer,
                &camera,
                &format!("hits: {}\ndeaths: {}", self.player.hits, self.player.deaths),
                camera_aabb.center() + vec2(0.0, 0.0),
                geng::TextAlign::CENTER,
                1.0,
                Rgba::BLACK,
                0.02,
                Rgba::WHITE,
            );
        } else {
            self.geng.default_font().draw_with_outline(
                framebuffer,
                &camera,
                &format!("Hits: {}", self.player.hits),
                camera_aabb.top_left() + vec2(0.2, -0.5),
                geng::TextAlign::LEFT,
                0.5,
                Rgba::BLACK,
                0.01,
                Rgba::WHITE,
            );
            self.geng.default_font().draw_with_outline(
                framebuffer,
                &camera,
                &format!("Deaths: {}", self.player.deaths),
                camera_aabb.top_left() + vec2(0.2, -1.0),
                geng::TextAlign::LEFT,
                0.5,
                Rgba::BLACK,
                0.01,
                Rgba::WHITE,
            );
            self.geng.default_font().draw_with_outline(
                framebuffer,
                &camera,
                &format!("Fatigue delay: {:.1}s", self.player.fatigue),
                camera_aabb.top_left() + vec2(0.2, -1.5),
                geng::TextAlign::LEFT,
                0.5,
                Rgba::BLACK,
                0.01,
                Rgba::WHITE,
            );
        }
    }
}
