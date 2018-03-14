use conrod::{self, widget, Colorable, Labelable, Point, Positionable, Widget};
use gui::widgets::student;

/// The type upon which we'll implement the `Widget` trait.
#[derive(WidgetCommon)]
pub struct CircularButton<'a> {
    /// An object that handles some of the dirty work of rendering a GUI. We don't
    /// really have to worry about it.
    #[conrod(common_builder)]
    common: widget::CommonBuilder,
    /// Optional label string for the button.
    maybe_label: Option<&'a str>,
    /// See the Style struct below.
    style: Style,
    /// Whether the button is currently enabled, i.e. whether it responds to
    /// user input.
    enabled: bool,
}

// We use `#[derive(WidgetStyle)] to vastly simplify the definition and implementation of the
// widget's associated `Style` type. This generates an implementation that automatically
// retrieves defaults from the provided theme in the following order:
//
// 1. If the field is `None`, falls back to the style stored within the `Theme`.
// 2. If there are no style defaults for the widget in the `Theme`, or if the
//    default field is also `None`, falls back to the expression specified within
//    the field's `#[conrod(default = "expr")]` attribute.

/// Represents the unique styling for our CircularButton widget.
#[derive(Copy, Clone, Debug, Default, PartialEq, WidgetStyle)]
pub struct Style {
    /// Color of the button.
    #[conrod(default = "theme.shape_color")]
    pub color: Option<conrod::Color>,
    /// Color of the button's label.
    #[conrod(default = "theme.label_color")]
    pub label_color: Option<conrod::Color>,
    /// Font size of the button's label.
    #[conrod(default = "theme.font_size_medium")]
    pub label_font_size: Option<conrod::FontSize>,
    /// Specify a unique font for the label.
    #[conrod(default = "theme.font_id")]
    pub label_font_id: Option<Option<conrod::text::font::Id>>,
}

// We'll create the widget using a `Circle` widget and a `Text` widget for its label.
//
// Here is where we generate the type that will produce these identifiers.
widget_ids! {
    struct Ids {
        circle,
        text,
    }
}

/// Represents the unique, cached state for our CircularButton widget.
pub struct State {
    ids: Ids,
}

impl<'a> CircularButton<'a> {
    /// Create a button context to be built upon.
    pub fn new() -> Self {
        CircularButton {
            common: widget::CommonBuilder::default(),
            style: Style::default(),
            maybe_label: None,
            enabled: true,
        }
    }

    /// Specify the font used for displaying the label.
    pub fn label_font_id(mut self, font_id: conrod::text::font::Id) -> Self {
        self.style.label_font_id = Some(Some(font_id));
        self
    }

    /// If true, will allow user inputs.  If false, will disallow user inputs.  Like
    /// other Conrod configs, this returns self for chainability. Allow dead code
    /// because we never call this in the example.
    #[allow(dead_code)]
    pub fn enabled(mut self, flag: bool) -> Self {
        self.enabled = flag;
        self
    }
}

/// A custom Conrod widget must implement the Widget trait. See the **Widget** trait
/// documentation for more details.
impl<'a> Widget for CircularButton<'a> {
    /// The State struct that we defined above.
    type State = State;
    /// The Style struct that we defined using the `widget_style!` macro.
    type Style = Style;
    /// The event produced by instantiating the widget.
    ///
    /// `Some` when clicked, otherwise `None`.
    type Event = Option<()>;

    fn init_state(&self, id_gen: widget::id::Generator) -> Self::State {
        State {
            ids: Ids::new(id_gen),
        }
    }

    fn style(&self) -> Self::Style {
        self.style.clone()
    }

    /// Optionally specify a function to use for determining whether or not a point is over a
    /// widget, or if some other widget's function should be used to represent this widget.
    ///
    /// This method is optional to implement. By default, the bounding rectangle of the widget
    /// is used.
    fn is_over(&self) -> widget::IsOverFn {
        use conrod::graph::Container;
        use conrod::Theme;
        fn is_over_widget(widget: &Container, _: Point, _: &Theme) -> widget::IsOver {
            let unique = widget.state_and_style::<State, Style>().unwrap();
            unique.state.ids.circle.into()
        }
        is_over_widget
    }

    /// Update the state of the button by handling any input that has occurred since the last
    /// update.
    fn update(self, args: widget::UpdateArgs<Self>) -> Self::Event {
        let widget::UpdateArgs {
            id,
            state,
            rect,
            ui,
            style,
            ..
        } = args;

        let (color, event) = {
            let input = ui.widget_input(id);

            // If the button was clicked, produce `Some` event.
            let event = input.clicks().left().next().map(|_| ());

            let color = style.color(&ui.theme);
            let color = input.mouse().map_or(color, |mouse| {
                if mouse.buttons.left().is_down() {
                    color.clicked()
                } else {
                    color.highlighted()
                }
            });

            (color, event)
        };

        // Finally, we'll describe how we want our widget drawn by simply instantiating the
        // necessary primitive graphics widgets.
        //
        // Conrod will automatically determine whether or not any changes have occurred and
        // whether or not any widgets need to be re-drawn.
        //
        // The primitive graphics widgets are special in that their unique state is used within
        // conrod's backend to do the actual drawing. This allows us to build up more complex
        // widgets by using these simple primitives with our familiar layout, coloring, etc
        // methods.
        //
        // If you notice that conrod is missing some sort of primitive graphics that you
        // require, please file an issue or open a PR so we can add it! :)

        // First, we'll draw the **Circle** with a radius that is half our given width.
        let radius = rect.w() / 2.0;
        widget::Circle::fill(radius)
            .middle_of(id)
            .graphics_for(id)
            .color(color)
            .set(state.ids.circle, ui);

        // Now we'll instantiate our label using the **Text** widget.
        if let Some(ref label) = self.maybe_label {
            let label_color = style.label_color(&ui.theme);
            let font_size = style.label_font_size(&ui.theme);
            let font_id = style.label_font_id(&ui.theme).or(ui.fonts.ids().next());
            widget::Text::new(label)
                .and_then(font_id, widget::Text::font_id)
                .middle_of(id)
                .font_size(font_size)
                .graphics_for(id)
                .color(label_color)
                .set(state.ids.text, ui);
        }

        event
    }
}

/// Provide the chainable color() configuration method.
impl<'a> Colorable for CircularButton<'a> {
    fn color(mut self, color: conrod::Color) -> Self {
        self.style.color = Some(color);
        self
    }
}

/// Provide the chainable label(), label_color(), and label_font_size()
/// configuration methods.
impl<'a> Labelable<'a> for CircularButton<'a> {
    fn label(mut self, text: &'a str) -> Self {
        self.maybe_label = Some(text);
        self
    }
    fn label_color(mut self, color: conrod::Color) -> Self {
        self.style.label_color = Some(color);
        self
    }
    fn label_font_size(mut self, size: conrod::FontSize) -> Self {
        self.style.label_font_size = Some(size);
        self
    }
}
