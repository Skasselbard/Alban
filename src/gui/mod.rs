use conrod::{self, widget, Colorable, Positionable, Ui, Widget};
use conrod::image::Map;
use conrod::backend::glium::Renderer;
use conrod::backend::glium::glium::{self, Surface};

macro_rules! WIDTH {() => {800};}
macro_rules! HEIGHT {() => {600};}

#[derive(PartialEq, Eq)]
enum WindowEvents {
    CloseWindow,
    DoNothing,
}

widget_ids!(struct Ids { 
    master,
    top,
    left,
    bottom,
    right,
    center,
    body,
    top_text,
    left_text,
    bottom_text,
    right_text,
    center_text,
    });

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

// Draw the Ui.
fn set_widgets(ref mut ui: conrod::UiCell, ids: &Ids) {
    use conrod::color;

    // Construct our main `Canvas` tree.
    widget::Canvas::new()
        .flow_down(&[
            (
                ids.top,
                widget::Canvas::new().color(color::BLUE).pad_bottom(20.0),
            ),
            (
                ids.body,
                widget::Canvas::new().length(300.0).flow_right(&[
                    (
                        ids.left,
                        widget::Canvas::new().color(color::LIGHT_ORANGE).pad(20.0),
                    ),
                    (ids.center, widget::Canvas::new().color(color::ORANGE)),
                    (
                        ids.right,
                        widget::Canvas::new().color(color::DARK_ORANGE).pad(20.0),
                    ),
                ]),
            ),
            (
                ids.bottom,
                widget::Canvas::new()
                    .color(color::BLUE)
                    .scroll_kids_vertically(),
            ),
        ])
        .set(ids.master, ui);

    widget::Text::new("Top")
        .color(color::BLACK)
        .font_size(48)
        .middle_of(ids.top)
        .set(ids.top_text, ui);
    widget::Text::new("Left")
        .color(color::BLACK)
        .mid_bottom_of(ids.left)
        .set(ids.left_text, ui);
    widget::Text::new("Bottom")
        .color(color::BLACK)
        .top_left_of(ids.bottom)
        .set(ids.bottom_text, ui);
    widget::Text::new("Right")
        .color(color::BLACK)
        .bottom_right_of(ids.right)
        .set(ids.right_text, ui);
    widget::Text::new("Center")
        .color(color::BLACK)
        .bottom_right_of(ids.center)
        .set(ids.center_text, ui);
}
