mod circle {

    use iced::advanced::layout::{self, Layout};
    use iced::advanced::renderer;
    use iced::advanced::widget::{self, Widget};

    use iced::{mouse, Border};
    use iced::{Color, Element, Length, Rectangle, Size};

    pub struct Circle {
        radius: f32,
    }

    impl Circle {
        pub fn new(radius: f32) -> Self {
            Self { radius }
        }
    }

    pub fn circle(radius: f32) -> Circle {
        Circle::new(radius)
    }

    impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer> for Circle
    where
        Renderer: renderer::Renderer,
    {
        fn size(&self) -> Size<Length> {
            Size {
                width: Length::Shrink,
                height: Length::Shrink,
            }
        }

        fn layout(
            &self,
            _tree: &mut widget::Tree,
            _renderer: &Renderer,
            _limits: &layout::Limits,
        ) -> layout::Node {
            layout::Node::new(Size::new(self.radius * 2.0, self.radius * 2.0))
        }

        fn draw(
            &self,
            _state: &widget::Tree,
            renderer: &mut Renderer,
            _theme: &Theme,
            _style: &renderer::Style,
            layout: Layout<'_>,
            _cursor: mouse::Cursor,
            _viewport: &Rectangle,
        ) {
            renderer.fill_quad(
                renderer::Quad {
                    bounds: layout.bounds(),
                    border: Border::with_radius(self.radius),
                    ..renderer::Quad::default()
                },
                Color::BLACK,
            );
        }
    }

    impl<'a, Message, Theme, Renderer> From<Circle>
        for Element<'a, Message, Theme, Renderer>
    where
        Renderer: renderer::Renderer,
    {
        fn from(circle: Circle) -> Self {
            Self::new(circle)
        }
    }
}


use circle::circle;
use iced::widget::{column, slider, text};
use iced::{Element, Sandbox};

pub fn main() -> iced::Result {
    Example::run(Default::default())
}

struct Example {
    radius: f32,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    RadiusChanged(f32),
}
impl Sandbox for Example {
    type Message= Message;

    fn new() -> Self {
        Example { radius: 50.0 }
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::RadiusChanged(radius) => {
                self.radius = radius;
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let content = column![
            circle(self.radius),
            text(self.radius),
            slider(1.0..=100.0, self.radius, Message::RadiusChanged).step(0.01),
        ]
        .padding(20)
        .spacing(20)
        .max_width(500);
        content.into()
    }

    fn title(&self) -> String {
        String::from("ds")
    }
}
