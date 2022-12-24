use super::*;

mod draw;
mod handle_event;
mod update;

pub use draw::*;

type Time = R32;

pub struct Game {
    geng: Geng,
    assets: Rc<Assets>,
    render: Render,
    framebuffer_size: Vec2<usize>,
    controlling_camera: bool,
    player: Player,
    delayed_input: Option<Time>,
    control: Control,
    time: Time,
    show_timer: bool,
}

pub enum Control {
    Disabled,
    Direction,
    Power {
        direction: Vec2<Coord>,
        time: Time,
    },
    Precision {
        direction: Vec2<Coord>,
        power: Coord,
        time: Time,
    },
    Hitting {
        time: Time,
        hit: Vec3<Coord>,
    },
}

impl Game {
    pub fn new(geng: &Geng, assets: &Rc<Assets>) -> Self {
        Self {
            render: Render::new(geng, assets),
            geng: geng.clone(),
            assets: assets.clone(),
            framebuffer_size: vec2(1, 1),
            controlling_camera: false,
            player: Player::new(),
            delayed_input: None,
            control: Control::Disabled,
            time: Time::ZERO,
            show_timer: false,
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
