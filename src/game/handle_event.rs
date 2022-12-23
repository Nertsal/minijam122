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
            Control::Direction => {
                let direction = self.screen_pos_to_move_dir(self.geng.window().mouse_pos());
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

    pub fn screen_pos_to_move_dir(&self, position: Vec2<f64>) -> Vec2<Coord> {
        let ray = self.camera.pixel_ray(
            self.framebuffer_size.map(|x| x as f32),
            position.map(|x| x as f32),
        );
        let cast = util::intersect_ray_with_closest(
            self.assets
                .level
                .iter()
                .flat_map(|mesh| mesh.data.chunks(3))
                .map(|vs| [vs[0].a_pos, vs[1].a_pos, vs[2].a_pos]),
            0.0,
            ray,
        );
        cast.map(|cast| (ray.dir * cast).xy().map(Coord::new) - self.player.position.xy())
            .unwrap_or(Vec2::ZERO)
    }
}
