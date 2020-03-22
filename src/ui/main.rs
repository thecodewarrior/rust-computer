use std::ops::{Index, IndexMut};
use std::time::{Duration, Instant};

use super::state::*;
use super::worker::{SimulatorHandle, SimulatorState};
use druid::lens::{self, LensExt};
use druid::widget::{
    Button, CrossAxisAlignment, Flex, FlexParams, Label, List, MainAxisAlignment, Scroll, Slider,
    WidgetExt,
};
use druid::{
    AppLauncher, BoxConstraints, Color, Data, Env, Event, EventCtx, LayoutCtx, Lens, LifeCycle,
    LifeCycleCtx, LocalizedString, MouseButton, PaintCtx, Point, Rect, RenderContext, Size,
    TimerToken, UnitPoint, UpdateCtx, Widget, WindowDesc,
};
use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::rc::Rc;
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

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
            sim_handle,
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

    let mut thread_state = sim_handle.thread_state.write().unwrap();
    thread_state.paused.set_paused(false);
    thread_state.frequency = 8_000_000.;
}

fn make_main_ui() -> impl Widget<AppData> {
    Flex::row()
        .with_child(SimStateReader {
            timer_id: TimerToken::INVALID,
            ui_ups: 10.,
        })
        .with_child(
            Flex::column()
                .main_axis_alignment(MainAxisAlignment::Start)
                .cross_axis_alignment(CrossAxisAlignment::Start)
                .must_fill_main_axis(true)
                .with_child(
                    Label::new(|data: &AppData, _env: &_| {
                        format!("PC: 0x{:08x}", data.sim_state.cpu.program_counter)
                    })
                    .padding(3.0),
                )
                .with_child(
                    Label::new(|data: &AppData, _env: &_| {
                        format!("{} Hz", data.sim_state.actual_frequency)
                    })
                    .padding(3.0),
                )
                .with_child(
                    Label::new("Vars")
                        .align_vertical(UnitPoint::LEFT)
                        .padding(3.0),
                )
                .with_flex_child(
                    Scroll::new(List::new(|| {
                        Label::new(|item: &u32, _env: &_| format!("0x{:08x}", item))
                            .align_vertical(UnitPoint::LEFT)
                            .padding(3.0)
                    }))
                    .vertical()
                    .lens(
                        AppData::sim_state
                            .then(UiSimState::cpu)
                            .then(UiCpuState::vars),
                    ),
                    1.0,
                )
                .with_child(
                    Label::new("Stack")
                        .align_vertical(UnitPoint::LEFT)
                        .padding(3.0),
                )
                .with_flex_child(
                    Scroll::new(List::new(|| {
                        Label::new(|item: &u32, _env: &_| format!("0x{:08x}", item))
                            .align_vertical(UnitPoint::LEFT)
                            .padding(3.0)
                    }))
                    .vertical()
                    .lens(
                        AppData::sim_state
                            .then(UiSimState::cpu)
                            .then(UiCpuState::stack),
                    ),
                    1.0,
                )
        )
        .with_flex_spacer(1.)
        .background(BG)
}

struct SimStateReader {
    timer_id: TimerToken,
    ui_ups: f64,
}

impl Widget<AppData> for SimStateReader {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut AppData, _env: &Env) {
        match event {
            Event::WindowConnected => {
                let deadline = Instant::now() + Duration::from_secs_f64(1. / self.ui_ups);
                self.timer_id = ctx.request_timer(deadline);
            }
            Event::Timer(id) => {
                if *id == self.timer_id {
                    {
                        let mut thread_state = data.sim_handle
                            .thread_state
                            .write()
                            .unwrap();
                        thread_state.ui_frequency = self.ui_ups;
                        data.sim_state.actual_frequency = thread_state.actual_frequency;
                    }

                    {
                        let sim_state = data.sim_handle.sim_state.read().unwrap();
                        data.sim_state.cpu.program_counter =
                            sim_state.computer.cpu.program_counter.address;
                        {
                            let stack: &mut Vec<u32> = Arc::make_mut(&mut data.sim_state.cpu.stack);
                            stack.clear();
                            if let Some(ref frame) = sim_state.computer.cpu.frames.last() {
                                stack.clone_from(&frame.stack);
                            }
                        }
                        {
                            let vars: &mut Vec<u32> = Arc::make_mut(&mut data.sim_state.cpu.vars);
                            vars.clear();
                            if let Some(ref frame) = sim_state.computer.cpu.frames.last() {
                                vars.clone_from(&frame.vars);
                            }
                        }
                    }
                    let deadline = Instant::now() + Duration::from_secs_f64(1. / self.ui_ups);
                    self.timer_id = ctx.request_timer(deadline);
                }
            }
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
