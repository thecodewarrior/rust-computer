use std::time::{Duration, Instant};

use super::state::*;
use super::{worker::{SimulatorHandle}};
use druid::lens::{self, LensExt};
use druid::widget::{
    CrossAxisAlignment, Flex, Label, List, MainAxisAlignment, Scroll,
    WidgetExt, Controller, Container,
};
use druid::{
    AppLauncher, Color, Data, Env, Event, EventCtx, Lens, LocalizedString, RenderContext, Size,
    TimerToken, UnitPoint, Widget, WindowDesc, Key, KeyCode,
};
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::rc::Rc;
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

const BG: Color = Color::grey8(23_u8);

#[derive(Clone, Data, Lens)]
struct AppData {
    sim_state: UiSimState,
    #[data(ignore)]
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
    thread_state.frequency = 1.;
}

const MONO_FONT: Key<&str> = Key::new("rust-computer.mono_font");

fn make_main_ui() -> impl Widget<AppData> {
    let controller = SimStateReader {
        timer_id: TimerToken::INVALID,
        ui_ups: 10.,
    };
    Flex::row()
        .with_child(
            Flex::column()
                .main_axis_alignment(MainAxisAlignment::Start)
                .cross_axis_alignment(CrossAxisAlignment::Start)
                .must_fill_main_axis(true)
                .with_child(
                    Label::new(|data: &AppData, _env: &_| {
                        format!("PC: 0x{:08x}", data.sim_state.cpu.program_counter)
                    })
                    .with_font(MONO_FONT)
                    .padding(3.0),
                )
                .with_child(
                    Label::new(|data: &AppData, _env: &_| {
                        format!("{:.2} Hz", data.sim_state.actual_frequency)
                    })
                    .padding(3.0),
                )
                .with_child(
                    Label::new("Registers")
                        .align_vertical(UnitPoint::LEFT)
                        .padding(3.0),
                )
                .with_flex_child(
                    Scroll::new(List::new(|| {
                        Label::new(|item: &(usize, u32), _env: &_| format!("R{:<2} 0x{:08x}", item.0, item.1))
                            .with_font(MONO_FONT)
                            .align_vertical(UnitPoint::LEFT)
                            .padding(3.0)
                    }))
                    .vertical()
                    .lens(
                        AppData::sim_state
                            .then(UiSimState::cpu)
                            .then(UiCpuState::registers),
                    ),
                    1.0,
                )
                .with_child(
                    Label::new("Vars")
                        .align_vertical(UnitPoint::LEFT)
                        .padding(3.0),
                )
                .with_flex_child(
                    Scroll::new(List::new(|| {
                        Label::new(|item: &u32, _env: &_| format!("0x{:08x}", item))
                            .with_font(MONO_FONT)
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
                ),
        )
        .with_flex_spacer(1.)
        .background(BG)
        .controller(controller)
        .env_scope(|env: &mut druid::Env, data: &AppData| {
            env.set(MONO_FONT, "monospace");
        })
}

struct SimStateReader {
    timer_id: TimerToken,
    ui_ups: f64,
}

impl Controller<AppData, Container<AppData>> for SimStateReader {
    fn event(&mut self, child: &mut Container<AppData>, ctx: &mut EventCtx, event: &Event, data: &mut AppData, env: &Env) {
        match event {
            Event::WindowConnected => {
                let deadline = Instant::now() + Duration::from_secs_f64(1. / self.ui_ups);
                self.timer_id = ctx.request_timer(deadline);
                ctx.request_focus();
            }
            Event::Timer(id) => {
                if *id == self.timer_id {
                    {
                        let mut thread_state = data.sim_handle.thread_state.write().unwrap();
                        thread_state.ui_frequency = self.ui_ups;
                        data.sim_state.actual_frequency = thread_state.actual_frequency;
                    }

                    {
                        let sim_state = data.sim_handle.sim_state.read().unwrap();
                        data.sim_state.cpu.program_counter =
                            sim_state.computer.cpu.program_counter.address;
                        {
                            let vars: &mut Vec<(usize, u32)> = Arc::make_mut(&mut data.sim_state.cpu.registers);
                            vars.clear();
                            if let Some(frame) = sim_state.computer.cpu.frames.last() {
                                let registers = &frame.registers;
                                for i in 0 .. registers.len() {
                                    vars.push((i, registers[i]));
                                }
                            }
                        }
                        {
                            let vars: &mut Vec<u32> = Arc::make_mut(&mut data.sim_state.cpu.vars);
                            vars.clear();
                            if let Some(frame) = sim_state.computer.cpu.frames.last() {
                                vars.clone_from(&frame.vars);
                            }
                        }
                    }
                    let deadline = Instant::now() + Duration::from_secs_f64(1. / self.ui_ups);
                    self.timer_id = ctx.request_timer(deadline);
                }
            },
            Event::KeyDown(e) => {
                if e.key_code == KeyCode::Space && !e.is_repeat {
                    let did_pause: bool;
                    {
                        let thread_state = data.sim_handle.thread_state.read().unwrap(); 
                        did_pause = !thread_state.paused.is_paused();
                        thread_state.paused.set_paused(did_pause);
                    }
                    if did_pause {
                        let sim_state = data.sim_handle.sim_state.read().unwrap(); 
                        let mut f = File::create("debug/memory.bin").unwrap();
                        f.write_all(&sim_state.computer.memory.data[..]);
                    }
                }
                if e.key_code == KeyCode::Period {
                    if data.sim_handle.thread_state.read().unwrap().paused.is_paused() {
                        let mut sim_state = data.sim_handle.sim_state.write().unwrap(); 
                        sim_state.computer.tick(); 
                        let mut f = File::create("debug/memory.bin").unwrap();
                        f.write_all(&sim_state.computer.memory.data[..]);
                    }
                }
            },
            _ => (),
        }
        child.event(ctx, event, data, env)
    }
}
