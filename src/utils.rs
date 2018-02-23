//! Collection of various handy utilities
//!

use std::cmp::Ordering;
use std::f64;

use cairo::Context;
use palette::Srgba;

use shape;

/// Wrapper of f64 that implements Ord.
///
/// In this context it is mostly used to find min and max in data containers of f64.
///
/// Thanks to [stackoverflow](https://stackoverflow.com/questions/28247990/how-to-do-a-binary-search-on-a-vec-of-floats/28248065#28248065)
#[derive(PartialEq,PartialOrd)]
pub struct NonNan {
    val: f64,
}

impl NonNan {
    pub fn new(val: f64) -> Option<NonNan> {
        if val.is_nan() {
            None
        } else {
            Some(NonNan { val: val })
        }
    }

    pub fn val(&self) -> f64 {
        self.val
    }
}

impl Eq for NonNan {}

impl Ord for NonNan {
    fn cmp(&self, other: &NonNan) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

/*
/// ## Directional rectangle
///
/// Defined by a start coordinate, an end coordinate, and a width, relative to its parent frame.
/// The start coordinate and end coordinate forms a vector, and the width is the length of a
/// vector that is perpendicular to the (start, end) vector, pointing to the left relative to the
/// (start, end) vector. This means that a negative width will point to the right relative to
/// (start, end).
///
/// This is useful when defining axes. An axis is a directional line, but because axes can have
/// ticks and gridlines that extend normally on the axis direction, it is useful to express an axis
/// as a directional rectangle.
///
/// ```
///       |                     (s) ----------> (e)
///       | (w)                       |
///       |                           | (-w)
/// (s) ----------> (e)               |
/// ```
///
pub struct DirRect {
    start: Coord,
    end: Coord,
    width: f64,
}

impl DirRect {
    fn new() -> DirRect {
        DirRect {
            start: Coord::with_coordinates(0.0, 0.0),
            end: Coord::with_cordinates(1.0, 1.0),
            width: 0.0,
        }
    }

    fn set_coordinates(&mut self, start: Coord, end: Coord, width: f64) {
        self.start = start;
        self.end = end;
        self.width = width;
    }

    fn scale_size(&mut self, factor: f64) {
        self.width *= factor;
    }

    fn fit(&mut self, frame: Rectangle) {
        self.start = self.start.relative_to(&frame);
        self.end = self.end.relative_to(&frame);
        self.scale_size(frame.diag_len());
    }
}
*/

/// ## Drawable
///
/// All objects that can be drawn should implement this trait.
pub trait Drawable {
    fn set_color_internal(&mut self, color: Srgba);
    fn is_color_updated(&self) -> bool;
    fn scale_size(&mut self, factor: f64);
    fn fit(&mut self, global_frame: &shape::Rectangle, data_frame: &shape::Rectangle);
    fn draw(&self, cr: &Context, fig_rel_height: f64, fig_rel_width: f64);
}

/// ## Plottable
///
/// All objects that can be used to plot things (e.g. represent data) should implement this.
pub trait Plottable {
    fn data_frame(&self) -> shape::Rectangle;
    fn data_x_min(&self) -> f64;
    fn data_x_max(&self) -> f64;
    fn data_y_min(&self) -> f64;
    fn data_y_max(&self) -> f64;
    fn set_data_frame(&mut self, new_data_frame: shape::Rectangle);
}

/*
/// ## DataContainer
///
/// Containers that hold data that we are able to plot should implement this.
pub trait DataContainer<T: Num + PartialOrd> {
    fn length(&self) -> usize;
    fn num_dimensions(&self) -> usize;
    fn range(&self) -> (T, T);
}

impl<T: Num + PartialOrd> DataContainer<T> for Vec<T> {
    fn length(&self) -> usize {
        self.len()
    }

    fn num_dimensions(&self) -> usize {
        1
    }

    fn range(&self) -> (T, T) {
        let mut max_val = self[0];
        let mut min_val = self[0];
        for &val in self.iter() {
            if val > max_val { max_val = val };
            if val < min_val { min_val = val };
        }
        (min_val, max_val)
    }

}

impl<T: Num + PartialOrd> DataContainer<T> for Array1<T> {
    fn length(&self) -> usize {
        self.dim()
    }

    fn num_dimensions(&self) -> usize {
        1
    }

    fn range(&self) -> (T, T) {
        let mut max_val = self[0];
        let mut min_val = self[0];
        for &val in self.iter() {
            if val > max_val { max_val = val };
            if val < min_val { min_val = val };
        }
        (min_val, max_val)
    }
}

impl<T: Num + PartialOrd> DataContainer<T> for Array2<T> {
    fn length(&self) -> usize {
        self.dim().0
    }

    fn num_dimensions(&self) -> usize {
        2
    }

    // TODO
    fn range(&self) -> (T, T) {
        (self[[0, 0]], self[[0, 0]])
    }
}

impl<T: Num + PartialOrd> DataContainer<T> for Array3<T> {
    fn length(&self) -> usize {
        self.dim().0
    }

    fn num_dimensions(&self) -> usize {
        3
    }

    // TODO
    fn range(&self) -> (T, T) {
        (self[[0, 0, 0]], self[[0, 0, 0]])
    }
}
*/

/// Return a number that is the input `number` rounded up to the nearest multiplum of `nearest`
/// of order of magnitude `omagn`.
///
/// Examples:
#[allow(dead_code)]
pub fn round_up(number: f64, omagn: i32, nearest: f64) -> f64 {
    let nearest_pow = nearest * 10.0_f64.powi(omagn);
    number - number % nearest_pow + nearest_pow
}

/// Return a number that is the input `number` rounded down to the nearest multiplum of `nearest`
/// of order of magnitude `omagn`.
///
/// Examples:
pub fn round_down(number: f64, omagn: i32, nearest: f64) -> f64 {
    let nearest_pow = nearest * 10.0_f64.powi(omagn);
    if number >= 0.0 {
        number - number % nearest_pow
    } else {
        number - number % nearest_pow - nearest_pow
    }
}

/// Return a number that is the input `number` rounded to the nearest multiplum of `nearest` of
/// order of magnitude `omagn`.
///
/// Examples:
pub fn round_nearest(number: f64, omagn: i32, nearest: f64) -> f64 {
    let nearest_pow = nearest * 10.0_f64.powi(omagn);
    let round_up = number - number % nearest_pow + nearest_pow;
    let round_down = number - number % nearest_pow;
    if (round_down - number).abs() > (round_up - number).abs() {
        round_up
    } else {
        round_down
    }
}

/// Map a number linearly from a reference system A to another reference system B.
pub fn map_range(old_number: f64, old_min: f64, old_max: f64, new_min: f64, new_max: f64) -> f64 {
    if (old_min - old_max).abs() > f64::EPSILON {
        ((old_number - old_min) / (old_max - old_min) * new_max +
         (old_max - old_number) / (old_max - old_min) * new_min)
    } else {
        (old_min + old_max) / 2.0
    }
}

/// Find the order of magnitude of a number.
///
/// For a number **n > 0**, we define the order of magnitude **p** to be the integer such that
///
/// ```text,no_run
/// n \in [10^p, 10^{p+1})
/// ```
///
/// If *n = 0*, we define *p = 0*.
///
/// Examples:
///
/// |         n |   p |
/// | ---------:| --: |
/// | 123456.78 |   5 |
/// |  12345.67 |   4 |
/// |   1234.56 |   3 |
/// |    123.45 |   2 |
/// |     12.34 |   1 |
/// |      1.23 |   0 |
/// |         0 |   0 |
/// |    0.1234 |  -1 |
/// |    0.0123 |  -2 |
/// |    0.0012 |  -3 |
/// |    0.0001 |  -4 |
///
pub fn order_of_magnitude(number: f64) -> i32 {
    // TODO: Perhaps make the "near zero" estimate dependent on the order of magnitude of the other
    // tick values. The current hard-coded version works for most cases, but not for very small
    // ones.
    if number < 1.0e-10 && number > -1.0e-10 {
        0
    } else {
        number.abs().log10().floor() as i32
    }
}


/// Format a tick label w.r.t. space and clarity
///
/// Examples where n is the number, p is the order of magnitude and m is the intended output
///
///  |         n |   p |     m   |
///  | --------: | --: | ------: |
///  | 123456.78 |   5 |  1.23e5 |
///  |  12345.67 |   4 |  1.23e4 |
///  |   1234.56 |   3 |  1.23e3 |
///  |    123.45 |   2 |     123 |
///  |     12.34 |   1 |    12.3 |
///  |      1.23 |   0 |    1.23 |
///  |      0.00 |   0 |    0.00 |
///  |    0.1234 |  -1 |    0.12 |
///  |    0.0123 |  -2 |   0.012 |
///  |   0.00123 |  -3 | 1.23e-3 |
///  |  0.000123 |  -4 | 1.23e-4 |
///
/// This means that the with of a tick label (for *|p| < 10*) will be at most 8 characters
/// (e.g. *-1.23e-4*)
///
/// Note that this takes the order of magnitude from outside.
pub fn prettify(number: f64) -> String {
    let omagn = order_of_magnitude(number);
    if omagn > 2 || omagn < -2 {
        format!("{:>.2e}", number)
    } else if omagn == 2 {
        format!("{:>.0}", number)
    } else if omagn == 1 {
        format!("{:>.1}", number)
    } else if omagn == 0 || omagn == -1 {
        format!("{:>.2}", number)
    } else if omagn == -2 {
        format!("{:>.3}", number)
    } else {
        String::from("Invalid order of magnitude. Should be unreachable.")
    }
}
