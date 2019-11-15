#[macro_use] extern crate conrod_core;
extern crate conrod_glium;
extern crate glium;

use conrod_core::{widget, Positionable, Colorable, Widget};
use glium::Surface;

const WIDTH: u32 = 400;
const HEIGHT: u32 = 200;

widget_ids! {
    struct Ids {
        circle,
    }
}

fn main() {
    let mut event_loop = glium::glutin::EventsLoop::new();
    let window = glium::glutin::WindowBuilder::new()
        .with_title("Circle Widget Demo")
        .with_dimensions((WIDTH, HEIGHT).into());
    let context = glium::glutin::ContextBuilder::new()
        .with_vsync(true)
        .with_multisampling(4);
    let display = glium::Display::new(window, context, &event_loop).unwrap();
    let mut ui = conrod_core::UiBuilder::new([WIDTH as f64, HEIGHT as f64]).build();

    // A type used for converting `conrod_core::render::Primitives` into `Command`s that can be used
    // for drawing to the glium `Surface`.
    let mut renderer = conrod_glium::Renderer::new(&display).unwrap();
    // The image map describing each of our widget->image mappings (in our case, none).
    let image_map = conrod_core::image::Map::<glium::texture::Texture2d>::new();
    let ids = Ids::new(ui.widget_id_generator());

    let mut events = Vec::new();
    'render: loop {
        events.clear();
        event_loop.poll_events(|event| { events.push(event); });

        if events.is_empty() {
            event_loop.run_forever(|event| {
                events.push(event);
                glium::glutin::ControlFlow::Break
            })
        }

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
        }

        set_ui(ui.set_widgets(), &ids);

        if let Some(primitives) = ui.draw_if_changed() {
            renderer.fill(&display, primitives, &image_map);
            let mut target = display.draw();
            target.clear_color(0., 0., 0., 1.);
            renderer.draw(&display, &mut target, &image_map).unwrap();
            target.finish().unwrap();
        }
    }
}

fn set_ui(ref mut ui: conrod_core::UiCell, ids: &Ids) {
    widget::Circle::fill(50.)
        .middle_of(ui.window)
        .color(conrod_core::color::RED)
        .set(ids.circle, ui);
}