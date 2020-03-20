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
    pub paused: PauseState,
}
pub struct SimulatorState {
    pub computer: Computer,
}

impl SimulatorHandle {
    pub fn new() -> SimulatorHandle {
        let thread_state = Arc::new(RwLock::new(SimulatorThreadState {
            frequency: 1.,
            paused: PauseState::new(true),
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
    loop {
        let target_duration: Duration;
        {
            let thread_state = thread_state_lock.read().unwrap();
            thread_state.paused.wait_if_paused();
            target_duration = Duration::from_secs_f64(1. / thread_state.frequency);
        }

        let start_time = Instant::now();
        {
            let mut sim_state = sim_state_lock.write().unwrap();
            sim_state.computer.tick();
        }

        let end_time = Instant::now();
        let delta = end_time.duration_since(start_time);
        if delta < target_duration {
            std::thread::sleep(target_duration - delta);
        }
    }
}
