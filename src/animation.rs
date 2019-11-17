use conrod_core;
use conrod_core::{widget, Positionable, Colorable, Widget, Point};

widget_ids! {
    struct Ids {
        circle,
    }
}

pub struct Animation {
    n_frame: u64,
    ids: Ids,
}

impl Animation {
    pub fn new(ui: &mut conrod_core::Ui) -> Self {
        Animation {
            n_frame: 0,
            ids: Ids::new(ui.widget_id_generator()),
        }
    }

    pub fn set_ui(&mut self, ref mut ui: conrod_core::UiCell) {
        self.n_frame += 1;

        widget::Circle::fill(10.)
            .xy([(((self.n_frame as i64) % 400)-200) as f64, 0.])
            .color(conrod_core::color::RED)
            .set(self.ids.circle, ui);
    }
}
