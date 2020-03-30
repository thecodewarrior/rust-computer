use std::sync::{Arc, Condvar, Mutex, RwLock};
use std::thread;
use std::time::{Duration, Instant};
// use crate::Computer;
use super::super::Computer; // crate:: doesn't seem to work with vscode. possibly related to rust issue#69933?
use super::utils::*;
use spin_sleep::LoopHelper;

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
    let mut loop_helper = LoopHelper::builder()
        .report_interval_s(0.25)
        .native_accuracy_ns(5_000_000)
        .build_without_target_rate(); // we'll set the target rate during the loop
                                      // split up batches so the UI thread has plenty of opportunities to interrupt as they drift out of sync.
    let frame_split = 2.;
    loop {
        loop_helper.loop_start();
        let paused = Arc::clone(&thread_state_lock.read().unwrap().paused);
        if paused.wait_if_paused() {
            // restart the helper if the simulation paused, since it'll have a crazy inaccurate delta.
            loop_helper = LoopHelper::builder()
                .report_interval_s(0.25)
                .native_accuracy_ns(5_000_000)
                .build_without_target_rate();
            loop_helper.loop_start();
        }

        let target_frequency: f64;
        let ui_frequency: f64;
        {
            let thread_state = thread_state_lock.read().unwrap();
            target_frequency = thread_state.frequency;
            ui_frequency = thread_state.ui_frequency * frame_split;
        }

        let updates_per_frame: u32;
        if target_frequency < ui_frequency {
            updates_per_frame = 1;
            loop_helper.set_target_rate(target_frequency as f64);
        } else {
            updates_per_frame = (target_frequency / ui_frequency + 0.5) as u32;
            // calculate loop duration based on the update count, since that's the important thing to keep constant
            loop_helper.set_target_rate(target_frequency / updates_per_frame as f64);
        }

        {
            let mut sim_state = sim_state_lock.write().unwrap();
            for _ in 0..updates_per_frame {
                sim_state.computer.tick();
            }
        }

        if let Some(ups) = loop_helper.report_rate() {
            let mut thread_state = thread_state_lock.write().unwrap();
            thread_state.actual_frequency = updates_per_frame as f64 * ups;
        }

        loop_helper.loop_sleep();
    }
}
