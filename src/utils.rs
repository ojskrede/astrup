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

    pub fn from_corners(left: f64, right: f64, top: f64, bottom: f64) -> Frame {
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
        self.bottom = val;
    }

    pub fn set_left(&self, val: f64) {
        self.left = val;
    }

    pub fn set_right(&self, val: f64) {
        self.right = val;
    }

    pub fn set_top(&self, val: f64) {
        self.top = val;
    }

    pub fn set_bottom(&self, val: f64) {
        self.bottom = val;
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


/// ## Compute marks
///
/// Marks are used by axis ticks, and axis gridlines, to determine their location.
///
/// This method will return a list of evenly spaced marks according to the following method.
/// This assumes that the data range is known, and that know how many marks we want. The latter
/// is determined by a variable, and will be used more of a guide than as the actual number of
/// marks we get in the end.
///
/// ### Method
///
/// 1. Find the orider of magnitude of the difference in the data range. Call this omagn.
/// 2a. Let min_point be min(data) rounded down to nearest 10^(omagn - 2).
/// 2b. Let max_point be max(data) rounded up to nearest 10^(omagn - 2).
/// 3. mark_distance = (max_point - min_point) / num_labels rounded to nearest 10^(omagn - 2)
/// 4. Then, let mark_k = min_point + k*mark_distance, for k = 0 until mark_k is greater or
///    equal to max(data).
/// 5. Transform between labels in the data framework (the above) and positions in the drawing
///    framework using the data range and axis frame.
///
///
/// TODO:
///  - Add a feature that only accepts marks at locations 10^k * {1, 2, 5} for integer k.
///  - Compute the martk data location based on largest data frame. Then update the axis' data
///  range to be cover (be the same as) its mark data range. Then adjust the plot location of
///  its marks, data, gridlines, etc. Currently the axis range is determined by the range of
///  the data, and not the range of its marks. Also, the user should be able to set the data
///  range, this should then determine the mark range, which in turn should determine the axis
///  range.
fn compute_marks(&self, ref_num_marks: usize, fig_min: f64, fig_max: f64, data_min: f64, data_max: f64) -> Vec<Mark> {
    let data_diff = data_max - data_min;
    let omagn = data_diff.log10().ceil();
    let actual_min_point = round_out(data_min, omagn);
    let ref_max_point = round_out(data_max, omagn);
    let mark_distance = round_nearest((ref_max_point - actual_min_point) / ref_num_marks as f64, omagn);

    let mut data_loc_k = actual_min_point;
    let mut marks = Vec::<Mark>::new();
    let mut add_next = true;
    while add_next {
        if data_loc_k > ref_max_point {
            add_next = false;
        }

        let fig_loc_k = change_domain(data_loc_k, data_min, data_max, fig_min, fig_max);
        let mark_k = Mark::new();
        mark_k.set(fig_loc_k, data_loc_k);

        marks.push(mark_k);
        data_loc_k += mark_distance;
    }
    marks
}
