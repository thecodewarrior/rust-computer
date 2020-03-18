#[macro_use] extern crate conrod_core;
extern crate conrod_glium;
#[macro_use] extern crate conrod_winit;
extern crate find_folder;
extern crate glium;

mod cpu;
mod computer;
mod ui;

pub fn main() {
    ui::ui_main();
}