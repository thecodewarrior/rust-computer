use druid::{Data, Lens};
use std::sync::Arc;

#[derive(Clone, Data, Lens)]
pub struct UiSimState {
    pub cpu: UiCpuState,
}

#[derive(Clone, Data, Lens)]
pub struct UiCpuState {
    pub program_counter: u32,
    pub stack: Arc<Vec<u32>>,
}

impl UiSimState {
    pub fn new() -> UiSimState {
        UiSimState {
            cpu: UiCpuState {
                program_counter: 0,
                stack: Arc::new(vec![]),
            },
        }
    }
}
