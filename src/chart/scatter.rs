//! ## Scatter
//!
//! Module that defines the Scatter struct
//!

use cairo::Context;
use ndarray::AsArray;
use palette::Srgba;

use {chart, color, shape, utils};
use utils::Drawable;

/// Scatter chart
///
/// A type used to visualise data with points. Each point coordinate is determined by the input
/// data arrays.
#[derive(Clone, Debug)]
pub struct Scatter {
    data_points: Vec<chart::point::Point>,
    global_frame: shape::Rectangle,
    data_frame: shape::Rectangle,
    color: color::Color,
    is_color_updated: bool,
    shape: chart::point::Shape,
    point_size: f64,
}

impl Scatter {
    /// Create and return a new Scatter chart
    pub fn new<'a, I: AsArray<'a, f64>>(x_data_coords: I, y_data_coords: I) -> Scatter {
        let x_view: Vec<_> = x_data_coords
            .into()
            .iter()
            .map(|v| utils::NonNan::new(*v).unwrap())
            .collect();
        let y_view: Vec<_> = y_data_coords
            .into()
            .iter()
            .map(|v| utils::NonNan::new(*v).unwrap())
            .collect();
        let x_data_min = &x_view.iter().min().expect("Could not find x min");
        let x_data_max = &x_view.iter().max().expect("Could not find x max");
        let y_data_min = &y_view.iter().min().expect("Could not find y min");
        let y_data_max = &y_view.iter().max().expect("Could not find y max");

        let point_color = color::Color::with_custom(&color::CustomColor::Blue);
        let shape = chart::point::Shape::Circle;
        let point_size = 0.002;
        let mut data_points = Vec::<chart::point::Point>::new();
        for (x, y) in x_view.iter().zip(y_view.iter()) {
            let mut point = chart::point::Point::new(x.val(), y.val());
            point.set_color_internal(point_color.as_srgba());
            point.set_shape(shape.clone());
            point.set_size(point_size);
            data_points.push(chart::point::Point::new(x.val(), y.val()));
        }
        Scatter {
            data_points: data_points,
            global_frame: shape::Rectangle::new(),
            data_frame: shape::Rectangle::with_boundaries(
                x_data_min.val(),
                x_data_max.val(),
                y_data_min.val(),
                y_data_max.val(),
            ),
            color: point_color,
            is_color_updated: false,
            shape: shape,
            point_size: point_size,
        }
    }

    /// Set the point color
    pub fn set_color(&mut self, color: &color::CustomColor) -> &mut Self {
        self.color.set_color_custom(color);
        self.is_color_updated = true;
        self
    }

    /// Set the point color
    pub fn set_color_rgb(&mut self, red: f32, green: f32, blue: f32) -> &mut Self {
        self.color.set_color_rgb(red, green, blue);
        self.is_color_updated = true;
        self
    }

    /// Set the point color
    pub fn set_color_rgba(&mut self, red: f32, green: f32, blue: f32, alpha: f32) -> &mut Self {
        self.color.set_color_rgba(red, green, blue, alpha);
        self.is_color_updated = true;
        self
    }

    /// Set the point color
    pub fn set_color_rgb_u8(&mut self, red: u8, green: u8, blue: u8) -> &mut Self {
        self.color.set_color_rgb_u8(red, green, blue);
        self.is_color_updated = true;
        self
    }

    /// Set the point color
    pub fn set_color_rgba_u8(&mut self, red: u8, green: u8, blue: u8, alpha: u8) -> &mut Self {
        self.color.set_color_rgba_u8(red, green, blue, alpha);
        self.is_color_updated = true;
        self
    }

    /// Set the point color
    pub fn set_color_html(&mut self, color: &color::HtmlColor) -> &mut Self {
        self.color.set_color_html(color);
        self.is_color_updated = true;
        self
    }

    /// Set the scatter point size
    pub fn set_point_size(&mut self, size: f64) -> &mut Self {
        self.point_size = size;
        self
    }

    /// Set the shape of the scatter point. Circle, tick, or square.
    ///
    /// TODO: Circle is currently filled. This is perhaps not the expected shape.
    ///
    /// | Input                              | Shape  |
    /// | ---------------------------------- | ------ |
    /// | "Circle" or "circle" or "c" or "o" | Circle |
    /// | "Square" or "square" or "s"        | Square |
    /// | "Tick" or "tick" or "t"            | Tick   |
    /// | Any other &str                     | Circle |
    #[allow(unknown_lints)]
    #[allow(match_same_arms)]
    pub fn set_shape(&mut self, shape_id: &str) -> &mut Self {
        // TODO: Move this to draw and get rid of enum??
        self.shape = match shape_id {
            "Circle" | "circle" | "c" | "o" => chart::point::Shape::Circle,
            "Square" | "square" | "s" => chart::point::Shape::Square,
            "Tick" | "tick" | "t" => chart::point::Shape::Tick,
            _ => chart::point::Shape::Circle,
        };
        self
    }
}

impl utils::Drawable for Scatter {
    fn set_color_internal(&mut self, color: Srgba) {
        self.color.set_color(color);
        self.is_color_updated = true;
    }

    fn is_color_updated(&self) -> bool {
        self.is_color_updated
    }

    fn scale_size(&mut self, factor: f64) {
        self.point_size *= factor;
    }

    fn fit(
        &mut self,
        canvas_global_frame: &shape::Rectangle,
        canvas_data_frame: &shape::Rectangle,
    ) {
        self.global_frame = canvas_global_frame.clone();
        self.data_frame = canvas_data_frame.clone();

        for data_point in &mut self.data_points {
            data_point.set_color_internal(self.color.as_srgba());
            data_point.set_shape(self.shape.clone());
            data_point.set_size(self.point_size);
            data_point.fit(canvas_global_frame, canvas_data_frame);
        }
    }

    fn draw(&self, cr: &Context, fig_rel_height: f64, fig_rel_width: f64) {
        for data_point in &self.data_points {
            let canvas_x = utils::map_range(
                data_point.x_coord(),
                self.data_frame.left(),
                self.data_frame.right(),
                self.global_frame.left(),
                self.global_frame.right(),
            );
            let canvas_y = utils::map_range(
                data_point.y_coord(),
                self.data_frame.bottom(),
                self.data_frame.top(),
                self.global_frame.bottom(),
                self.global_frame.top(),
            );
            let mut canvas_point = data_point.clone();
            canvas_point.set_x_coord(canvas_x);
            canvas_point.set_y_coord(canvas_y);
            canvas_point.draw(cr, fig_rel_height, fig_rel_width);
        }
    }
}

impl utils::Plottable for Scatter {
    fn data_frame(&self) -> shape::Rectangle {
        self.data_frame.clone()
    }

    fn data_x_min(&self) -> f64 {
        self.data_frame.left()
    }

    fn data_x_max(&self) -> f64 {
        self.data_frame.right()
    }

    fn data_y_min(&self) -> f64 {
        self.data_frame.bottom()
    }

    fn data_y_max(&self) -> f64 {
        self.data_frame.top()
    }

    fn set_data_frame(&mut self, new_data_frame: shape::Rectangle) {
        self.data_frame = new_data_frame;
    }
}
