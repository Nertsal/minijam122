use super::*;

pub struct Menu {
    geng: Geng,
    assets: Rc<Assets>,
    camera: Camera,
    framebuffer_size: Vec2<usize>,
    transition: Option<geng::Transition>,
    quad_geometry: ugli::VertexBuffer<draw_2d::Vertex>,
    time: f32,
}

impl Menu {
    pub fn new(geng: &Geng, assets: &Rc<Assets>) -> Self {
        Self {
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
            camera: Camera {
                fov: f32::PI / 5.0,
                pos: vec3(17.0, -10.0, 1.0),
                distance: 60.0,
                rot_h: 0.0,
                rot_v: f32::PI / 6.0,
            },
            framebuffer_size: vec2(1, 1),
            transition: None,
            time: 0.0,
        }
    }

    fn draw_gltf<'a>(
        &'a self,
        gltf: impl IntoIterator<Item = &'a Mesh>,
        matrix: Mat4<Coord>,
        framebuffer: &'a mut ugli::Framebuffer,
    ) {
        draw_gltf(
            gltf,
            &self.assets.shaders.gltf,
            matrix,
            framebuffer,
            &self.camera,
        )
    }
}

impl geng::State for Menu {
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        self.framebuffer_size = framebuffer.size();

        let texture = {
            // TODO: create not on every frame
            let mut postprocess_texture =
                ugli::Texture::new_uninitialized(self.geng.ugli(), framebuffer.size());
            let mut depth_buffer = ugli::Renderbuffer::new(self.geng.ugli(), framebuffer.size());
            let mut framebuffer = ugli::Framebuffer::new(
                self.geng.ugli(),
                ugli::ColorAttachment::Texture(&mut postprocess_texture),
                ugli::DepthAttachment::Renderbuffer(&mut depth_buffer),
            );
            let framebuffer = &mut framebuffer;
            ugli::clear(
                framebuffer,
                Some(Rgba::new(0.8, 0.8, 1.0, 1.0)),
                Some(1.0),
                None,
            );

            // Level in the background
            self.draw_gltf(&self.assets.level, Mat4::identity(), framebuffer);
            postprocess_texture
        };

        ugli::draw(
            framebuffer,
            &self.assets.shaders.postprocess,
            ugli::DrawMode::TriangleFan,
            &self.quad_geometry,
            ugli::uniforms! {
                u_texture_size: texture.size().map(|x| x as f32),
                u_texture: texture,
            },
            ugli::DrawParameters { ..default() },
        );
    }

    fn update(&mut self, delta_time: f64) {
        let delta_time = delta_time as f32;
        self.time += delta_time;

        let t = self.time * 0.1;
        self.camera.rot_h = t.sin();
        self.camera.pos = vec3(17.0, -5.0, 1.0) + vec2(0.0, -5.0).rotate(t.sin()).extend(0.0);
    }

    fn transition(&mut self) -> Option<geng::Transition> {
        self.transition.take()
    }

    fn ui<'a>(&'a mut self, cx: &'a geng::ui::Controller) -> Box<dyn geng::ui::Widget + 'a> {
        use geng::ui::*;

        let play = Button::new(cx, "PLAY");
        if play.was_clicked() {
            self.transition = Some(geng::Transition::Push(Box::new(Game::new(
                &self.geng,
                &self.assets,
            ))));
        }

        // Box::new(geng::ui::column![play].padding(0.5, 0.1, 0.05, 0.1))
        let fb = self.framebuffer_size.map(|x| x as f64);
        Box::new(play.fixed_size(fb * 0.1).align(vec2(0.5, 1.0)).padding(
            fb.y * 0.65,
            fb.x * 0.1,
            fb.y * 0.05,
            fb.x * 0.1,
        ))
    }
}
