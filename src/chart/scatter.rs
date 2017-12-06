//! ## Scatter
//!
//! Module that defines the Scatter struct
//!

use cairo::Context;
use ndarray::AsArray;

use chart::point::Point;
use utils;
use utils::{Frame, Drawable, Plottable, NonNan};


#[derive(Clone, Debug)]
pub struct Scatter {
    data_points: Vec<Point>,
    global_frame: Frame,
    data_frame: Frame,
}

impl Scatter {
    pub fn new<'a, I: AsArray<'a, f64>>(x_data_coords: I, y_data_coords: I) -> Scatter {
        let x_view: Vec<_> = x_data_coords.into().iter().map(|v| NonNan::new(*v).unwrap()).collect();
        let y_view: Vec<_> = y_data_coords.into().iter().map(|v| NonNan::new(*v).unwrap()).collect();
        let ref x_data_min = x_view.iter().min().expect("Could not find x min");
        let ref x_data_max = x_view.iter().max().expect("Could not find x max");
        let ref y_data_min = y_view.iter().min().expect("Could not find y min");
        let ref y_data_max = y_view.iter().max().expect("Could not find y max");

        let mut data_points = Vec::<Point>::new();
        for (ref x, ref y) in x_view.iter().zip(y_view.iter()) {
            data_points.push(Point::new(x.val(), y.val()));
        }
        Scatter {
            data_points: data_points,
            global_frame: Frame::new(),
            data_frame: Frame::from_sides(x_data_min.val(), x_data_max.val(),
                                          y_data_min.val(), y_data_max.val()),
        }
    }
}

impl Drawable for Scatter {
    fn scale_size(&mut self, _: f64) {}

    fn fit(&mut self, canvas_global_frame: &Frame, canvas_data_frame: &Frame) {
        self.global_frame = canvas_global_frame.clone();
        self.data_frame = canvas_data_frame.clone();

        for data_point in self.data_points.iter_mut() {
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
