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

pub struct Slider<'a> {
    cx: &'a Controller,
    sense: &'a mut Sense,
    pos: &'a mut Option<AABB<f64>>,
    tick_radius: &'a mut f32,
    value: f64,
    range: RangeInclusive<f64>,
    change: RefCell<&'a mut Option<f64>>,
}

impl<'a> Slider<'a> {
    const ANIMATION_SPEED: f32 = 5.0;

    pub fn new(cx: &'a Controller, value: f64, range: RangeInclusive<f64>) -> Self {
        Slider {
            cx,
            sense: cx.get_state(),
            tick_radius: cx.get_state(),
            pos: cx.get_state(),
            value,
            range,
            change: RefCell::new(cx.get_state()),
        }
    }

    pub fn get_change(&self) -> Option<f64> {
        self.change.borrow_mut().take()
    }
}

impl<'a> Widget for Slider<'a> {
    fn sense(&mut self) -> Option<&mut Sense> {
        Some(self.sense)
    }
    fn update(&mut self, delta_time: f64) {
        let target_tick_radius = if self.sense.is_hovered() || self.sense.is_captured() {
            1.0 / 2.0
        } else {
            1.0 / 6.0
        };
        *self.tick_radius += (target_tick_radius - *self.tick_radius)
            .clamp_abs(Self::ANIMATION_SPEED * delta_time as f32);
    }
    fn draw(&mut self, cx: &mut DrawContext) {
        *self.pos = Some(cx.position);
        let geng = cx.geng;
        let position = cx.position.map(|x| x as f32);
        let line_width = position.height() / 3.0;
        let value_position = if self.range.end() == self.range.start() {
            *self.tick_radius
        } else {
            *self.tick_radius
                + ((self.value - *self.range.start()) / (*self.range.end() - *self.range.start()))
                    as f32
                    * (position.width() - line_width)
        };
        geng.draw_2d(
            cx.framebuffer,
            &geng::PixelPerfectCamera,
            &draw_2d::Quad::new(
                AABB::from_corners(
                    position.bottom_left()
                        + vec2(value_position, (position.height() - line_width) / 2.0),
                    position.top_right()
                        - vec2(line_width / 2.0, (position.height() - line_width) / 2.0),
                ),
                cx.theme.usable_color,
            ),
        );
        geng.draw_2d(
            cx.framebuffer,
            &geng::PixelPerfectCamera,
            &draw_2d::Ellipse::circle(
                position.top_right() - vec2(line_width / 2.0, position.height() / 2.0),
                line_width / 2.0,
                cx.theme.usable_color,
            ),
        );
        geng.draw_2d(
            cx.framebuffer,
            &geng::PixelPerfectCamera,
            &draw_2d::Quad::new(
                AABB::from_corners(
                    position.bottom_left()
                        + vec2(line_width / 2.0, (position.height() - line_width) / 2.0),
                    position.bottom_left()
                        + vec2(value_position, (position.height() + line_width) / 2.0),
                ),
                cx.theme.hover_color,
            ),
        );
        geng.draw_2d(
            cx.framebuffer,
            &geng::PixelPerfectCamera,
            &draw_2d::Ellipse::circle(
                position.bottom_left() + vec2(line_width / 2.0, position.height() / 2.0),
                line_width / 2.0,
                cx.theme.hover_color,
            ),
        );
        geng.draw_2d(
            cx.framebuffer,
            &geng::PixelPerfectCamera,
            &draw_2d::Ellipse::circle(
                position.bottom_left() + vec2(value_position, position.height() / 2.0),
                *self.tick_radius * position.height(),
                cx.theme.hover_color,
            ),
        );
    }
    fn handle_event(&mut self, event: &geng::Event) {
        let aabb = match *self.pos {
            Some(pos) => pos,
            None => return,
        };
        if self.sense.is_captured() {
            if let geng::Event::MouseDown { position, .. }
            | geng::Event::MouseMove { position, .. } = &event
            {
                let position = position.x - aabb.x_min;
                let new_value = *self.range.start()
                    + ((position - aabb.height() / 6.0) / (aabb.width() - aabb.height() / 3.0))
                        .clamp(0.0, 1.0)
                        * (*self.range.end() - *self.range.start());
                **self.change.borrow_mut() = Some(new_value);
            }
        }
    }

    fn calc_constraints(&mut self, _children: &ConstraintsContext) -> Constraints {
        Constraints {
            min_size: vec2(1.0, 1.0) * self.cx.theme().text_size as f64,
            flex: vec2(1.0, 0.0),
        }
    }
}
