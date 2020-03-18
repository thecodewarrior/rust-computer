use conrod_core::{Ui, UiCell};
use glium::Surface;
use super::interop;
use std::time::{Instant, Duration};

pub trait ConrodApp {
    fn frame(&mut self, ui: &mut UiCell);
} 

pub fn conrod_launch<A: ConrodApp, F>(title: &str, width: u32, height: u32, setup: F) where F: Fn(&mut Ui, std::path::PathBuf) -> A {
    // Build the window.
    let mut events_loop = glium::glutin::EventsLoop::new();
    let window = glium::glutin::WindowBuilder::new()
        .with_title(title) // !
        .with_dimensions((width, height).into()); // !
    let context = glium::glutin::ContextBuilder::new()
        .with_vsync(true)
        .with_multisampling(4);
    let display = glium::Display::new(window, context, &events_loop).unwrap();
    let display = interop::GliumDisplayWinitWrapper(display);

    // construct our `UI`.
    let mut ui = conrod_core::UiBuilder::new([width as f64, height as f64]).build(); // !

    // A type used for converting `conrod_core::render::Primitives` into `Command`s that can be used
    // for drawing to the glium `Surface`.
    let mut renderer = conrod_glium::Renderer::new(&display.0).unwrap();

    // The image map describing each of our widget->image mappings (in our case, none).
    let image_map = conrod_core::image::Map::<glium::texture::Texture2d>::new();

    let mut events = Vec::new();


    // Add a `Font` to the `UI`'s `font::Map` from file.
    let assets = find_folder::Search::KidsThenParents(3, 5).for_folder("assets").unwrap();

    let mut app = setup(&mut ui, assets);

    let mut last_frame = Instant::now();
    let frame_time = Duration::from_secs_f32(1./30.);
    'render: loop {
        events.clear();

        // Get all the new events since the last frame.
        events_loop.poll_events(|event| { events.push(event); });

        // Process the events.
        for event in events.drain(..) {

            // Break from the loop upon `Escape` or closed window.
            match event.clone() {
                glium::glutin::Event::WindowEvent { event, .. } => {
                    match event {
                        glium::glutin::WindowEvent::CloseRequested |
                        glium::glutin::WindowEvent::KeyboardInput {
                            input: glium::glutin::KeyboardInput {
                                virtual_keycode: Some(glium::glutin::VirtualKeyCode::Escape),
                                ..
                            },
                            ..
                        } => break 'render,
                        _ => (),
                    }
                }
                _ => (),
            };

            // Use the `winit` backend feature to convert the winit event to a conrod input.
            let input = match interop::convert_event(event, &display) {
                None => continue,
                Some(input) => input,
            };

            // Handle the input with the `UI`.
            ui.handle_event(input);
        }

        // Set the widgets.
        app.frame(&mut ui.set_widgets());

        // Draw the `UI` if it has changed.
        if let Some(primitives) = ui.draw_if_changed() {
            renderer.fill(&display.0, primitives, &image_map);
            let mut target = display.0.draw();
            target.clear_color(0.0, 0.0, 0.0, 1.0);
            renderer.draw(&display.0, &mut target, &image_map).unwrap();
            target.finish().unwrap();
        }

        let now_time = Instant::now();
        let delta = now_time.duration_since(last_frame);
        last_frame = now_time;
        if delta < frame_time {
            std::thread::sleep(frame_time - delta);
        }
    }
}