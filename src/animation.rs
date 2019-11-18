use conrod_core;
use conrod_core::{widget, Positionable, Colorable, Widget, Sizeable, Labelable};

widget_ids! {
    struct Ids {
        circle,
        button,
    }
}

pub struct Animation {
    n_frame: u64,
    ids: Ids,
    click_count: u64,
}

impl Animation {
    pub fn new(ui: &mut conrod_core::Ui) -> Self {
        Animation {
            n_frame: 0,
            ids: Ids::new(ui.widget_id_generator()),
            click_count: 0,
        }
    }

    pub fn next_frame(&mut self, ref mut ui: conrod_core::UiCell) {
        self.n_frame += 1;

        widget::Circle::fill(10.)
            .xy([(((self.n_frame as i64) % 400)-200) as f64, 0.])
            .color(conrod_core::color::RED)
            .set(self.ids.circle, ui);

        for _click in widget::Button::new()
            .mid_top()
            .w_h(100.0, 40.0)
            .label(format!("ボタン: {}", self.click_count).as_str())
            .label_font_size(12)
            .set(self.ids.button, ui) {
            self.click_count += 1;
        }
    }
}
