//! ## Draw
//!
//! Module that defines the Draw enum
//!

use cairo::Context;

/*
use line::Line;
use scatter::Scatter;
use hist::Hist;
use image::Image;

pub enum Draw {
    Line,
    Scatter,
    Hist,
    Image,
}
*/

#[derive(Clone, Debug)]
pub struct Frame {
    min_x: f64,
    max_x: f64,
    min_y: f64,
    max_y: f64,
}

impl Frame {
    pub fn new(min_x: f64, max_x: f64, min_y: f64, max_y: f64) -> Frame {
        Frame {
            min_x: min_x,
            max_x: max_x,
            min_y: min_y,
            max_y: max_y,
        }
    }

    pub fn min_x(&self) -> f64 {
        self.min_x
    }

    pub fn max_x(&self) -> f64 {
        self.max_x
    }

    pub fn min_y(&self) -> f64 {
        self.min_y
    }

    pub fn max_y(&self) -> f64 {
        self.max_y
    }
}

pub trait Drawable {
    fn draw_fn(&self, cr: &Context);
    fn fit(&mut self, frame: &Frame);
    fn min_x(&self) -> f64;
    fn max_x(&self) -> f64;
    fn min_y(&self) -> f64;
    fn max_y(&self) -> f64;
    fn frame(&self) -> Frame;
}

pub fn vec_range(vec: &Vec<f64>) -> (f64, f64) {
    let mut max_val = vec[0];
    let mut min_val = vec[0];
    for val in vec.iter() {
        max_val = val.max(max_val);
        min_val = val.min(min_val);
    }
    (min_val, max_val)
}
