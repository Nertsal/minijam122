use super::*;

pub struct Menu {
    geng: Geng,
    assets: Rc<Assets>,
    camera: Camera,
    framebuffer_size: Vec2<usize>,
    transition: Option<geng::Transition>,
    time: f32,
}

impl Menu {
    pub fn new(geng: &Geng, assets: &Rc<Assets>) -> Self {
        Self {
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
        draw_gltf(gltf, matrix, &*self.assets, framebuffer, &self.camera)
    }
}

impl geng::State for Menu {
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        self.framebuffer_size = framebuffer.size();
        ugli::clear(framebuffer, Some(Rgba::BLACK), Some(1.0), None);

        // Level in the background
        self.draw_gltf(&self.assets.level, Mat4::identity(), framebuffer);
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
