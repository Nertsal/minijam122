use super::*;

impl Game {
    pub fn handle_event(&mut self, event: geng::Event) {
        match event {
            geng::Event::MouseDown {
                button: geng::MouseButton::Right,
                ..
            } => {
                self.geng.window().lock_cursor();
                self.controlling_camera = true;
            }
            geng::Event::MouseDown {
                position,
                button: geng::MouseButton::Left,
            } if !self.controlling_camera => {
                self.click(position);
            }
            geng::Event::MouseUp {
                button: geng::MouseButton::Right,
                ..
            } => {
                self.geng.window().unlock_cursor();
                self.controlling_camera = false;
            }
            geng::Event::MouseMove { delta, .. } if self.controlling_camera => {
                let sensitivity = 0.01;
                self.camera.rot_h += -delta.x as f32 * sensitivity;
                self.camera.rot_v =
                    (self.camera.rot_v + delta.y as f32 * sensitivity).clamp(0.0, f32::PI);
            }
            geng::Event::KeyDown { key: geng::Key::R } => {
                self.player.position = Vec3::ZERO;
                self.player.velocity = Vec3::ZERO;
            }
            _ => {}
        }
    }

    fn click(&mut self, position: Vec2<f64>) {
        let position = position.map(|x| x as f32);
        let dir = position - self.framebuffer_size.map(|x| x as f32 / 2.0);
        self.player.velocity += (dir.normalize_or_zero() * 5.0).extend(0.0).map(Coord::new);
    }
}
