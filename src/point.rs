//! ## Point
//!
//! Module that defines the Point struct
//!

use std::f64::consts::PI;

use cairo::Context;

use utils;
use utils::{Drawable, Frame};
use color;

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
    color: [f64; 4],
    size: f64,
    shape: Shape,
}

impl Point {
    pub fn new(x_coord: f64, y_coord: f64) -> Point {
        Point {
            x_coord: x_coord,
            y_coord: y_coord,
            color: color::black(1.0),
            size: 0.005,
            shape: Shape::Circle,
        }
    }

    pub fn set_color(&mut self, color_id: &str, mut alpha: f64) {
        alpha = alpha.max(0.0).min(1.0);
        self.color = match color_id {
            "Red" | "red" | "R" | "r" => color::red(alpha),
            "Green" | "green" | "G" | "g" => color::green(alpha),
            "Blue" | "blue" | "B" | "b" => color::blue(alpha),
            "Black" | "black" | "K" | "k" => color::black(alpha),
            "Gray" | "gray" | "O" | "o" => color::gray(alpha),
            "White" | "white" | "W" | "w" => color::white(alpha),
            _ => color::red(alpha),
        };
    }

    pub fn set_shape(&mut self, shape_id: &str) {
        // TODO: Move this to draw_fn and get rid of enum??
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

    pub fn change_domain(&mut self, old_frame: &Frame, new_frame: &Frame) {
        self.x_coord = utils::change_domain(self.x_coord,
                                            old_frame.x_min(), old_frame.x_max(),
                                            new_frame.x_min(), new_frame.x_max());
        self.y_coord = utils::change_domain(self.y_coord,
                                            old_frame.y_min(), old_frame.y_max(),
                                            new_frame.y_min(), new_frame.y_max());
    }
}

impl Drawable for Point {
    fn draw_fn(&self, cr: &Context) {
        cr.set_source_rgba(self.color[0], self.color[1], self.color[2], self.color[3]);
        match self.shape {
            Shape::Circle => cr.arc(self.x_coord, self.y_coord, self.size, 0., 2.0*PI),
            Shape::Square => cr.rectangle(self.x_coord, self.y_coord, self.size, self.size),
        }
        cr.fill()
    }

    fn fit(&mut self, frame: &Frame) {}

    // TODO: The following does not really make sense for this struct, and only really for
    // the plot variants (line, scatter, etc.).
    // Consider splitting the Drawable trait into a Drawable and a PlotVariant trait.
    fn data_frame(&self) -> Frame {
        Frame::new(self.x_coord, self.x_coord, self.y_coord, self.y_coord)
    }

    fn set_data_frame(&mut self, new_data_frame: Frame) {}

    fn data_x_min(&self) -> f64 {
        self.x_coord
    }

    fn data_x_max(&self) -> f64 {
        self.x_coord
    }

    fn data_y_min(&self) -> f64 {
        self.y_coord
    }

    fn data_y_max(&self) -> f64 {
        self.y_coord
    }
}
