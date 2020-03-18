use std::path::PathBuf;
use conrod_core::{color, widget, Colorable, Positionable, Widget, Ui, UiCell};
use super::support::*;


pub fn ui_main() {
    conrod_launch("Hello Conrod!", 400, 300, App::setup)
}

struct App {
    ids: Ids,
    active: bool
}
impl App {
    fn setup(ui: &mut Ui, assets: PathBuf) -> App {
        let font_path = assets.join("fonts/NotoSans/NotoSans-Regular.ttf"); // !
        ui.fonts.insert_from_file(font_path).unwrap(); // !

        App {
            ids: Ids::new(ui.widget_id_generator()),
            active: false
        }
    }
}

widget_ids!(struct Ids { text }); 
impl ConrodApp for App {
    fn frame(&mut self, ui: &mut UiCell) {
        self.active = !self.active;
        widget::Text::new("Hello World!") // !
            .middle_of(ui.window)
            .color(color::WHITE)
            .font_size(32)
            .set(self.ids.text, ui);
    }
}
