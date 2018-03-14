mod widgets;

use conrod::{self, Ui};
use conrod::image::Map;
use conrod::backend::glium::Renderer;
use conrod::backend::glium::glium::{self, Surface};
use gui::widgets::master::*;

macro_rules! WIDTH {() => {800};}
macro_rules! HEIGHT {() => {600};}

#[derive(PartialEq, Eq)]
enum WindowEvents {
    CloseWindow,
    DoNothing,
}

pub fn main() {
    let (mut events_loop, display, mut ui, mut renderer) = init_ui();
    // The image map describing each of our widget->image mappings (in our case, none).
    let image_map = conrod::image::Map::<glium::texture::Texture2d>::new();
    let mut events = Vec::new();
    // Generate the widget identifiers.

    let ids = Ids::new(ui.widget_id_generator());

    'render: loop {
        events.clear();
        wait_for_events(&mut events_loop, &mut events);

        let return_event = process_events(&mut events);
        if return_event == WindowEvents::CloseWindow {
            break;
        }
        //draw ui
        set_widgets(ui.set_widgets(), &ids);
        redraw_ui(&mut ui, &mut renderer, &display, &image_map);
    }
}

fn init_ui() -> (glium::glutin::EventsLoop, glium::Display, Ui, Renderer) {
    let events_loop = glium::glutin::EventsLoop::new();
    let window = glium::glutin::WindowBuilder::new()
        .with_title("Hello Conrod!")
        .with_dimensions(WIDTH!(), HEIGHT!());
    let context = glium::glutin::ContextBuilder::new().with_vsync(true);
    //.with_multisampling(4);
    let display = glium::Display::new(window, context, &events_loop).unwrap();
    let mut ui = conrod::UiBuilder::new([WIDTH!() as f64, HEIGHT!() as f64]).build();
    ui.fonts
        .insert_from_file("./assets/fonts/NotoSans/NotoSans-Regular.ttf")
        .unwrap();
    // A type used for converting `conrod::render::Primitives` into `Command`s that can be used for drawing to the glium `Surface`.
    let renderer = Renderer::new(&display).unwrap();
    (events_loop, display, ui, renderer)
}

fn redraw_ui(
    ui: &mut Ui,
    renderer: &mut Renderer,
    display: &glium::Display,
    image_map: &Map<glium::texture::Texture2d>,
) {
    if let Some(primitives) = ui.draw_if_changed() {
        renderer.fill(display, primitives, image_map);
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);
        renderer.draw(display, &mut target, image_map).unwrap();
        target.finish().unwrap();
    }
}

fn wait_for_events(
    events_loop: &mut glium::glutin::EventsLoop,
    events: &mut Vec<glium::glutin::Event>,
) {
    // Get all the new events since the last frame.
    events_loop.poll_events(|event| {
        events.push(event);
    });
    // If there are no new events, wait for one.
    if events.is_empty() {
        events_loop.run_forever(|event| {
            events.push(event);
            glium::glutin::ControlFlow::Break
        });
    }
}

fn process_events(events: &mut Vec<glium::glutin::Event>) -> WindowEvents {
    for event in events.drain(..) {
        // Break from the loop upon `Escape` or closed window.
        match event.clone() {
            glium::glutin::Event::WindowEvent { event, .. } => match event {
                glium::glutin::WindowEvent::Closed
                | glium::glutin::WindowEvent::KeyboardInput {
                    input:
                        glium::glutin::KeyboardInput {
                            virtual_keycode: Some(glium::glutin::VirtualKeyCode::Escape),
                            ..
                        },
                    ..
                } => return WindowEvents::CloseWindow,
                _ => (),
            },
            _ => (),
        };
    }
    WindowEvents::DoNothing
}
