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
        match self.control {
            Control::Disabled => return,
            Control::Direction { time } => {
                let (sin, cos) = time.sin_cos();
                let direction = vec2(cos, sin);
                self.control = Control::Power {
                    direction,
                    time: Time::ZERO,
                };
            }
            Control::Power { direction, time } => {
                let power = Coord::new((1.0 - time.as_f32().cos()) * 2.5);
                self.player.velocity += (direction * power).extend(Coord::ZERO);
                self.control = Control::Disabled;
            }
        }
    }
}
