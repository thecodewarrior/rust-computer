use druid::{Data, Lens};
use std::sync::Arc;

#[derive(Clone, Data, Lens)]
pub struct UiSimState {
    pub cpu: UiCpuState,
    pub actual_frequency: f64,
}

#[derive(Clone, Data, Lens)]
pub struct UiCpuState {
    pub program_counter: u32,
    pub registers: Arc<Vec<(usize, u32)>>,
    pub vars: Arc<Vec<u32>>,
}

impl UiSimState {
    pub fn new() -> UiSimState {
        UiSimState {
            cpu: UiCpuState {
                program_counter: 0,
                registers: Arc::new(vec![]),
                vars: Arc::new(vec![]),
            },
            actual_frequency: 0.,
        }
    }
}
