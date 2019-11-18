#[macro_use] extern crate conrod_core;
extern crate conrod_glium;
#[allow(unused_imports)]
#[macro_use] extern crate conrod_winit;
extern crate glium;
extern crate find_folder;

use glium::Surface;

mod event;
mod constants;
mod animation;

fn main() {
    let mut events_loop = glium::glutin::EventsLoop::new();
    let window = glium::glutin::WindowBuilder::new()
        .with_title("Circle Widget Demo")
        .with_dimensions((constants::WIDTH, constants::HEIGHT).into());
    let context = glium::glutin::ContextBuilder::new()
        .with_vsync(true)
        .with_multisampling(4);
    let display = glium::Display::new(window, context, &events_loop).unwrap();
    let display = event::GliumDisplayWinitWrapper(display);

    let mut ui = conrod_core::UiBuilder::new([constants::WIDTH as f64, constants::HEIGHT as f64]).build();

    let assets = find_folder::Search::KidsThenParents(3, 5).for_folder("assets").unwrap();
    let font_path = assets.join("fonts/ipam.ttf");
    ui.fonts.insert_from_file(font_path).unwrap();

    // A type used for converting `conrod_core::render::Primitives` into `Command`s that can be used
    // for drawing to the glium `Surface`.
    let mut renderer = conrod_glium::Renderer::new(&display.0).unwrap();
    // The image map describing each of our widget->image mappings (in our case, none).
    let image_map = conrod_core::image::Map::<glium::texture::Texture2d>::new();

    let mut animation_loop = event::AnimationLoop::new();
    let mut anim = animation::Animation::new(&mut ui);

    'render: loop {
        for event in animation_loop.next(&mut events_loop, 1000 / constants::FPS as u64) {

            // Use the `winit` backend feature to convert the winit event to a conrod one.
            if let Some(event) = event::convert_event(event.clone(), &display) {
                ui.handle_event(event);
            }

            // Break from the loop upon `Escape` or closed window.
            match event {
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

        anim.next_frame(ui.set_widgets());

        if let Some(primitives) = ui.draw_if_changed() {
            renderer.fill(&display.0, primitives, &image_map);
            let mut target = display.0.draw();
            target.clear_color(0., 0., 0., 1.);
            renderer.draw(&display.0, &mut target, &image_map).unwrap();
            target.finish().unwrap();
        }
    }
}
