use std::ops::{Index, IndexMut};
use std::time::{Duration, Instant};

use druid::widget::{Button, Flex, Label, Slider, WidgetExt};
use druid::{
    AppLauncher, BoxConstraints, Color, Data, Env, Event, EventCtx, LayoutCtx, Lens, LifeCycle,
    LifeCycleCtx, LocalizedString, MouseButton, PaintCtx, Point, Rect, RenderContext, Size,
    TimerToken, UpdateCtx, Widget, WindowDesc,
};
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::rc::Rc;
use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use super::state::*;
use super::worker::{SimulatorHandle, SimulatorState};

const BG: Color = Color::grey8(23_u8);

#[derive(Clone, Data, Lens)]
struct AppData {
    sim_state: UiSimState,
    #[druid(ignore)]
    sim_handle: Rc<SimulatorHandle>, // we don't want to clone the handle
}

pub fn ui_main() {
    let window = WindowDesc::new(make_main_ui)
        .window_size(Size {
            width: 800.0,
            height: 800.0,
        })
        .resizable(false)
        .title(
            LocalizedString::new("custom-widget-demo-window-title")
                .with_placeholder("Game of Life"),
        );
    let sim_state = UiSimState::new();
    let sim_handle = Rc::new(SimulatorHandle::new());
    setup_sim(&sim_handle);
    AppLauncher::with_window(window)
        .use_simple_logger()
        .launch(AppData {
            sim_state,
            sim_handle
        })
        .expect("launch failed");
}

pub fn setup_sim(sim_handle: &SimulatorHandle) {
    let mut sim_state = sim_handle.sim_state.write().unwrap();

    let args: Vec<String> = env::args().collect();
    let mut f = File::open(&args[1]).expect("No file");
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer);
    let limit = std::cmp::min(sim_state.computer.memory.data.len(), buffer.len());
    sim_state.computer.memory.data[..limit].copy_from_slice(&buffer[..limit]);
}

fn make_main_ui() -> impl Widget<AppData> {
    Flex::column()
    .with_child(SimStateReader {
        timer_id: TimerToken::INVALID,
    })
    .with_child(
        Label::new(|data: &AppData, _env: &_| format!("{:.2}FPS", data.sim_state.cpu.program_counter))
            .padding(3.0),
    )
    .with_flex_spacer(1.)
    .background(BG)
}

struct SimStateReader {
    timer_id: TimerToken,
}

impl Widget<AppData> for SimStateReader {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut AppData, _env: &Env) {
        match event {
            Event::WindowConnected => {
                data.sim_handle.thread_state.read().unwrap().paused.set_paused(false);
                let deadline = Instant::now() + Duration::from_millis(100 as u64);
                self.timer_id = ctx.request_timer(deadline);
            },
            Event::Timer(id) => {
                if *id == self.timer_id {
                    {
                        let sim_state = data.sim_handle.sim_state.read().unwrap();
                        data.sim_state.cpu.program_counter = sim_state.computer.cpu.program_counter.address;
                    }
                    let deadline =
                        Instant::now() + Duration::from_millis(100 as u64);
                    self.timer_id = ctx.request_timer(deadline);
                }
            },
            _ => (),
        }
    }

    fn lifecycle(&mut self, _ctx: &mut LifeCycleCtx, _event: &LifeCycle, _data: &AppData, _: &Env) {
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &AppData, data: &AppData, _: &Env) {
        if !old_data.same(data) {
            // send settings/etc. that other widgets change to the sim thread?
        }
    }

    fn layout(&mut self, _: &mut LayoutCtx, bc: &BoxConstraints, _: &AppData, _: &Env) -> Size {
        bc.min()
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &AppData, _env: &Env) {
        // nop
    }
}