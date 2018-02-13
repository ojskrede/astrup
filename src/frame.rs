//! Definition of the Frame struct

use cairo::Context;
use palette::Rgba;

/// ## Frame
///
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
    display_border: bool,
    color: Rgba,
    thickness: f64
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
            display_border: false,
            color: Rgba::new(0.0, 0.0, 0.0, 1.0),
            thickness: 0.0,
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
            display_border: false,
            color: Rgba::new(0.0, 0.0, 0.0, 1.0),
            thickness: 0.0,
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

    /// Whether or not to display the border (equivalent to thickness = 0.0)
    pub fn display_border(&mut self, val: bool) {
        self.display_border = val;
    }

    /// Set the color of the frame border
    pub fn set_color(&mut self, color: Rgba) {
        self.color = color;
    }

    /// Set the line width of the frame border
    pub fn set_thickness(&mut self, val: f64) {
        self.thickness = val;
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

    /// Scale the frame border thickness with factor
    pub fn scale_size(&mut self, factor: f64) {
        self.thickness *= factor;
    }

    /// Draw a border around the frame
    pub fn draw(&self, cr: &Context) {
        if self.display_border {
            cr.set_source_rgba(self.color.red as f64, self.color.green as f64,
                               self.color.blue as f64, self.color.alpha as f64);
            cr.set_line_width(self.thickness);
            cr.rectangle(self.left(), self.bottom(), self.width(), self.height());
            cr.stroke();
        }
    }
}
