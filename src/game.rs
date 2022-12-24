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
    delayed_input: Option<Time>,
    control: Control,
    quad_geometry: ugli::VertexBuffer<draw_2d::Vertex>,
    outline_texture: RefCell<ugli::Texture>,
    color_texture: RefCell<ugli::Texture>,
    depth_buffer: RefCell<ugli::Renderbuffer<ugli::DepthComponent>>,
}

enum Control {
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
            color_texture: RefCell::new(ugli::Texture::new_uninitialized(geng.ugli(), vec2(1, 1))),
            outline_texture: RefCell::new(ugli::Texture::new_uninitialized(
                geng.ugli(),
                vec2(1, 1),
            )),
            depth_buffer: RefCell::new(ugli::Renderbuffer::new(geng.ugli(), vec2(1, 1))),
            quad_geometry: ugli::VertexBuffer::new_static(
                geng.ugli(),
                vec![
                    draw_2d::Vertex {
                        a_pos: vec2(0.0, 0.0),
                    },
                    draw_2d::Vertex {
                        a_pos: vec2(1.0, 0.0),
                    },
                    draw_2d::Vertex {
                        a_pos: vec2(1.0, 1.0),
                    },
                    draw_2d::Vertex {
                        a_pos: vec2(0.0, 1.0),
                    },
                ],
            ),
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
            player: Player::new(),
            delayed_input: None,
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
