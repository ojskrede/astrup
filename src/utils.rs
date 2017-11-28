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

#[derive(Clone, Debug)]
pub struct Frame {
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
}

impl Frame {
    pub fn new(x_min: f64, x_max: f64, y_min: f64, y_max: f64) -> Frame {
        Frame {
            x_min: x_min,
            x_max: x_max,
            y_min: y_min,
            y_max: y_max,
        }
    }

    pub fn x_min(&self) -> f64 {
        self.x_min
    }

    pub fn x_max(&self) -> f64 {
        self.x_max
    }

    pub fn y_min(&self) -> f64 {
        self.y_min
    }

    pub fn y_max(&self) -> f64 {
        self.y_max
    }

    pub fn set_x_min(&mut self, val: f64) {
        self.x_min = val;
    }

    pub fn set_x_max(&mut self, val: f64) {
        self.x_max = val;
    }

    pub fn set_y_min(&mut self, val: f64) {
        self.y_min = val;
    }

    pub fn set_y_max(&mut self, val: f64) {
        self.y_max = val;
    }
}

/// ## Drawable
///
/// All objects that can be drawn should implement this trait.
pub trait Drawable {
    fn draw_fn(&self, cr: &Context);
    fn fit(&mut self, frame: &Frame);
    fn data_frame(&self) -> Frame;
    fn set_data_frame(&mut self, new_data_frame: Frame);
    fn data_x_min(&self) -> f64;
    fn data_x_max(&self) -> f64;
    fn data_y_min(&self) -> f64;
    fn data_y_max(&self) -> f64;
    fn scale_size(&mut self, factor: f64);
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

/// Map a number in one reference system to another
pub fn change_domain(old_number: f64, old_min: f64, old_max: f64,
                     new_min: f64, new_max: f64) -> f64 {
    if old_min != old_max {
        ((old_number - old_min) / (old_max - old_min) * new_max +
         (old_max - old_number) / (old_max - old_min) * new_min)
    } else {
        (old_min + old_max) / 2.0
    }
}
