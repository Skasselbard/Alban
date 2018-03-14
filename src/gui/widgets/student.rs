use conrod::{self, widget, Widget};

#[derive(WidgetCommon)]
pub struct Student {
    #[conrod(common_builder)] common: widget::CommonBuilder,
    id: String,
    name: String,
    style: Style,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, WidgetStyle)]
pub struct Style {
    #[conrod(default = "theme.shape_color")] pub color: Option<conrod::Color>,
    #[conrod(default = "theme.label_color")] pub label_color: Option<conrod::Color>,
    #[conrod(default = "theme.font_size_medium")] pub label_font_size: Option<conrod::FontSize>,
    #[conrod(default = "theme.font_id")] pub label_font_id: Option<Option<conrod::text::font::Id>>,
}
widget_ids!{
    struct Ids{
        text_id,
        text_name
    }
}

impl Student {
    pub fn new() -> Self {
        Student {
            id: String::new(),
            name: String::new(),
            common: widget::CommonBuilder::default(),
            style: Style::default(),
        }
    }
}

pub struct State {
    ids: Ids,
}

impl Widget for Student {
    type State = State;
    type Style = Style;
    type Event = Option<()>;
    fn init_state(&self, id_gen: widget::id::Generator) -> Self::State {
        State {
            ids: Ids::new(id_gen),
        }
    }
    fn style(&self) -> Self::Style {
        self.style.clone()
    }
    fn update(self, args: widget::UpdateArgs<Self>) -> Self::Event {
        let widget::UpdateArgs {
            id,
            ..,
            ..,
            ui,
            style,
            ..
        } = args;

        let input = ui.widget_input(id);

        // If the button was clicked, produce `Some` event.
        //let event = input.clicks().left().next().map(|_| ());

        let color = style.color(&ui.theme);
        let color = input.mouse().map_or(color, |mouse| {
            if mouse.buttons.left().is_down() {
                color.clicked()
            } else {
                color.highlighted()
            }
        });
        //TODO: Insert widgets somehow
        None
    }
}
