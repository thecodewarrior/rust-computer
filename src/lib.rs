#![feature(proc_macro_hygiene)]
#[macro_use]
extern crate rust_computer_macros;

mod computer;
mod cpu;
mod ui;
pub use computer::Computer;

pub fn main() {
    ui::ui_main();
    // test_sleep_accuracy();
}

// run tests for system sleep accuracy. Used to determine the correct factor for spin_sleep 
#[allow(unused)]
fn test_sleep_accuracy() {
    test_delta(10000000, 10); // 10.0s * 10 = 100s
    test_delta(1000000, 10);  // 1s * 10 = 10s
    test_delta(100000, 100);  // .1s * 100 = 10s
    test_delta(10000, 1000);  // .01s * 1000 = 10s
    test_delta(1000, 10000);  // .001s * 10000 = 10s
    test_delta(100, 100000);  // .0001s * 100000 = 10s
    test_delta(10, 1000000);  // .00001s * 1000000 = 10s
    test_delta(1, 10000000);  // .000001s * 10000000 = 10s
    // 170s = 2m 50s test
}

fn test_delta(sleep_micros: u64, test_count: u32) {
    let sleep_time = std::time::Duration::from_micros(sleep_micros);
    let sleep_micros = sleep_micros as u128;

    println!("Testing {}μs sleep accuracy. Running test {} times:", sleep_micros, test_count);

    let mut total_delta = 0;
    let mut max_delta = 0;
    let mut min_delta = u128::MAX;
    for _ in 0 .. test_count {
        let start = std::time::Instant::now();
        std::thread::sleep(sleep_time);
        let delta = start.elapsed().as_micros() - sleep_micros;
        total_delta += delta;
        max_delta = std::cmp::max(max_delta, delta);
        min_delta = std::cmp::min(min_delta, delta);
    }

    println!("  - Average delta: {}μs", total_delta / test_count as u128);
    println!("  - Maximum delta: {}μs", max_delta);
    println!("  - Minimum delta: {}μs", min_delta);
}
