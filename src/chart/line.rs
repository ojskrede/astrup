//! ## Line
//!
//! Module that defines the Line struct
//!

use cairo::Context;

use chart::point::Point;
use utils;
use utils::{Frame, Drawable, Plottable};

#[derive(Clone, Debug)]
pub struct Line {
    data_points: Vec<Point>,
    data_frame: Frame,
    color: [f64; 4],
    line_width: f64,
}

impl Line {
    pub fn new(x_data_coords: &Vec<f64>, y_data_coords: &Vec<f64>) -> Line {
        let (x_data_min, x_data_max) = utils::vec_range(&x_data_coords);
        let (y_data_min, y_data_max) = utils::vec_range(&y_data_coords);

        let mut data_points = Vec::<Point>::new();
        for (&x, &y) in x_data_coords.iter().zip(y_data_coords.iter()) {
            let mut point = Point::new(x, y);
            //point.set_color("r", 0.8);
            point.set_size(0.0);
            data_points.push(point);
        }
        Line {
            data_points: data_points,
            data_frame: Frame::new(x_data_min, x_data_max, y_data_min, y_data_max),
            color: [0.1, 0.2, 0.5, 0.9],
            line_width: 0.005,
        }
    }
}

impl Drawable for Line {
    fn draw(&self, cr: &Context) {
        let mut first_point = true;
        for data_point in self.data_points.iter() {
            if !first_point {
                cr.set_source_rgba(self.color[0], self.color[1], self.color[2], self.color[3]);
                cr.set_line_width(self.line_width);
                cr.line_to(data_point.x_coord(), data_point.y_coord());
                cr.stroke();
            }
            data_point.draw(cr);
            cr.move_to(data_point.x_coord(), data_point.y_coord());
            first_point = false;
        }
    }

    fn fit(&mut self, plot_frame: &Frame) {
        //let mut plot_points = Vec::<Point>::new();
        for data_point in self.data_points.iter_mut() {
            let plot_x = utils::change_domain(data_point.x_coord(),
                                              self.data_frame.x_min(), self.data_frame.x_max(),
                                              plot_frame.x_min(), plot_frame.x_max());
            let plot_y = utils::change_domain(data_point.y_coord(),
                                              self.data_frame.y_min(), self.data_frame.y_max(),
                                              plot_frame.y_min(), plot_frame.y_max());
            data_point.set_x_coord(plot_x);
            data_point.set_y_coord(plot_y);
            //plot_points.push(Point::new(plot_x, plot_y));
        }
        // FIXME: Do automatic fit in draw point, and only compute plot domain coordinates then
        //self.data_points = plot_points;
    }

    fn scale_size(&mut self, factor: f64) {
        for data_point in self.data_points.iter_mut() {
            data_point.scale_size(factor);
        }
        self.line_width *= factor;
    }
}

impl Plottable for Line {
    fn data_frame(&self) -> Frame {
        self.data_frame.clone()
    }

    fn data_x_min(&self) -> f64 {
        self.data_frame.x_min()
    }

    fn data_x_max(&self) -> f64 {
        self.data_frame.x_max()
    }

    fn data_y_min(&self) -> f64 {
        self.data_frame.y_min()
    }

    fn data_y_max(&self) -> f64 {
        self.data_frame.y_max()
    }

    fn set_data_frame(&mut self, new_data_frame: Frame) {
        self.data_frame = new_data_frame;
    }
}
