use iced::canvas::{Path, Stroke};
use iced::{
    canvas::{self, Cache, Canvas, Cursor, Geometry},
    executor, Application, Color, Command, Container, Element, Length, Point, Rectangle, Settings,
    Size,
};
use iced_backend::IcedBackend;
use plotters::prelude::{
    ChartBuilder, Circle, EmptyElement, IntoDrawingArea, IntoFont, LineSeries, PointSeries, Text,
    RED, WHITE,
};

pub fn main() -> iced::Result {
    Plot::run(Settings {
        antialiasing: true,
        ..Settings::default()
    })
}

struct Plot {
    plot: Cache,
}

#[derive(Debug, Clone, Copy)]
enum Message {}

impl Application for Plot {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Plot {
                plot: Default::default(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Plotters Iced Backend")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        let canvas = Canvas::new(self)
            .width(Length::Units(1000))
            .height(Length::Units(1000));

        Container::new(canvas)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .center_x()
            .center_y()
            .into()
    }
}

impl canvas::Program<Message> for Plot {
    fn draw(&self, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry> {
        let clock = self.plot.draw(bounds.size(), |mut frame| {
            let root = IcedBackend::new(&mut frame).unwrap().into_drawing_area();
            root.fill(&WHITE).unwrap();

            let root = root.margin(10, 10, 10, 10);
            // After this point, we should be able to draw construct a chart context
            let mut chart = ChartBuilder::on(&root)
                // Set the caption of the chart
                .caption("This is our first plot", ("sans-serif", 40).into_font())
                // Set the size of the label region
                .x_label_area_size(20)
                .y_label_area_size(40)
                // Finally attach a coordinate on the drawing area and make a chart context
                .build_cartesian_2d(0f32..10f32, 0f32..10f32)
                .unwrap();

            // Then we can draw a mesh
            chart
                .configure_mesh()
                // We can customize the maximum number of labels allowed for each axis
                .x_labels(5)
                .y_labels(5)
                // We can also change the format of the label text
                .y_label_formatter(&|x| format!("{:.3}", x))
                .draw()
                .unwrap();

            // And we can draw something in the drawing area
            chart
                .draw_series(LineSeries::new(
                    vec![(0.0, 0.0), (5.0, 5.0), (8.0, 7.0)],
                    &RED,
                ))
                .unwrap();
            // Similarly, we can draw point series
            chart
                .draw_series(PointSeries::of_element(
                    vec![(0.0, 0.0), (5.0, 5.0), (8.0, 7.0)],
                    5,
                    &RED,
                    &|c, s, st| {
                        return EmptyElement::at(c)    // We want to construct a composed element on-the-fly
                        + Circle::new((0,0),s,st.filled()) // At this point, the new pixel coordinate is established
                        + Text::new(format!("{:?}", c), (10, 0), ("sans-serif", 10).into_font());
                    },
                ))
                .unwrap();
        });
        vec![clock]
    }
}
