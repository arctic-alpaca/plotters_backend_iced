use iced::widget::canvas::{Frame, Path, Stroke};
use iced::{Point, Size};
use plotters_backend::{
    BackendColor, BackendCoord, BackendStyle, BackendTextStyle, DrawingBackend, DrawingErrorKind,
    FontStyle, FontTransform,
};

#[derive(Debug)]
pub struct IcedError;

impl std::fmt::Display for IcedError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "{:?}", self)
    }
}

impl std::error::Error for IcedError {}

/// The drawing backend that is backed with a Cairo context
pub struct IcedBackend<'a> {
    frame: &'a mut Frame,
    width: u32,
    height: u32,
    init_flag: bool,
}

impl<'a> IcedBackend<'a> {
    pub fn new(frame: &'a mut Frame) -> Result<Self, IcedError> {
        let width = frame.width() as u32;
        let height = frame.height() as u32;
        let ret = Self {
            frame,
            width,
            height,
            init_flag: false,
        };
        Ok(ret)
    }

    fn from_backend_color_to_iced_color(&self, color: &BackendColor) -> iced::Color {
        iced::Color::from_rgba(
            f32::from(color.rgb.0) / 255.0,
            f32::from(color.rgb.1) / 255.0,
            f32::from(color.rgb.2) / 255.0,
            color.alpha as f32,
        )
    }
    fn from_backend_point_to_iced_point(&self, point: &(i32, i32)) -> iced::Point {
        Point::new(point.0 as f32, point.1 as f32)
    }

    fn from_backend_style_to_iced_stroke<S: BackendStyle>(
        &self,
        style: &S,
    ) -> iced::widget::canvas::Stroke {
        let mut stroke: Stroke = Default::default();
        stroke.color = self.from_backend_color_to_iced_color(&style.color());
        stroke.width = style.stroke_width() as f32;
        stroke
    }
}

impl<'a> DrawingBackend for IcedBackend<'a> {
    type ErrorType = IcedError;

    fn get_size(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    fn ensure_prepared(&mut self) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        Ok(())
    }

    fn present(&mut self) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        Ok(())
    }

    fn draw_pixel(
        &mut self,
        point: (i32, i32),
        color: BackendColor,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        self.frame.fill_rectangle(
            self.from_backend_point_to_iced_point(&point),
            Size::new(1.0, 1.0),
            self.from_backend_color_to_iced_color(&color),
        );
        Ok(())
    }

    fn draw_line<S: BackendStyle>(
        &mut self,
        from: (i32, i32),
        to: (i32, i32),
        style: &S,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        self.frame.stroke(
            &Path::line(
                self.from_backend_point_to_iced_point(&from),
                self.from_backend_point_to_iced_point(&to),
            ),
            self.from_backend_style_to_iced_stroke(style),
        );
        Ok(())
    }

    fn draw_rect<S: BackendStyle>(
        &mut self,
        upper_left: (i32, i32),
        bottom_right: (i32, i32),
        style: &S,
        fill: bool,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        if fill {
            let size = Size::new(
                (bottom_right.0 - upper_left.0) as f32,
                (bottom_right.1 - upper_left.1) as f32,
            );
            self.frame.fill_rectangle(
                self.from_backend_point_to_iced_point(&upper_left),
                size,
                self.from_backend_color_to_iced_color(&style.color()),
            );
        } else {
            let path = Path::new(|builder| {
                builder.move_to(self.from_backend_point_to_iced_point(&upper_left));
                builder.line_to(
                    self.from_backend_point_to_iced_point(&(bottom_right.0, upper_left.1)),
                );
                builder.line_to(self.from_backend_point_to_iced_point(&bottom_right));
                builder.line_to(
                    self.from_backend_point_to_iced_point(&(upper_left.0, bottom_right.1)),
                );
                builder.line_to(self.from_backend_point_to_iced_point(&upper_left));
            });
            self.frame
                .stroke(&path, self.from_backend_style_to_iced_stroke(style));
        }

        Ok(())
    }

    fn draw_path<S: BackendStyle, I: IntoIterator<Item = BackendCoord>>(
        &mut self,
        path: I,
        style: &S,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        let drawing_path = Path::new(|builder| {
            let iterator = path.into_iter();
            for (index, point) in iterator.enumerate() {
                if index == 0 {
                    builder.move_to(self.from_backend_point_to_iced_point(&point));
                } else {
                    builder.line_to(self.from_backend_point_to_iced_point(&point));
                }
            }
        });
        self.frame
            .stroke(&drawing_path, self.from_backend_style_to_iced_stroke(style));
        Ok(())
    }

    fn draw_circle<S: BackendStyle>(
        &mut self,
        center: (i32, i32),
        radius: u32,
        style: &S,
        fill: bool,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        if fill {
            self.frame.fill(
                &Path::circle(
                    self.from_backend_point_to_iced_point(&center),
                    radius as f32,
                ),
                self.from_backend_color_to_iced_color(&style.color()),
            );
        } else {
            self.frame.stroke(
                &Path::circle(
                    self.from_backend_point_to_iced_point(&center),
                    radius as f32,
                ),
                self.from_backend_style_to_iced_stroke(style),
            );
        }

        Ok(())
    }

    fn fill_polygon<S: BackendStyle, I: IntoIterator<Item = BackendCoord>>(
        &mut self,
        vert: I,
        style: &S,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        let path = Path::new(|builder| {
            let iterator = vert.into_iter();
            for (index, point) in iterator.enumerate() {
                if index == 0 {
                    builder.move_to(self.from_backend_point_to_iced_point(&point));
                } else {
                    builder.line_to(self.from_backend_point_to_iced_point(&point));
                }
            }
        });
        self.frame
            .fill(&path, self.from_backend_color_to_iced_color(&style.color()));
        Ok(())
    }
}
