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

pub trait Drawable {
    fn draw_fn(&self, cr: &Context);
}
