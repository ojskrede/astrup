//! ## Scatter
//!
//! Module that defines the Scatter struct
//!

use cairo::Context;
use ndarray::AsArray;
use palette::Rgba;

use chart::point::{Point, Shape};
use utils;
use utils::{Frame, Drawable, Plottable, NonNan};

/// Scatter chart
///
/// A type used to visualise data with points. Each point coordinate is determined by the input
/// data arrays.
#[derive(Clone, Debug)]
pub struct Scatter {
    data_points: Vec<Point>,
    global_frame: Frame,
    data_frame: Frame,
    color: Rgba,
    shape: Shape,
    point_size: f64,
}

impl Scatter {
    /// Create and return a new Scatter chart
    pub fn new<'a, I: AsArray<'a, f64>>(x_data_coords: I, y_data_coords: I) -> Scatter {
        let x_view: Vec<_> = x_data_coords.into().iter().map(|v| NonNan::new(*v).unwrap()).collect();
        let y_view: Vec<_> = y_data_coords.into().iter().map(|v| NonNan::new(*v).unwrap()).collect();
        let ref x_data_min = x_view.iter().min().expect("Could not find x min");
        let ref x_data_max = x_view.iter().max().expect("Could not find x max");
        let ref y_data_min = y_view.iter().min().expect("Could not find y min");
        let ref y_data_max = y_view.iter().max().expect("Could not find y max");

        let color = Rgba::new(0.1, 0.1, 0.8, 0.9);
        let shape = Shape::Circle;
        let point_size = 0.01;
        let mut data_points = Vec::<Point>::new();
        for (ref x, ref y) in x_view.iter().zip(y_view.iter()) {
            let mut point = Point::new(x.val(), y.val());
            point.set_color(color);
            point.set_shape(shape.clone());
            point.set_size(point_size);
            data_points.push(Point::new(x.val(), y.val()));
        }
        Scatter {
            data_points: data_points,
            global_frame: Frame::new(),
            data_frame: Frame::from_sides(x_data_min.val(), x_data_max.val(),
                                          y_data_min.val(), y_data_max.val()),
            color: color,
            shape: shape,
            point_size: point_size,
        }
    }

    /// Set the scatter point color
    pub fn set_color(mut self, red: f32, green: f32, blue: f32, alpha: f32) -> Self {
        self.color = Rgba::new(red, green, blue, alpha);
        self
    }

    /// Set the scatter point size
    pub fn set_point_size(mut self, size: f64) -> Self {
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
    pub fn set_shape(mut self, shape_id: &str) -> Self {
        // TODO: Move this to draw and get rid of enum??
        self.shape = match shape_id {
            "Circle" | "circle" | "c" | "o" => Shape::Circle,
            "Square" | "square" | "s" => Shape::Square,
            "Tick" | "tick" | "t" => Shape::Tick,
            _ => Shape::Circle,
        };
        self
    }
}

impl Drawable for Scatter {
    fn scale_size(&mut self, factor: f64) {
        self.point_size *= factor;
    }

    fn fit(&mut self, canvas_global_frame: &Frame, canvas_data_frame: &Frame) {
        self.global_frame = canvas_global_frame.clone();
        self.data_frame = canvas_data_frame.clone();

        for data_point in self.data_points.iter_mut() {
            data_point.set_color(self.color);
            data_point.set_shape(self.shape.clone());
            data_point.set_size(self.point_size);
            data_point.fit(canvas_global_frame, canvas_data_frame);
        }
    }

    fn draw(&self, cr: &Context) {
        for data_point in self.data_points.iter() {
            let canvas_x = utils::map_range(data_point.x_coord(),
                                            self.data_frame.left(), self.data_frame.right(),
                                            self.global_frame.left(), self.global_frame.right());
            let canvas_y = utils::map_range(data_point.y_coord(),
                                            self.data_frame.bottom(), self.data_frame.top(),
                                            self.global_frame.bottom(), self.global_frame.top());
            let mut canvas_point = data_point.clone();
            canvas_point.set_x_coord(canvas_x);
            canvas_point.set_y_coord(canvas_y);
            canvas_point.draw(cr);
        }
    }
}

impl Plottable for Scatter {
    fn data_frame(&self) -> Frame {
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

    fn set_data_frame(&mut self, new_data_frame: Frame) {
        self.data_frame = new_data_frame;
    }
}
