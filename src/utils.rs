//! ## Utils
//!
//! Collection of various handy utilities
//!

use std::cmp::Ordering;

use cairo::Context;

/// A simple container for an (x, y) coordinate
#[derive(Clone, Debug)]
pub struct Coord {
    x: f64,
    y: f64,
}

impl Coord {
    /// Create and return a new Coord
    pub fn new(x: f64, y: f64) -> Coord {
        Coord {
            x: x,
            y: y,
        }
    }

    /// Update a coordinate
    pub fn set(&mut self, x: f64, y: f64) {
        self.x = x;
        self.y = y;
    }

    /// Update the first element of a coordinate
    pub fn set_x(&mut self, x: f64) {
        self.x = x;
    }

    /// Update the second element of a coordinate
    pub fn set_y(&mut self, y: f64) {
        self.y = y;
    }

    /// Return the first element of a coordinate
    pub fn x(&self) -> f64 {
        self.x
    }

    /// Return the second element of a coordinate
    pub fn y(&self) -> f64 {
        self.y
    }

    /// Return the distance between this coordinate and the origin
    pub fn len(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    /// Maps this coordinate, which is assumed to be relative to a unit square, to a different
    /// reference system defined by the input frame.
    pub fn relative_to(&self, frame: &Frame) -> Coord {
        let x = map_range(self.x, 0.0, 1.0, frame.left(), frame.right());
        let y = map_range(self.y, 0.0, 1.0, frame.bottom(), frame.top());
        Coord::new(x, y)
    }

    /// Returns a coordinate that is in the middle between self and other, and shifted a distance
    /// to the left of the line going from self to other.
    pub fn perp_bisector(&self, other: &Coord, scale_factor: f64) -> Coord {
        let dx = other.x() - self.x();
        let dy = other.y() - self.y();
        let mid_x = (other.x() + self.x()) / 2.0;
        let mid_y = (other.y() + self.y()) / 2.0;
        //let norm = Coord::new(-dy, dx); A point normal on (start, end)
        //let mid = Coord::new(mid_x, mid_y); A point in the middle of (start, end)
        Coord::new(mid_x - dy * scale_factor, mid_y + dx * scale_factor)
    }

    /// Returns a unit normal vector that is perpendicular on the vector from self to other.
    pub fn perp_direction(&self, other: &Coord) -> Coord {
        let dx = other.x() - self.x();
        let dy = other.y() - self.y();
        Coord::new(-dy, dx)
    }
}

/// A structure for text elements like labels and titles
#[derive(Clone, Debug)]
pub struct Text {
    content: String,
    font_size: f64,
    angle: f64,
    hor_offset: f64,
    ver_offset: f64,
}

impl Text {
    /// Create and return a new Text struct
    pub fn new(content: &str) -> Text {
        Text {
            content: String::from(content),
            font_size: 0.03,
            angle: 0.0,
            hor_offset: 0.0,
            ver_offset: 0.0,
        }
    }

    /// Return the content of the text
    pub fn content(&self) -> String {
        self.content.clone()
    }

    /// Return the text font size
    pub fn font_size(&self) -> f64 {
        self.font_size
    }

    /// Return the angle of the text
    pub fn angle(&self) -> f64 {
        self.angle
    }

    /// Return the horisontal offset of the text
    pub fn hor_offset(&self) -> f64 {
        self.hor_offset
    }

    /// Return the vertical offset of the text
    pub fn ver_offset(&self) -> f64 {
        self.ver_offset
    }

    /// Overwrite the text content
    pub fn set_content(&mut self, content: &str) {
        self.content = String::from(content);
    }

    /// Overwrite the text font size
    pub fn set_font_size(&mut self, size: f64) {
        self.font_size = size;
    }

    /// Overwrite the text angle
    pub fn set_angle(&mut self, val: f64) {
        self.angle = val;
    }

    /// Owerwrite the text offset
    pub fn set_offset(&mut self, hor: f64, ver: f64) {
        self.hor_offset = hor;
        self.ver_offset = ver;
    }

    /// Scale the vertical and horisontal text offset
    // TODO: Separate vertical and horisontal scaling?
    pub fn scale_offset(&mut self, factor: f64) {
        self.hor_offset *= factor;
        self.ver_offset *= factor;
    }

    /// Scale the font size and offsets of the text
    pub fn scale_size(&mut self, factor: f64) {
        self.font_size *= factor;
        self.scale_offset(factor);
    }
}

/// A frame defined by its boundaries.
#[derive(Clone, Debug)]
pub struct Frame {
    left: f64,
    right: f64,
    top: f64,
    bottom: f64,
    is_left_updated: bool,
    is_right_updated: bool,
    is_bottom_updated: bool,
    is_top_updated: bool,
}

impl Frame {
    /// Return a new, default frame.
    pub fn new() -> Frame {
        Frame {
            left: 0.0,
            right: 1.0,
            bottom: 0.0,
            top: 1.0,
            is_left_updated: false,
            is_right_updated: false,
            is_bottom_updated: false,
            is_top_updated: false,
        }
    }

    /// Return a new frame from given coordinate values.
    pub fn from_sides(left: f64, right: f64, bottom: f64, top: f64) -> Frame {
        Frame {
            left: left,
            right: right,
            bottom: bottom,
            top: top,
            is_left_updated: false,
            is_right_updated: false,
            is_bottom_updated: false,
            is_top_updated: false,
        }
    }

    /// Update an already created frame
    pub fn set(&mut self, left: f64, right: f64, bottom: f64, top: f64) {
        self.left = left;
        self.right = right;
        self.bottom = bottom;
        self.top = top;
        self.is_left_updated = true;
        self.is_right_updated = true;
        self.is_bottom_updated = true;
        self.is_top_updated = true;
    }

    /// Update the left horisontal frame coordinate
    pub fn set_left(&mut self, val: f64) {
        self.left = val;
        self.is_left_updated = true;
    }

    /// Update the right horisontal frame coordinate
    pub fn set_right(&mut self, val: f64) {
        self.right = val;
        self.is_right_updated = true;
    }

    /// Update the bottom vertical frame coordinate
    pub fn set_bottom(&mut self, val: f64) {
        self.bottom = val;
        self.is_bottom_updated = true;
    }

    /// Update the top vertical frame coordinate
    pub fn set_top(&mut self, val: f64) {
        self.top = val;
        self.is_top_updated = true;
    }

    /// Is the left horisontal frame coordinate updated after the default set?
    pub fn is_left_updated(&self) -> bool {
        self.is_left_updated
    }

    /// Is the right horisontal frame coordinate updated after the default set?
    pub fn is_right_updated(&self) -> bool {
        self.is_right_updated
    }

    /// Is the bottom vertical frame coordinate updated after the default set?
    pub fn is_bottom_updated(&self) -> bool {
        self.is_bottom_updated
    }

    /// Is the top vertical frame coordinate updated after the default set?
    pub fn is_top_updated(&self) -> bool {
        self.is_top_updated
    }

    /// Return the left horisontal frame coordinate
    pub fn left(&self) -> f64 {
        self.left
    }

    /// Return the right horisontal frame coordinate
    pub fn right(&self) -> f64 {
        self.right
    }

    /// Return the bottom vertical frame coordinate
    pub fn bottom(&self) -> f64 {
        self.bottom
    }

    /// Return the top vertical frame coordinate
    pub fn top(&self) -> f64 {
        self.top
    }

    /// Return the hight of the frame
    pub fn height(&self) -> f64 {
        self.top - self.bottom
    }

    /// Return the width of the frame
    pub fn width(&self) -> f64 {
        self.right - self.left
    }

    /// Return the diagonal length of the frame
    pub fn diag_len(&self) -> f64 {
        let delta_x = self.right - self.left;
        let delta_y = self.top - self.bottom;
        (delta_x * delta_x + delta_y * delta_y).sqrt()
    }

    /// Returns this frame mapped to a different `reference_frame`. This is useful when one wants
    /// to map the local frame (`self`) to a global frame (`reference_frame`).
    ///
    /// Each coordinate x on any side S (top, left, bottom, right) is then mapped as
    ///
    /// ```text,no_run
    ///   x -> ((old_max - x)*new_min + (x - old_min)*new_max) / (old_max - old_min)
    /// ```
    ///
    /// Since the local frame is bounded by (0, 1) on each side, this simplifies to
    ///
    /// ```text,no_run
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

/// Wrapper of f64 that implements Ord.
///
/// In this context it is mostly used to find min and max in data containers of f64.
///
/// Thanks to
/// https://stackoverflow.com/questions/28247990/how-to-do-a-binary-search-on-a-vec-of-floats/28248065#28248065
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
            start: Coord::new(0.0, 0.0),
            end: Coord::new(1.0, 1.0),
            width: 0.0,
        }
    }

    fn set(&mut self, start: Coord, end: Coord, width: f64) {
        self.start = start;
        self.end = end;
        self.width = width;
    }

    fn scale_size(&mut self, factor: f64) {
        self.width *= factor;
    }

    fn fit(&mut self, frame: Frame) {
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
    fn scale_size(&mut self, factor: f64);
    fn fit(&mut self, global_frame: &Frame, data_frame: &Frame);
    fn draw(&self, cr: &Context);
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
    if old_min != old_max {
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
pub fn prettify(number: f64) -> String {
    let omagn = order_of_magnitude(number);
    if omagn > 2 || omagn < -2 {
        format!("{:>.2e}", number)
    } else {
        if omagn == 2 {
            format!("{:>.0}", number)
        } else if omagn == 1 {
            format!("{:>.1}", number)
        } else if omagn == 0 {
            format!("{:>.2}", number)
        } else if omagn == -1 {
            format!("{:>.2}", number)
        } else if omagn == -2 {
            format!("{:>.3}", number)
        } else {
            String::from("Invalid order of magnitude. Should be unreachable.")
        }
    }
}
