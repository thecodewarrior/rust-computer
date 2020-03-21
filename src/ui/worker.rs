use std::sync::{Arc, Condvar, Mutex, RwLock};
use std::thread;
use std::time::{Duration, Instant};
// use crate::Computer;
use super::super::Computer; // crate:: doesn't seem to work with vscode. possibly related to rust issue#69933?
use super::utils::*;

pub struct SimulatorHandle {
    pub handle: thread::JoinHandle<()>,
    pub thread_state: Arc<RwLock<SimulatorThreadState>>,
    pub sim_state: Arc<RwLock<SimulatorState>>,
}
pub struct SimulatorThreadState {
    pub frequency: f64,
    pub actual_frequency: f64,
    /// how many times the UI updates every second
    pub ui_frequency: f64,
    pub paused: Arc<PauseState>,
}
pub struct SimulatorState {
    pub computer: Computer,
}

impl SimulatorHandle {
    pub fn new() -> SimulatorHandle {
        let thread_state = Arc::new(RwLock::new(SimulatorThreadState {
            frequency: 5.,
            ui_frequency: 1.,
            actual_frequency: 0.,
            paused: Arc::new(PauseState::new(true)),
        }));
        let sim_state = Arc::new(RwLock::new(SimulatorState {
            computer: Computer::new(65536),
        }));

        let thread_state_clone = Arc::clone(&thread_state);
        let sim_state_clone = Arc::clone(&sim_state);
        let handle = thread::spawn(move || {
            run_simulation(thread_state_clone, sim_state_clone);
        });

        SimulatorHandle {
            handle,
            thread_state,
            sim_state,
        }
    }
}

fn run_simulation(
    thread_state_lock: Arc<RwLock<SimulatorThreadState>>,
    sim_state_lock: Arc<RwLock<SimulatorState>>,
) {
    // split up batches so the UI thread has plenty of opportunities to interrupt as they drift out of sync.
    let frame_split = 5.;
    let mut batch_size = 1;
    let mut last_end = Instant::now();
    loop {
        let target_frequency: f64;
        let ui_frequency: f64;
        let paused = Arc::clone(&thread_state_lock.read().unwrap().paused);
        paused.wait_if_paused();

        {
            let thread_state = thread_state_lock.read().unwrap();
            target_frequency = thread_state.frequency;
            ui_frequency = thread_state.ui_frequency * frame_split;
        }

        let start_time = Instant::now();
        {
            let mut sim_state = sim_state_lock.write().unwrap();
            for _ in 0 .. batch_size {
                sim_state.computer.tick();
            }
        }
        let end_time = Instant::now();
        {
            let mut thread_state = thread_state_lock.write().unwrap();
            thread_state.actual_frequency = batch_size as f64 / end_time.duration_since(last_end).as_secs_f64();
            last_end = end_time;
        }

        let batch_time = end_time.duration_since(start_time);

        let target_update_interval = 1. / target_frequency;
        let ui_update_interval = 1. / ui_frequency;

        let target_batch_size = ui_update_interval / target_update_interval; // This is the number of updates we want to run per UI update
        let single_update_time = batch_time.as_secs_f64() / batch_size as f64;
        let max_batch_size = ui_update_interval / single_update_time ; // Maximum number of update loops per UI update, as an integer
        if max_batch_size > target_batch_size {
            batch_size = target_batch_size as u32;
            let batch_rounding_compensation = batch_size as f64 / target_batch_size;
            let target_loop_duration = Duration::from_secs_f64(ui_update_interval * batch_rounding_compensation);
            if target_loop_duration > batch_time {
                thread::sleep(target_loop_duration - batch_time);
            }
        } else if batch_size < 1 {
            batch_size = 1;
            thread::sleep(Duration::from_secs_f64(target_update_interval) - Instant::now().duration_since(start_time));
        }
    }
}
