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
            render: {
                let mut render = Render::new(geng, assets);
                render.camera = Camera {
                    fov: f32::PI / 5.0,
                    pos: vec3(17.0, -10.0, 1.0),
                    distance: 60.0,
                    rot_h: 0.0,
                    rot_v: f32::PI / 6.0,
                };
                render
            },
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
        use button::Button;
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
        Box::new(play.fixed_size(fb * 0.2).align(vec2(0.5, 1.0)).padding(
            fb.y * 0.8,
            fb.x * 0.1,
            fb.y * 0.05,
            fb.x * 0.1,
        ))
    }
}

mod button {
    use geng::prelude::*;
    use geng::ui;
    use geng::ui::*;

    pub struct Button<'a> {
        sense: &'a mut Sense,
        clicked: bool,
        inner: Box<dyn Widget + 'a>,
        f: Option<Box<dyn FnMut() + 'a>>,
    }

    impl<'a> Button<'a> {
        pub fn new(cx: &'a Controller, text: &str) -> Self {
            let sense: &'a mut Sense = cx.get_state();
            let text = Text::new(
                text.to_owned(),
                cx.theme().font.clone(),
                cx.theme().text_size,
                if sense.is_hovered() {
                    cx.theme().hover_color
                } else {
                    cx.theme().usable_color
                },
            )
            .shrink(if sense.is_captured() {
                cx.theme().press_ratio as f64
            } else {
                0.0
            });
            let mut ui = ui::stack![text];
            if sense.is_hovered() {
                ui.push(Box::new(
                    ColorBox::new(cx.theme().hover_color)
                        .constraints_override(Constraints {
                            min_size: vec2(0.0, 1.0),
                            flex: vec2(1.0, 0.0),
                        })
                        .flex_align(vec2(Some(0.0), Some(0.0)), vec2(0.5, 0.0)),
                ));
            }
            Self {
                clicked: sense.take_clicked(),
                sense,
                inner: Box::new(ui),
                f: None,
            }
        }
        pub fn was_clicked(&self) -> bool {
            self.clicked
        }
    }

    impl Widget for Button<'_> {
        fn sense(&mut self) -> Option<&mut Sense> {
            Some(self.sense)
        }
        fn calc_constraints(&mut self, cx: &ConstraintsContext) -> Constraints {
            cx.get_constraints(&self.inner)
        }
        fn walk_children_mut(&mut self, mut f: Box<dyn FnMut(&mut dyn Widget) + '_>) {
            f(&mut self.inner);
        }
        fn layout_children(&mut self, cx: &mut LayoutContext) {
            cx.set_position(&self.inner, cx.position);
        }
        fn handle_event(&mut self, event: &geng::Event) {
            #![allow(unused_variables)]
            if let Some(f) = &mut self.f {
                if self.sense.take_clicked() {
                    f();
                }
            }
        }
    }

    use geng::Font;

    pub struct Text<T: AsRef<str>, F: AsRef<Font>> {
        text: T,
        font: F,
        size: f32,
        color: Rgba<f32>,
    }

    impl<T: AsRef<str>, F: AsRef<Font>> Text<T, F> {
        pub fn new(text: T, font: F, size: f32, color: Rgba<f32>) -> Self {
            Self {
                text,
                font,
                size,
                color,
            }
        }
    }

    impl<T: AsRef<str>, F: AsRef<Font>> Widget for Text<T, F> {
        fn calc_constraints(&mut self, _cx: &ConstraintsContext) -> Constraints {
            Constraints {
                min_size: vec2(
                    self.font
                        .as_ref()
                        .measure_bounding_box(self.text.as_ref())
                        .map(|aabb| aabb.map(|x| x * self.size))
                        .map_or(0.0, |aabb| aabb.width() as f64),
                    self.size as f64,
                ),
                flex: vec2(0.0, 0.0),
            }
        }
        fn draw(&mut self, cx: &mut DrawContext) {
            let font = cx.geng.default_font();
            let size = self.size * cx.position.width() as f32
                / font
                    .measure_bounding_box(self.text.as_ref())
                    .map(|aabb| aabb.map(|x| x * self.size))
                    .map_or(0.0, |aabb| aabb.width());
            font.draw_with_outline(
                cx.framebuffer,
                &geng::PixelPerfectCamera,
                self.text.as_ref(),
                cx.position.center().map(|x| x as f32),
                geng::TextAlign::CENTER,
                size,
                self.color,
                self.size * 0.1,
                self.color.map_rgb(|x| 1.0 - x),
            )
        }
    }
}
