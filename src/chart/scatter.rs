//! ## Scatter
//!
//! Module that defines the Scatter struct
//!

use cairo::Context;

use chart::point::Point;
use utils;
use utils::{Frame, Drawable, Plottable};

#[derive(Clone, Debug)]
pub struct Scatter {
    data_points: Vec<Point>,
    global_frame: Frame,
    data_frame: Frame,
}

impl Scatter {
    pub fn new(x_data_coords: &Vec<f64>, y_data_coords: &Vec<f64>) -> Scatter {
        let (x_data_min, x_data_max) = utils::vec_range(&x_data_coords);
        let (y_data_min, y_data_max) = utils::vec_range(&y_data_coords);

        let mut data_points = Vec::<Point>::new();
        for (&x, &y) in x_data_coords.iter().zip(y_data_coords.iter()) {
            data_points.push(Point::new(x, y));
        }
        Scatter {
            data_points: data_points,
            global_frame: Frame::new(),
            data_frame: Frame::from_sides(x_data_min, x_data_max, y_data_min, y_data_max),
        }
    }
}

impl Drawable for Scatter {
    fn scale_size(&mut self, factor: f64) {
        for data_point in self.data_points.iter_mut() {
            data_point.scale_size(factor);
        }
    }

    fn fit(&mut self, canvas_global_frame: &Frame, canvas_data_frame: &Frame) {
        self.global_frame = canvas_global_frame.clone();
        self.data_frame = canvas_data_frame.clone();
        let scale_factor = self.global_frame.diag_len() / 2f64.sqrt();
        self.scale_size(scale_factor);
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
