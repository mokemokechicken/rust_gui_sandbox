use rand::prelude::*;

use conrod_core;
use conrod_core::{widget, Positionable, Colorable, Widget, Sizeable, Labelable};

widget_ids! {
    struct Ids {
        circles[],
        button_up,
        button_down,
    }
}

pub struct Animation {
    n_frame: u64,
    ids: Ids,
    balls: Vec<Ball>,
    thread_rng: ThreadRng,
}

impl Animation {
    pub fn new(ui: &mut conrod_core::Ui) -> Self {
        Animation {
            n_frame: 0,
            ids: Ids::new(ui.widget_id_generator()),
            balls: Vec::new(),
            thread_rng: rand::thread_rng(),
        }
    }

    pub fn next_frame(&mut self, ref mut ui: conrod_core::UiCell) {
        self.n_frame += 1;
        let (width, height) = (ui.win_w, ui.win_h);

        for (i, ball) in self.balls.iter_mut().enumerate() {
            ball.x += ball.dx;
            ball.y += ball.dy;
            if ball.x < -width/2. || width/2. < ball.x {
                ball.dx = -ball.dx;
            }
            if ball.y < -height/2. || height/2. < ball.y {
                ball.dy = -ball.dy;
            }

            widget::Circle::fill(10.)
                .x_y(ball.x, ball.y)
                .color(conrod_core::color::RED)
                .set(self.ids.circles[i], ui);
        }

        for _click in widget::Button::new()
            .mid_top()
            .w_h(100.0, 40.0)
            .label("↑増やす")
            .label_font_size(12)
            .set(self.ids.button_up, ui) {
            self.balls.push(Ball::new(&mut self.thread_rng, width, height));
            self.ids.circles.resize(self.balls.len(), &mut ui.widget_id_generator());
        }

        for _click in widget::Button::new()
            .w_h(100.0, 40.0)
            .label("↓減らす")
            .label_font_size(12)
            .set(self.ids.button_down, ui) {
            self.balls.pop();
            self.ids.circles.resize(self.balls.len(), &mut ui.widget_id_generator());
        }
    }
}

struct Ball {
    x: f64,
    y: f64,
    dx: f64,
    dy: f64,
}

impl Ball {
    fn new(rng: &mut ThreadRng, width: f64, height: f64) -> Self {
        Ball {
            x: rng.gen_range(-width/2., width/2.),
            y: rng.gen_range(-height/2., height/2.),
            dx: rng.gen_range(-3., 3.),
            dy: rng.gen_range(-3., 3.),
        }
    }
}
