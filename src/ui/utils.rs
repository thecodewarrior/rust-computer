use std::sync::{Mutex, Condvar};
use std::time::{Instant, Duration};
 
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

    pub fn wait_if_paused(&self) {
        let mut paused = self.mutex.lock().unwrap();
        while *paused {
            paused = self.condvar.wait(paused).unwrap();
        }
    }

    pub fn set_paused(&self, new_state: bool) {
        let mut state = self.mutex.lock().unwrap();
        *state = new_state;
        self.condvar.notify_all();
    }
}