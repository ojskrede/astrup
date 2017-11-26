//! ## Scatter
//!
//! Module that defines the Scatter struct
//!

use cairo::Context;

use point::Point;
use utils;
use utils::{Frame, Drawable};

#[derive(Clone, Debug)]
pub struct Scatter {
    data_points: Vec<Point>,
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
            data_frame: Frame::new(x_data_min, x_data_max, y_data_min, y_data_max),
        }
    }
}

impl Drawable for Scatter {
    fn draw_fn(&self, cr: &Context) {
        for data_point in self.data_points.iter() {
            data_point.draw_fn(cr)
        }
    }

    fn fit(&mut self, plot_frame: &Frame) {
        let mut plot_points = Vec::<Point>::new();
        for data_point in self.data_points.iter() {
            let plot_x = utils::change_domain(data_point.x_coord(),
                                              self.data_frame.x_min(), self.data_frame.x_max(),
                                              plot_frame.x_min(), plot_frame.x_max());
            let plot_y = utils::change_domain(data_point.y_coord(),
                                              self.data_frame.y_min(), self.data_frame.y_max(),
                                              plot_frame.y_min(), plot_frame.y_max());
            plot_points.push(Point::new(plot_x, plot_y));
        }
        // FIXME: Do automatic fit in draw point, and only compute plot domain coordinates then
        self.data_points = plot_points;
    }

    fn data_frame(&self) -> Frame {
        self.data_frame.clone()
    }
}
