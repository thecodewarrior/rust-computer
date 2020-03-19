use super::support::*;
use crate::Computer;
use conrod_core::{color, widget, Colorable, Sizeable, Positionable, Ui, UiCell, Widget};
use conrod_core::text::font;
use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::PathBuf;
use std::time::Duration;

pub fn ui_main() {
    conrod_launch("Hello Conrod!", 600, 400, App::setup)
}

struct App {
    ids: Ids,
    fonts: Fonts,
    computer: Computer,
}

impl App {
    fn setup(ui: &mut Ui, assets: PathBuf) -> App {
        let args: Vec<String> = env::args().collect();

        let font_path = assets.join("fonts/NotoSans/NotoSans-Regular.ttf");
        let fonts = Fonts {
            mono: ui.fonts.insert_from_file(assets.join("fonts/SourceCodePro/SourceCodePro-Regular.ttf")).unwrap(),
            normal: ui.fonts.insert_from_file(assets.join("fonts/NotoSans/NotoSans-Regular.ttf")).unwrap(),
        };

        let mut computer = Computer::new(65536);

        let mut f = File::open(&args[1]).expect("No file");
        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer);
        let limit = std::cmp::min(computer.memory.data.len(), buffer.len());
        computer.memory.data[..limit].copy_from_slice(&buffer[..limit]);

        App {
            ids: Ids::new(ui.widget_id_generator()),
            fonts,
            computer,
        }
    }
}

struct Fonts {
    normal: font::Id,
    mono: font::Id
}
widget_ids!(struct Ids {
    program_counter, 
    stack_reference_width,
    stack,
});
impl ConrodApp for App {
    fn frame(&mut self, ui: &mut UiCell) {
        self.computer.tick();

        widget::Text::new(&format!(
            "PC: {:08x}",
            self.computer.cpu.program_counter.address
        ))
        .top_left_of(ui.window)
        .color(color::WHITE)
        .font_size(12)
        .set(self.ids.program_counter, ui);

        let (mut items, _) = widget::List::flow_down(self.computer.cpu.stack.len())
        .item_size(14.)
        .top_right_of(ui.window)
        .h_of(ui.window)
        .set(self.ids.stack, ui);

        while let Some(item) = items.next(ui) {
            let i = item.i;
            let item_text = format!("{:08x}", self.computer.cpu.stack[i]);
            let label = widget::Text::new(&item_text)
            .font_size(12)
            .font_id(self.fonts.mono)
            .color(color::WHITE);

            item.set(label, ui);
        }
    }

    fn frame_time(&self) -> Duration {
        Duration::from_secs_f32(1. / 2.)
    }
}
