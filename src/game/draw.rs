use super::*;

impl Game {
    pub fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        self.framebuffer_size = framebuffer.size();
        self.render
            .draw_world(&self.player, &self.control, framebuffer);

        {
            // Blinking animation
            ugli::draw(
                framebuffer,
                &self.assets.shaders.blink,
                ugli::DrawMode::TriangleFan,
                &self.render.quad_geometry,
                ugli::uniforms! {
                    u_closed: self.eyes_fatigue.as_f32(),
                },
                ugli::DrawParameters {
                    blend_mode: Some(ugli::BlendMode::default()),
                    ..default()
                },
            );
        }

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
        // let camera_pos =
        //     |relative: Vec2<_>| camera_aabb.bottom_left() + camera_aabb.size() * relative;

        if !self.player.finished {
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
                &format!("Losses: {}", self.player.deaths),
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

        if self.show_timer {
            let (mins, secs, mils) = time(self.run_time.as_f32());
            self.geng.default_font().draw_with_outline(
                framebuffer,
                &camera,
                &format!("{:02}:{:02}.{:03.0}", mins, secs, mils),
                camera_aabb.top_right() + vec2(-0.2, -0.5),
                geng::TextAlign::RIGHT,
                0.5,
                Rgba::BLACK,
                0.01,
                Rgba::WHITE,
            );
        }
    }
}

pub fn time(secs: f32) -> (u32, u32, f32) {
    let mut time = secs;
    let mut mins = 0;
    while time > 60.0 {
        mins += 1;
        time -= 60.0;
    }
    let secs = time.floor() as u32;
    let mils = time.fract() * 1000.0;
    (mins, secs, mils)
}
