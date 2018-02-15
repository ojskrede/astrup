//! ## Point
//!
//! Module that defines the Point struct
//!

use std::f64::consts::PI;

use cairo::Context;
use palette::Rgba;

use ::{utils, shape, coord};

#[derive(Clone, Debug)]
pub enum Shape {
    Circle,
    Square,
    Tick,
    //Diamond,
    //Star,
}

#[derive(Clone, Debug)]
pub struct Point {
    coord: coord::Coord,
    color: Rgba,
    size: f64,
    shape: Shape,
}

impl Point {
    pub fn new(x_coord: f64, y_coord: f64) -> Point {
        Point {
            coord: coord::Coord::new(x_coord, y_coord),
            color: Rgba::new(0.5, 0.2, 0.1, 0.9),
            size: 0.01,
            shape: Shape::Circle,
        }
    }

    pub fn set_coord(&mut self, x_val: f64, y_val: f64) {
        self.coord.set_x(x_val);
        self.coord.set_y(y_val);
    }

    pub fn set_x_coord(&mut self, val: f64) {
        self.coord.set_x(val);
    }

    pub fn set_y_coord(&mut self, val: f64) {
        self.coord.set_y(val);
    }

    /// Set the color of the point
    pub fn set_color(&mut self, color: Rgba) {
        self.color = color;
    }

    /// Set the color of the point
    pub fn set_color_rgb(&mut self, red: f32, green: f32, blue: f32) {
        let red = red.max(0.0);
        let red = red.min(1.0);
        let green = green.max(0.0);
        let green = green.min(1.0);
        let blue = blue.max(0.0);
        let blue = blue.min(1.0);
        self.color = Rgba::new(red, green, blue, 1.0);
    }

    /// Set the plot background color. **Note**: This is different from the canvas background color.
    pub fn set_color_rgba(&mut self, red: f32, green: f32, blue: f32, alpha: f32) {
        let red = red.max(0.0);
        let red = red.min(1.0);
        let green = green.max(0.0);
        let green = green.min(1.0);
        let blue = blue.max(0.0);
        let blue = blue.min(1.0);
        let alpha = alpha.max(0.0);
        let alpha = alpha.min(1.0);
        self.color = Rgba::new(red, green, blue, alpha);
    }

    pub fn set_shape(&mut self, shape: Shape) {
        self.shape = shape;
    }

    pub fn set_size(&mut self, size: f64) {
        self.size = size;
    }

    pub fn coord(&self) -> coord::Coord {
        self.coord.clone()
    }

    pub fn x_coord(&self) -> f64 {
        self.coord.x()
    }

    pub fn y_coord(&self) -> f64 {
        self.coord.y()
    }

    pub fn map_range(&mut self, old_frame: &shape::Rectangle, new_frame: &shape::Rectangle) {
        let new_x = utils::map_range(self.x_coord(),
                                     old_frame.left(), old_frame.right(),
                                     new_frame.right(), new_frame.right());
        let new_y = utils::map_range(self.y_coord(),
                                     old_frame.bottom(), old_frame.top(),
                                     new_frame.bottom(), new_frame.top());
        self.set_coord(new_x, new_y);
    }
}

impl utils::Drawable for Point {
    fn scale_size(&mut self, factor: f64) {
        self.size *= factor;
    }

    fn fit(&mut self, canvas_global_frame: &shape::Rectangle, _: &shape::Rectangle) {
        self.scale_size(canvas_global_frame.diag_len() / 2f64.sqrt());
    }

    fn draw(&self, cr: &Context, fig_rel_height: f64, fig_rel_width: f64) {
        cr.set_source_rgba(self.color.red as f64, self.color.green as f64,
                           self.color.blue as f64, self.color.alpha as f64);
        match self.shape {
            // TODO: Scale size of circle and square
            Shape::Circle => cr.arc(self.coord.x(), self.coord.y(), self.size, 0., 2.0*PI),
            Shape::Square => cr.rectangle(self.coord.x(), self.coord.y(), self.size, self.size),
            Shape::Tick => {
                // Vertical tick
                let start = coord::Coord::new(self.coord.x(), self.coord.y() - self.size);
                let end = coord::Coord::new(self.coord.x(), self.coord.y() + self.size);
                let direction = start.unit_direction_to(&end);
                let size = self.size * (direction.x().abs() * fig_rel_width + direction.y().abs() * fig_rel_height);
                cr.set_line_width(size / 4.0);
                cr.move_to(self.coord.x(), self.coord.y() - size);
                cr.line_to(self.coord.x(), self.coord.y() + size);
                cr.stroke();
            },
        }
        cr.fill()
    }
}
