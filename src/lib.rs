#![feature(proc_macro_hygiene)]
#[macro_use]
extern crate rust_computer_macros;

#[macro_use]
extern crate conrod_core;
extern crate conrod_glium;
#[macro_use]
extern crate conrod_winit;
extern crate find_folder;
extern crate glium;

mod computer;
mod cpu;
mod ui;
pub use computer::Computer;

pub fn main() {
    ui::ui_main();
}
