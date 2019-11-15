extern crate conrod_glium;
extern crate glium;

fn main() {
    println!("Hello, world!");
    let mut event_loop = glium::glutin::EventsLoop::new();
    let window = glium::glutin::WindowBuilder::new()
        .with_title("Hello Conrod!")
        .with_dimensions((400, 200).into());
    let context = glium::glutin::ContextBuilder::new()
        .with_vsync(true)
        .with_multisampling(4);
    let display = glium::Display::new(window, context, &event_loop).unwrap();

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
    }
}
