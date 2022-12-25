use super::*;

impl Game {
    pub fn ui<'a>(&'a mut self, cx: &'a geng::ui::Controller) -> Box<dyn geng::ui::Widget + 'a> {
        use geng::ui::*;

        if self.player.finished {
            let fb = self.framebuffer_size.map(|x| x as f64);
            let font_size = fb.y as f32 * 0.1;

            let play_button = crate::ui::Button::new(cx, "PLAY AGAIN", font_size);
            if play_button.was_clicked() {
                self.reset();
            }

            let finish_text = crate::ui::Text::new(
                "YOU FINISHED!",
                self.geng.default_font(),
                font_size,
                Rgba::BLACK,
            );
            let (mins, secs, mils) = time(self.run_time.as_f32());
            let hits_text = crate::ui::Text::new(
                format!("hits: {}", self.player.hits),
                self.geng.default_font(),
                font_size,
                Rgba::BLACK,
            );
            let deaths_text = crate::ui::Text::new(
                format!("losses: {}", self.player.deaths),
                self.geng.default_font(),
                font_size,
                Rgba::BLACK,
            );
            let time_text = crate::ui::Text::new(
                format!("{:02}:{:02}.{:03.0}", mins, secs, mils),
                self.geng.default_font(),
                font_size,
                Rgba::BLACK,
            );

            Box::new(
                geng::ui::column![
                    geng::ui::column![
                        finish_text, //.padding_bottom(fb.y * 0.15),
                        hits_text,   //.padding_bottom(fb.y * 0.1),
                        deaths_text, //.padding_bottom(fb.y * 0.1),
                        time_text,   //.padding_bottom(fb.y * 0.1)
                    ]
                    .padding_bottom(fb.y * 0.1),
                    play_button
                ]
                .padding(fb.y * 0.2, fb.x * 0.1, fb.y * 0.05, fb.x * 0.6),
            )
        } else {
            Box::new(Void)
        }
    }
}
