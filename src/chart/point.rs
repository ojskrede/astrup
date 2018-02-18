//! ## Point
//!
//! Module that defines the Point struct
//!

use std::f64::consts::PI;
use failure::Error;

use cairo::Context;
use palette::Rgba;

use ::{utils, shape, coord, color};

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
    color: color::Color,
    is_color_updated: bool,
    size: f64,
    shape: Shape,
}

impl Point {
    pub fn new(x_coord: f64, y_coord: f64) -> Point {
        Point {
            coord: coord::Coord::new_from(x_coord, y_coord),
            color: color::Color::new_default("blue"),
            is_color_updated: false,
            size: 0.003,
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

    /// Set the point color using the default, built in colors
    pub fn set_color(&mut self, color_name: &str) {
        self.color.set_color_default(color_name);
        self.is_color_updated = true;
    }

    /// Set the point color
    pub fn set_color_rgb(&mut self, red: f32, green: f32, blue: f32) {
        self.color.set_color_rgb(red, green, blue);
        self.is_color_updated = true;
    }

    /// Set the point color
    pub fn set_color_rgba(&mut self, red: f32, green: f32, blue: f32, alpha: f32) {
        self.color.set_color_rgba(red, green, blue, alpha);
        self.is_color_updated = true;
    }

    /// Set the point color
    pub fn set_color_rgb_u8(&mut self, red: u8, green: u8, blue: u8) {
        self.color.set_color_rgb_u8(red, green, blue);
        self.is_color_updated = true;
    }

    /// Set the point color
    pub fn set_color_rgba_u8(&mut self, red: u8, green: u8, blue: u8, alpha: u8) {
        self.color.set_color_rgba_u8(red, green, blue, alpha);
        self.is_color_updated = true;
    }

    /// Set the point color from name. See the [palette
    /// documentation](https://docs.rs/palette/0.3.0/palette/named/index.html) for more info.
    pub fn set_color_str(&mut self, color_name: &str) -> Result<(), Error> {
        self.color.set_color_str(color_name)?;
        self.is_color_updated = true;
        Ok(())
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
    fn set_color_internal(&mut self, color: Rgba) {
        self.color.set_color(color);
        self.is_color_updated = true;
    }

    fn is_color_updated(&self) -> bool {
        self.is_color_updated
    }

    fn scale_size(&mut self, factor: f64) {
        self.size *= factor;
    }

    fn fit(&mut self, canvas_global_frame: &shape::Rectangle, _: &shape::Rectangle) {
        self.scale_size(canvas_global_frame.diag_len());
    }

    fn draw(&self, cr: &Context, fig_rel_height: f64, fig_rel_width: f64) {
        let point_color = self.color.as_rgba();
        cr.set_source_rgba(point_color.red as f64, point_color.green as f64,
                           point_color.blue as f64, point_color.alpha as f64);
        match self.shape {
            // TODO: Scale size of circle and square
            Shape::Circle => cr.arc(self.coord.x(), self.coord.y(), self.size, 0., 2.0*PI),
            Shape::Square => cr.rectangle(self.coord.x(), self.coord.y(), self.size, self.size),
            Shape::Tick => {
                // Vertical tick
                let start = coord::Coord::new_from(self.coord.x(), self.coord.y() - self.size);
                let end = coord::Coord::new_from(self.coord.x(), self.coord.y() + self.size);
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
