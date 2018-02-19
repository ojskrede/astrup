//! Definition of the Chart enum

pub use self::scatter::Scatter;
pub use self::line::Line;

mod scatter;
mod line;
mod point;

use cairo::Context;
use palette::Rgba;

use ::{utils, shape, chart};

/// A graphical representation of data.
#[derive(Clone, Debug)]
pub enum Chart {
    Scatter(chart::scatter::Scatter),
    Line(chart::line::Line),
}

/*
impl Chart {
    fn new(&self) -> Chart {
        match *self {
            Chart::Point(ref p) => p.new(),
            Chart::Scatter(ref s) => s.new(),
            Chart::Line(ref l) => l.new(),
        }
    }
}
*/

impl utils::Drawable for Chart {
    fn set_color_internal(&mut self, color: Rgba) {
        match *self {
            Chart::Scatter(ref mut s) => s.set_color_internal(color),
            Chart::Line(ref mut l) => l.set_color_internal(color),
        }
    }

    fn is_color_updated(&self) -> bool {
        match *self {
            Chart::Scatter(ref s) => s.is_color_updated(),
            Chart::Line(ref l) => l.is_color_updated(),
        }
    }

    fn scale_size(&mut self, factor: f64) {
        match *self {
            Chart::Scatter(ref mut s) => s.scale_size(factor),
            Chart::Line(ref mut l) => l.scale_size(factor),
        }
    }

    fn fit(&mut self, global_frame: &shape::Rectangle, data_frame: &shape::Rectangle) {
        match *self {
            Chart::Scatter(ref mut s) => s.fit(global_frame, data_frame),
            Chart::Line(ref mut l) => l.fit(global_frame, data_frame),
        }
    }

    fn draw(&self, cr: &Context, fig_rel_height: f64, fig_rel_width: f64) {
        match *self {
            Chart::Scatter(ref s) => s.draw(cr, fig_rel_height, fig_rel_width),
            Chart::Line(ref l) => l.draw(cr, fig_rel_height, fig_rel_width),
        }
    }
}

impl utils::Plottable for Chart {

    fn data_frame(&self) -> shape::Rectangle {
        match *self {
            Chart::Scatter(ref s) => s.data_frame(),
            Chart::Line(ref l) => l.data_frame(),
        }
    }

    fn data_x_min(&self) -> f64 {
        match *self {
            Chart::Scatter(ref s) => s.data_x_min(),
            Chart::Line(ref l) => l.data_x_min(),
        }
    }

    fn data_x_max(&self) -> f64 {
        match *self {
            Chart::Scatter(ref s) => s.data_x_max(),
            Chart::Line(ref l) => l.data_x_max(),
        }
    }

    fn data_y_min(&self) -> f64 {
        match *self {
            Chart::Scatter(ref s) => s.data_y_min(),
            Chart::Line(ref l) => l.data_y_min(),
        }
    }

    fn data_y_max(&self) -> f64 {
        match *self {
            Chart::Scatter(ref s) => s.data_y_max(),
            Chart::Line(ref l) => l.data_y_max(),
        }
    }

    fn set_data_frame(&mut self, new_data_frame: shape::Rectangle) {
        match *self {
            Chart::Scatter(ref mut s) => s.set_data_frame(new_data_frame),
            Chart::Line(ref mut l) => l.set_data_frame(new_data_frame),
        }
    }
}
