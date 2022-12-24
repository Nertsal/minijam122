use super::*;

impl Game {
    pub fn handle_event(&mut self, event: geng::Event) {
        match event {
            geng::Event::MouseDown { button, position } => match button {
                geng::MouseButton::Left => {
                    if !self.controlling_camera {
                        self.click(position);
                    }
                }
                geng::MouseButton::Middle => {
                    if let Some(pos) = self.raycast_to_mouse(self.geng.window().mouse_pos()) {
                        self.player.position = pos + Vec2::ZERO.extend(self.player.radius);
                        self.player.velocity = Vec3::ZERO;
                    }
                }
                geng::MouseButton::Right => {
                    self.geng.window().lock_cursor();
                    self.controlling_camera = true;
                }
            },
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
            geng::Event::Wheel { delta } => {
                let sensitivity = 0.01;
                self.camera.distance =
                    (self.camera.distance - delta as f32 * sensitivity).clamp(1.0, 7.5);
            }
            geng::Event::KeyDown { key } => match key {
                geng::Key::R => {
                    self.player.position = Vec3::ZERO;
                    self.player.velocity = Vec3::ZERO;
                }
                geng::Key::T => {
                    if let Some(pos) = self.raycast_to_mouse(self.geng.window().mouse_pos()) {
                        self.player.position = pos + Vec2::ZERO.extend(self.player.radius);
                        self.player.velocity = Vec3::ZERO;
                    }
                }
                _ => {}
            },
            _ => {}
        }
    }

    fn click(&mut self, position: Vec2<f64>) {
        match self.control {
            Control::Disabled | Control::Hitting { .. } => {}
            Control::Direction => {
                let direction = self.screen_pos_to_move_dir(position);
                self.control = Control::Power {
                    direction,
                    time: Time::ZERO,
                };
            }
            Control::Power { direction, time } => {
                let power = Coord::new((1.0 - time.as_f32().cos()) * 7.0);
                self.control = Control::Precision {
                    direction,
                    power,
                    time: Time::ZERO,
                };
            }
            Control::Precision {
                direction,
                power,
                time,
            } => {
                let angle = (time * Coord::new(3.0)).sin() * Coord::new(f32::PI / 12.0);
                let direction = direction.rotate(angle);
                self.control = Control::Hitting {
                    time: Time::ZERO,
                    hit: (direction * power).extend(Coord::ZERO),
                };
            }
        }
    }

    pub fn raycast_to_mouse(&self, position: Vec2<f64>) -> Option<Vec3<Coord>> {
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
        cast.map(|cast| (ray.from + ray.dir * cast).map(Coord::new))
    }

    pub fn screen_pos_to_move_dir(&self, position: Vec2<f64>) -> Vec2<Coord> {
        let ray = self.camera.pixel_ray(
            self.framebuffer_size.map(|x| x as f32),
            position.map(|x| x as f32),
        );
        let t = util::intersect_ray_with_plane(
            vec3(0.0, 0.0, 1.0),
            -self.player.position.z.as_f32(),
            ray,
        );
        let raycast = (ray.from + ray.dir * t).map(Coord::new);
        -(raycast.xy() - self.player.position.xy()).normalize_or_zero()
    }
}