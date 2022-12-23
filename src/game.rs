use super::*;

mod draw;
mod handle_event;
mod update;

type Time = R32;

pub struct Game {
    geng: Geng,
    assets: Rc<Assets>,
    framebuffer_size: Vec2<usize>,
    camera: Camera,
    controlling_camera: bool,
    player: Player,
    control: Control,
}

enum Control {
    Disabled,
    Direction,
    Power { direction: Vec2<Coord>, time: Time },
}

impl Game {
    pub fn new(geng: &Geng, assets: &Rc<Assets>) -> Self {
        Self {
            geng: geng.clone(),
            assets: assets.clone(),
            framebuffer_size: vec2(1, 1),
            camera: Camera {
                fov: f32::PI / 3.0,
                pos: vec3(0.0, 0.0, 1.0),
                distance: 5.0,
                rot_h: 0.0,
                rot_v: f32::PI / 3.0,
            },
            controlling_camera: false,
            player: Player {
                id: Id(0),
                position: Vec3::ZERO,
                velocity: Vec3::ZERO,
                radius: Coord::new(0.1),
                color: Rgba::RED,
                last_shot: Vec3::ZERO,
            },
            control: Control::Disabled,
        }
    }
}

impl geng::State for Game {
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        self.draw(framebuffer)
    }

    fn handle_event(&mut self, event: geng::Event) {
        self.handle_event(event)
    }

    fn fixed_update(&mut self, delta_time: f64) {
        let delta_time = Time::new(delta_time as f32);
        self.update(delta_time)
    }
}
