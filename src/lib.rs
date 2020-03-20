#![feature(proc_macro_hygiene)]
#[macro_use]
extern crate rust_computer_macros;

mod computer;
mod cpu;
mod ui;
pub use computer::Computer;

pub fn main() {
    ui::ui_main();
}
