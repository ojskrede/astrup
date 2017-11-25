//! ## Scatter
//!
//! Module that defines the Scatter struct
//!

use cairo::Context;

use point::Point;
use utils::{Frame, Drawable, vec_range};

#[derive(Clone, Debug)]
pub struct Scatter {
    points: Vec<Point>,
    frame: Frame,
}

impl Scatter {
    pub fn new(x_coords: &Vec<f64>, y_coords: &Vec<f64>) -> Scatter {
        let (min_x, max_x) = vec_range(&x_coords);
        let (min_y, max_y) = vec_range(&y_coords);

        let mut points = Vec::<Point>::new();
        for (&x, &y) in x_coords.iter().zip(y_coords.iter()) {
            points.push(Point::new(x, y));
        }
        Scatter {
            points: points,
            frame: Frame::new(min_x, max_x, min_y, max_y),
        }
    }
}

impl Drawable for Scatter {
    fn draw_fn(&self, cr: &Context) {
        for point in self.points.iter() {
            point.draw_fn(cr)
        }
    }

    fn fit(&mut self, frame: &Frame) {
        let mut fitted_points = Vec::<Point>::new();
        for point in self.points.iter() {
            let mut scaled_x = 0.5;
            if self.frame.max_x() != self.frame.min_x() {
                scaled_x = (point.x_coord() - self.frame.min_x()) / (self.frame.max_x() - self.frame.min_x()) * frame.max_x() +
                           (self.frame.max_x() - point.x_coord()) / (self.frame.max_x() - self.frame.min_x()) * frame.min_x();
            }
            let mut scaled_y = 0.5;
            if self.frame.max_y() != self.frame.min_y() {
                scaled_y = (point.y_coord() - self.frame.min_y()) / (self.frame.max_y() - self.frame.min_y()) * frame.max_y() +
                           (self.frame.max_y() - point.y_coord()) / (self.frame.max_y() - self.frame.min_y()) * frame.min_y();
            }
            fitted_points.push(Point::new(scaled_x, scaled_y));
        }
        self.points = fitted_points;
    }

    fn min_x(&self) -> f64 {
        self.frame.min_x()
    }

    fn max_x(&self) -> f64 {
        self.frame.max_x()
    }

    fn min_y(&self) -> f64 {
        self.frame.min_y()
    }

    fn max_y(&self) -> f64 {
        self.frame.max_y()
    }

    fn frame(&self) -> Frame {
        self.frame.clone()
    }
}
