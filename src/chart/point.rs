//! ## Point
//!
//! Module that defines the Point struct
//!

use std::f64::consts::PI;

use cairo::Context;
use palette::Rgba;

use utils;
use utils::{Drawable, Frame};

#[derive(Clone, Debug)]
enum Shape {
    Circle,
    Square,
    //Diamond,
    //Star,
}

#[derive(Clone, Debug)]
pub struct Point {
    x_coord: f64,
    y_coord: f64,
    color: Rgba,
    size: f64,
    shape: Shape,
}

impl Point {
    pub fn new(x_coord: f64, y_coord: f64) -> Point {
        Point {
            x_coord: x_coord,
            y_coord: y_coord,
            color: Rgba::new(0.5, 0.2, 0.1, 0.9),
            size: 0.01,
            shape: Shape::Circle,
        }
    }

    pub fn set_color(&mut self, color: Rgba) {
        self.color = color;
    }

    pub fn set_shape(&mut self, shape_id: &str) {
        // TODO: Move this to draw and get rid of enum??
        self.shape = match shape_id {
            "Circle" | "circle" | "c" | "o" => Shape::Circle,
            "Square" | "square" | "s" => Shape::Square,
            _ => Shape::Circle,
        };
    }

    pub fn set_size(&mut self, size: f64) {
        self.size = size;
    }

    pub fn x_coord(&self) -> f64 {
        self.x_coord
    }

    pub fn y_coord(&self) -> f64 {
        self.y_coord
    }

    pub fn set_x_coord(&mut self, val: f64) {
        self.x_coord = val;
    }

    pub fn set_y_coord(&mut self, val: f64) {
        self.y_coord = val;
    }

    pub fn map_range(&mut self, old_frame: &Frame, new_frame: &Frame) {
        self.x_coord = utils::map_range(self.x_coord,
                                        old_frame.left(), old_frame.right(),
                                        new_frame.right(), new_frame.right());
        self.y_coord = utils::map_range(self.y_coord,
                                        old_frame.bottom(), old_frame.top(),
                                        new_frame.bottom(), new_frame.top());
    }
}

impl Drawable for Point {
    fn draw(&self, cr: &Context) {
        cr.set_source_rgba(self.color.red as f64, self.color.green as f64,
                           self.color.blue as f64, self.color.alpha as f64);
        match self.shape {
            Shape::Circle => cr.arc(self.x_coord, self.y_coord, self.size, 0., 2.0*PI),
            Shape::Square => cr.rectangle(self.x_coord, self.y_coord, self.size, self.size),
        }
        cr.fill()
    }

    fn fit(&mut self, _: &Frame, _: &Frame) {}

    fn scale_size(&mut self, factor: f64) {
        self.size *= factor;
    }
}
