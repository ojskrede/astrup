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
pub struct Text {
    content: String,
    font_size: f64,
}

impl Text {
    pub fn new(content: &str) -> Text {
        Text {
            content: String::from(content),
            font_size: 0.04,
        }
    }

    pub fn content(&self) -> String {
        self.content.clone()
    }

    pub fn font_size(&self) -> f64 {
        self.font_size
    }

    pub fn set_content(&mut self, content: &str) {
        self.content = String::from(content);
    }

    pub fn set_font_size(&mut self, size: f64) {
        self.font_size = size;
    }

    pub fn scale_size(&mut self, factor: f64) {
        self.font_size *= factor;
    }
}

/// A frame defined by its boundaries.
#[derive(Clone, Debug)]
pub struct Frame {
    left: f64,
    right: f64,
    top: f64,
    bottom: f64,
}

impl Frame {
    pub fn new() -> Frame {
        Frame {
            left: 0.0,
            right: 1.0,
            top: 0.0,
            bottom: 1.0,
        }
    }

    pub fn from_sides(left: f64, right: f64, top: f64, bottom: f64) -> Frame {
        Frame {
            left: left,
            right: right,
            top: top,
            bottom: bottom,
        }
    }

    pub fn set(&mut self, left: f64, right: f64, top: f64, bottom: f64) {
        self.left = left;
        self.right = right;
        self.top = top;
        self.bottom = bottom;
    }

    pub fn left(&self) -> f64 {
        self.left
    }

    pub fn right(&self) -> f64 {
        self.right
    }

    pub fn top(&self) -> f64 {
        self.top
    }

    pub fn bottom(&self) -> f64 {
        self.bottom
    }

    pub fn height(&self) -> f64 {
        self.top - self.bottom
    }

    pub fn width(&self) -> f64 {
        self.right - self.left
    }

    pub fn diag_len(&self) -> f64 {
        let delta_x = self.right - self.left;
        let delta_y = self.top - self.bottom;
        (delta_x * delta_x + delta_y * delta_y).sqrt()
    }

    pub fn set_left(&mut self, val: f64) {
        self.left = val;
    }

    pub fn set_right(&mut self, val: f64) {
        self.right = val;
    }

    pub fn set_top(&mut self, val: f64) {
        self.top = val;
    }

    pub fn set_bottom(&mut self, val: f64) {
        self.bottom = val;
    }

    /// Returns this frame mapped to a different `reference_frame`. This is useful when one wants
    /// to map the local frame (`self`) to a global frame (`reference_frame`).
    ///
    /// Each coordinate x on any side S (top, left, bottom, right) is then mapped as
    ///
    /// ```
    ///   x -> ((old_max - x)*new_min + (x - old_min)*new_max) / (old_max - old_min)
    /// ```
    ///
    /// Since the local frame is bounded by (0, 1) on each side, this simplifies to
    ///
    /// ```
    ///   x -> new_min + (new_max - new_min) * x
    /// ```
    pub fn relative_to(&self, reference: &Frame) -> Frame {
        let new_left = reference.left() + reference.width() * self.left();
        let new_right = reference.left() + reference.width() * self.right();
        let new_bottom = reference.bottom() + reference.height() * self.bottom();
        let new_top = reference.bottom() + reference.height() * self.top();
        Frame::from_sides(new_left, new_right, new_bottom, new_top)
    }
}

/// ## Drawable
///
/// All objects that can be drawn should implement this trait.
pub trait Drawable {
    fn draw(&self, cr: &Context);
    fn fit(&mut self, frame: &Frame);
    fn scale_size(&mut self, factor: f64);
}

/// ## Plottable
///
/// All objects that can be used to plot things (e.g. represent data) should implement this.
pub trait Plottable {
    fn data_frame(&self) -> Frame;
    fn data_x_min(&self) -> f64;
    fn data_x_max(&self) -> f64;
    fn data_y_min(&self) -> f64;
    fn data_y_max(&self) -> f64;
    fn set_data_frame(&mut self, new_data_frame: Frame);
}

/// Return a list of [vec.min(), vec.max()]
pub fn vec_range(vec: &Vec<f64>) -> (f64, f64) {
    let mut max_val = vec[0];
    let mut min_val = vec[0];
    for val in vec.iter() {
        max_val = val.max(max_val);
        min_val = val.min(min_val);
    }
    (min_val, max_val)
}

/// Return a number that is the input number rounded out to the nearest number one order of
/// magnitude below itself. Rounded out means that a positive number will be ceiled and a negative
/// number will be floored.
///
/// Examples:
/// assert_eq!(round_out(1234.0), 1300);
/// assert_eq!(round_out(-1234.0), -1300);
/// assert_eq!(round_out(1.234.0), 1.3);
/// assert_eq!(round_out(-1.234.0), -1.3);
/// assert_eq!(round_out(0.001234.0), 0.0013);
/// assert_eq!(round_out(-0.001234.0), -0.0013);
pub fn round_out(number: f64, omagn: f64) -> f64 {
    let below = 10.0_f64.powi(omagn as i32 - 2);
    let round_up = number - number % below + below;
    let round_down = number - number % below;
    if number < 0.0 {
        round_down
    } else {
        round_up
    }
}

/// Return a number that is the input number rounded to the nearest number one order of
/// magnitude below itself.
///
/// Examples:
/// assert_eq!(round_out(1234.0), 1200);
/// assert_eq!(round_out(-1234.0), -1200);
/// assert_eq!(round_out(9.876.0), 9.9);
/// assert_eq!(round_out(-9.876.0), -9.9);
pub fn round_nearest(number: f64, omagn: f64) -> f64 {
    let below = 10.0_f64.powi(omagn as i32 - 2);
    let round_up = number - number % below + below;
    let round_down = number - number % below;
    if (round_down - number).abs() > (round_up - number).abs() {
        round_up
    } else {
        round_down
    }
}

pub fn round_down(number: f64, omagn: f64) -> f64 {
    let below = 10.0_f64.powi(omagn as i32 - 2);
    number - number % below
}

pub fn round_up(number: f64, omagn: f64) -> f64 {
    let below = 10.0_f64.powi(omagn as i32 - 2);
    number - number % below + below
}

/// Map a number linearly from a reference system A to another reference system B.
pub fn map_range(old_number: f64, old_min: f64, old_max: f64, new_min: f64, new_max: f64) -> f64 {
    if old_min != old_max {
        ((old_number - old_min) / (old_max - old_min) * new_max +
         (old_max - old_number) / (old_max - old_min) * new_min)
    } else {
        (old_min + old_max) / 2.0
    }
}
