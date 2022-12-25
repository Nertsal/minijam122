use super::*;

pub struct Menu {
    geng: Geng,
    assets: Rc<Assets>,
    render: Render,
    framebuffer_size: Vec2<usize>,
    transition: Option<geng::Transition>,
    time: f32,
}

impl Menu {
    pub fn new(geng: &Geng, assets: &Rc<Assets>) -> Self {
        Self {
            render: Render::new(geng, assets),
            geng: geng.clone(),
            assets: assets.clone(),
            framebuffer_size: vec2(1, 1),
            transition: None,
            time: 0.0,
        }
    }
}

impl geng::State for Menu {
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        self.framebuffer_size = framebuffer.size();
        self.render
            .draw_world(&Player::new(), &Control::Disabled, framebuffer);
    }

    fn update(&mut self, delta_time: f64) {
        let delta_time = delta_time as f32;
        self.time += delta_time;

        let t = self.time * 0.1;
        self.render.camera.rot_h = t.sin();
        self.render.camera.pos =
            vec3(17.0, -5.0, 1.0) + vec2(0.0, -5.0).rotate(t.sin()).extend(0.0);
    }

    fn transition(&mut self) -> Option<geng::Transition> {
        self.transition.take()
    }

    fn ui<'a>(&'a mut self, cx: &'a geng::ui::Controller) -> Box<dyn geng::ui::Widget + 'a> {
        use geng::ui::*;

        let play = ui::Button::new(cx, "PLAY", self.framebuffer_size.y as f32 * 0.1);
        if play.was_clicked() {
            self.transition = Some(geng::Transition::Push(Box::new(Game::new(
                &self.geng,
                &self.assets,
                self.render.camera.clone(),
            ))));
        }

        // Box::new(geng::ui::column![play].padding(0.5, 0.1, 0.05, 0.1))
        let fb = self.framebuffer_size.map(|x| x as f64);
        Box::new(play.fixed_size(fb * 0.2).align(vec2(0.5, 1.0)).padding(
            fb.y * 0.8,
            fb.x * 0.1,
            fb.y * 0.05,
            fb.x * 0.1,
        ))
    }
}
