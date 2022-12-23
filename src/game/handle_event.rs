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
            _ => {}
        }
    }
}
