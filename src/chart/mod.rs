pub mod scatter;
pub mod line;
pub mod point;

use cairo::Context;

use chart::scatter::Scatter;
use chart::line::Line;
use utils::{Plottable, Drawable, Frame};

/// ## Chart
///
/// A chart is a graphical representation of data.
#[derive(Clone, Debug)]
pub enum Chart {
    Scatter(Scatter),
    Line(Line),
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

impl Drawable for Chart {
    fn draw(&self, cr: &Context) {
        match *self {
            Chart::Scatter(ref s) => s.draw(cr),
            Chart::Line(ref l) => l.draw(cr),
        }
    }

    fn fit(&mut self, plot_frame: &Frame) {
        match *self {
            Chart::Scatter(ref mut s) => s.fit(plot_frame),
            Chart::Line(ref mut l) => l.fit(plot_frame),
        }
    }

    fn scale_size(&mut self, factor: f64) {
        match *self {
            Chart::Scatter(ref mut s) => s.scale_size(factor),
            Chart::Line(ref mut l) => l.scale_size(factor),
        }
    }
}

impl Plottable for Chart {

    fn data_frame(&self) -> Frame {
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

    fn set_data_frame(&mut self, new_data_frame: Frame) {
        match *self {
            Chart::Scatter(ref mut s) => s.set_data_frame(new_data_frame),
            Chart::Line(ref mut l) => l.set_data_frame(new_data_frame),
        }
    }
}
