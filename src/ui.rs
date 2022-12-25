use super::*;

use geng::ui;
use geng::ui::*;

pub struct Button<'a> {
    sense: &'a mut Sense,
    clicked: bool,
    inner: Box<dyn Widget + 'a>,
    f: Option<Box<dyn FnMut() + 'a>>,
}

impl<'a> Button<'a> {
    pub fn new(cx: &'a Controller, text: &str, text_size: f32) -> Self {
        let sense: &'a mut Sense = cx.get_state();
        let text = Text::new(
            text.to_owned(),
            cx.theme().font.clone(),
            text_size,
            if sense.is_hovered() {
                Rgba::BLACK
            } else {
                Rgba::WHITE
            },
        )
        .shrink(if sense.is_captured() {
            cx.theme().press_ratio as f64
        } else {
            0.0
        });
        let ui = ui::stack![text];
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
        let size = self.size;
        // * cx.position.width() as f32
        // / font
        //     .measure_bounding_box(self.text.as_ref())
        //     .map_or(0.0, |aabb| aabb.map(|x| x * self.size).width());
        font.draw_with_outline(
            cx.framebuffer,
            &geng::PixelPerfectCamera,
            self.text.as_ref(),
            cx.position.center().map(|x| x as f32),
            geng::TextAlign::CENTER,
            size,
            self.color,
            size * 0.05,
            self.color.map_rgb(|x| 1.0 - x),
        )
    }
}
