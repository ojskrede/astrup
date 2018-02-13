//! Definition of the Chart enum

pub mod scatter;
pub mod line;
pub mod point;

use cairo::Context;

use ::{utils, frame, chart};

/// ## Chart
///
/// A chart is a graphical representation of data.
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
    fn scale_size(&mut self, factor: f64) {
        match *self {
            Chart::Scatter(ref mut s) => s.scale_size(factor),
            Chart::Line(ref mut l) => l.scale_size(factor),
        }
    }

    fn fit(&mut self, global_frame: &frame::Frame, data_frame: &frame::Frame) {
        match *self {
            Chart::Scatter(ref mut s) => s.fit(global_frame, data_frame),
            Chart::Line(ref mut l) => l.fit(global_frame, data_frame),
        }
    }

    fn draw(&self, cr: &Context) {
        match *self {
            Chart::Scatter(ref s) => s.draw(cr),
            Chart::Line(ref l) => l.draw(cr),
        }
    }
}

impl utils::Plottable for Chart {

    fn data_frame(&self) -> frame::Frame {
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

    fn set_data_frame(&mut self, new_data_frame: frame::Frame) {
        match *self {
            Chart::Scatter(ref mut s) => s.set_data_frame(new_data_frame),
            Chart::Line(ref mut l) => l.set_data_frame(new_data_frame),
        }
    }
}
