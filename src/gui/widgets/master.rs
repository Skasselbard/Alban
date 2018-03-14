use conrod::{self, widget, Colorable, Positionable, Widget};

widget_ids!(pub struct Ids { 
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

pub fn set_widgets(ref mut ui: conrod::UiCell, ids: &Ids) {
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
