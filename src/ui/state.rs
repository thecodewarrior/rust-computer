use druid::{Data, Lens};

#[derive(Clone, Data, Lens)]
pub struct UiSimState {
    pub cpu: UiCpuState,
}

#[derive(Clone, Data)]
pub struct UiCpuState {
    pub program_counter: u32,
}

impl UiSimState {
    pub fn new() -> UiSimState {
        UiSimState {
            cpu: UiCpuState {
                program_counter: 0,
            }
        }
    }
}