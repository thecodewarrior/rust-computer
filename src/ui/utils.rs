use std::sync::{Condvar, Mutex};
use std::time::{Duration, Instant};
use std::cmp::{max, min};
use std::thread;

pub struct PauseState {
    mutex: Mutex<bool>,
    condvar: Condvar,
}

impl PauseState {
    pub fn new(state: bool) -> PauseState {
        PauseState {
            mutex: Mutex::new(state),
            condvar: Condvar::new(),
        }
    }

    pub fn wait_if_paused(&self) -> bool {
        let mut paused = self.mutex.lock().unwrap();
        let did_pause = *paused;
        while *paused {
            paused = self.condvar.wait(paused).unwrap();
        }
        did_pause
    }

    pub fn set_paused(&self, new_state: bool) {
        let mut state = self.mutex.lock().unwrap();
        *state = new_state;
        self.condvar.notify_all();
    }
}