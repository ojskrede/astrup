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
    global_frame: Frame,
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
            data_frame: Frame::from_sides(x_data_min, x_data_max, y_data_min, y_data_max),
            global_frame: Frame::new(),
            color: [0.1, 0.2, 0.5, 0.9],
            line_width: 0.005,
        }
    }
}

impl Drawable for Line {
    fn scale_size(&mut self, factor: f64) {
        for data_point in self.data_points.iter_mut() {
            data_point.scale_size(factor);
        }
        self.line_width *= factor;
    }

    fn fit(&mut self, canvas_frame: &Frame) {
        self.global_frame = canvas_frame.clone();
        let scale_factor = self.global_frame.diag_len();
        self.scale_size(scale_factor);
    }

    fn draw(&self, cr: &Context) {
        // TODO: If it is not important to keep the original data, we could use
        // data_point.map_range() here.
        let mut first_point = true;
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

            if !first_point {
                cr.set_source_rgba(self.color[0], self.color[1], self.color[2], self.color[3]);
                cr.set_line_width(self.line_width);
                cr.line_to(canvas_point.x_coord(), canvas_point.y_coord());
                cr.stroke();
            }
            canvas_point.draw(cr);
            cr.move_to(canvas_point.x_coord(), canvas_point.y_coord());
            first_point = false;
        }
    }
}

impl Plottable for Line {
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
