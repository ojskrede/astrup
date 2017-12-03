//! ## Utils
//!
//! Collection of various handy utilities
//!

use cairo::Context;

#[derive(Clone, Debug)]
pub struct Coord {
    x: f64,
    y: f64,
}

impl Coord {
    pub fn new(x: f64, y: f64) -> Coord {
        Coord {
            x: x,
            y: y,
        }
    }

    pub fn set(&mut self, x: f64, y: f64) {
        self.x = x;
        self.y = y;
    }

    pub fn set_x(&mut self, x: f64) {
        self.x = x;
    }

    pub fn set_y(&mut self, y: f64) {
        self.y = y;
    }

    fn scale(&mut self, factor: f64) {
        self.x = self.x * factor;
        self.x = self.x * factor;
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

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
        let mid_x = (other.x() + self.x()) / 2.0;
        let mid_y = (other.y() + self.y()) / 2.0;
        Coord::new(-dy, dx)
    }
}

#[derive(Clone, Debug)]
pub struct Text {
    content: String,
    font_size: f64,
    angle: f64,
    hor_offset: f64,
    ver_offset: f64,
}

impl Text {
    pub fn new(content: &str) -> Text {
        Text {
            content: String::from(content),
            font_size: 0.04,
            angle: 0.0,
            hor_offset: 0.0,
            ver_offset: 0.0,
        }
    }

    pub fn content(&self) -> String {
        self.content.clone()
    }

    pub fn font_size(&self) -> f64 {
        self.font_size
    }

    pub fn angle(&self) -> f64 {
        self.angle
    }

    pub fn hor_offset(&self) -> f64 {
        self.hor_offset
    }

    pub fn ver_offset(&self) -> f64 {
        self.ver_offset
    }

    pub fn set_content(&mut self, content: &str) {
        self.content = String::from(content);
    }

    pub fn set_font_size(&mut self, size: f64) {
        self.font_size = size;
    }

    pub fn set_angle(&mut self, val: f64) {
        self.angle = val;
    }

    pub fn set_offset(&mut self, hor: f64, ver: f64) {
        self.hor_offset = hor;
        self.ver_offset = ver;
    }

    // TODO: Separate vertical and horizontal scaling?
    pub fn scale_offset(&mut self, factor: f64) {
        self.hor_offset *= factor;
        self.ver_offset *= factor;
    }

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
}

impl Frame {
    pub fn new() -> Frame {
        Frame {
            left: 0.0,
            right: 1.0,
            bottom: 0.0,
            top: 1.0,
        }
    }

    pub fn from_sides(left: f64, right: f64, bottom: f64, top: f64) -> Frame {
        Frame {
            left: left,
            right: right,
            bottom: bottom,
            top: top,
        }
    }

    pub fn set(&mut self, left: f64, right: f64, bottom: f64, top: f64) {
        self.left = left;
        self.right = right;
        self.bottom = bottom;
        self.top = top;
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

/// Return a number that is the input `number` rounded up to the nearest multiplum of `nearest`
/// of order of magnitude `omagn`.
///
/// Examples:
pub fn round_up(number: f64, omagn: f64, nearest: f64) -> f64 {
    let nearest_pow = nearest * 10.0_f64.powi(omagn as i32);
    number - number % nearest_pow + nearest_pow
}

/// Return a number that is the input `number` rounded down to the nearest multiplum of `nearest`
/// of order of magnitude `omagn`.
///
/// Examples:
pub fn round_down(number: f64, omagn: f64, nearest: f64) -> f64 {
    let nearest_pow = nearest * 10.0_f64.powi(omagn as i32);
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
pub fn round_nearest(number: f64, omagn: f64, nearest: f64) -> f64 {
    let nearest_pow = nearest * 10.0_f64.powi(omagn as i32);
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
