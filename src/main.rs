extern crate conrod_core;
extern crate conrod_glium;
extern crate glium;

const WIDTH: u32 = 400;
const HEIGHT: u32 = 200;

fn main() {
    let mut event_loop = glium::glutin::EventsLoop::new();
    let window = glium::glutin::WindowBuilder::new()
        .with_title("Hello Conrod!")
        .with_dimensions((WIDTH, HEIGHT).into());
    let context = glium::glutin::ContextBuilder::new()
        .with_vsync(true)
        .with_multisampling(4);
    let display = glium::Display::new(window, context, &event_loop).unwrap();
//    let mut ui = conrod_core::UiBuilder::new([WIDTH as f64, HEIGHT as f64]).build();

    let mut events = Vec::new();
    'render: loop {
        events.clear();
        event_loop.poll_events(|event| { events.push(event);});

        if events.is_empty() {
            event_loop.run_forever(|event| {
                events.push(event);
                glium::glutin::ControlFlow::Break
            })
        }

        for event in events.drain(..) {
            // Break from the loop upon `Escape` or closed window.
            match event.clone() {
                glium::glutin::Event::WindowEvent { event, ..} => {
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
    }
}
