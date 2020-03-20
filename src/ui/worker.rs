use std::thread;
use std::sync::RwLock;
use super::super::Computer;

pub struct SimulatorHandle {
    handle: thread::JoinHandle<()>,
    thread_state: RwLock<SimulatorThreadState>,
    sim_state: RwLock<SimulatorState>,
}

impl SimulatorHandle {
    pub fn new() -> SimulatorHandle {
        let thread_state = RwLock::new(SimulatorThreadState::new());
        let sim_state = RwLock::new(SimulatorState::new());
        let handle = thread::spawn(move || {
            run_simulation();
        });

        SimulatorHandle {
            handle, thread_state, sim_state
        }
    }
}

pub struct SimulatorThreadState {
    frequency: f64,
}

impl SimulatorThreadState {
    fn new() -> SimulatorThreadState {
        SimulatorThreadState { 
            frequency: 1. 
        }
    }
}

pub struct SimulatorState {
    computer: Computer
}

impl SimulatorState {
    fn new() -> SimulatorState {
        SimulatorState {
            computer: Computer::new(65536)
        }
    }
}


fn run_simulation() {

}