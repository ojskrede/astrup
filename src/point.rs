//! ## Point
//!
//! Module that defines the Point struct
//!

use std::f64::consts::PI;

use cairo::Context;

use draw::Drawable;
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
            color: color::blue(1.0),
            size: 0.01,
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
            "Gray" | "gray" | "O" | "o" => color::black(alpha),
            "White" | "white" | "W" | "w" => color::black(alpha),
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

    fn min_x(&self) -> f64 {
        self.x_coord
    }

    fn max_x(&self) -> f64 {
        self.x_coord
    }

    fn min_y(&self) -> f64 {
        self.y_coord
    }

    fn max_y(&self) -> f64 {
        self.y_coord
    }
}
