//! Definition of geometrical shapes
//!

use failure::Error;
use cairo::Context;

use ::{color, coord};

/// ## Rectangle
///
/// A frame defined by its boundaries.
#[derive(Clone, Debug)]
pub struct Rectangle {
    left: f64,
    right: f64,
    top: f64,
    bottom: f64,
    is_left_updated: bool,
    is_right_updated: bool,
    is_bottom_updated: bool,
    is_top_updated: bool,
    display_border: bool,
    color: color::Color,
    border_thickness: f64
}

impl Rectangle {
    /// Return a new, default frame.
    pub fn new() -> Rectangle {
        Rectangle {
            left: 0.0,
            right: 1.0,
            bottom: 0.0,
            top: 1.0,
            is_left_updated: false,
            is_right_updated: false,
            is_bottom_updated: false,
            is_top_updated: false,
            display_border: false,
            color: color::Color::new(),
            border_thickness: 0.0,
        }
    }

    /// Return a new frame from given coordinate values.
    pub fn new_from(left: f64, right: f64, bottom: f64, top: f64) -> Rectangle {
        Rectangle {
            left: left,
            right: right,
            bottom: bottom,
            top: top,
            is_left_updated: false,
            is_right_updated: false,
            is_bottom_updated: false,
            is_top_updated: false,
            display_border: false,
            color: color::Color::new(),
            border_thickness: 0.0,
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

    /// Return the line width of the border
    pub fn border_thickness(&self) -> f64 {
        self.border_thickness
    }

    /// Whether or not to display the border (equivalent to border_thickness = 0.0)
    pub fn display_border(&mut self, val: bool) {
        self.display_border = val;
    }

    /// Set border color
    pub fn set_color(&mut self, color_name: &str) {
        self.color.set_color_default(color_name);
    }

    /// Set border color
    pub fn set_color_rgb(&mut self, red: f32, green: f32, blue: f32) {
        self.color.set_color_rgb(red, green, blue);
    }

    /// Set border color
    pub fn set_color_rgba(&mut self, red: f32, green: f32, blue: f32, alpha: f32) {
        self.color.set_color_rgba(red, green, blue, alpha);
    }

    /// Set border color
    pub fn set_color_rgb_u8(&mut self, red: u8, green: u8, blue: u8) {
        self.color.set_color_rgb_u8(red, green, blue);
    }

    /// Set border color
    pub fn set_color_rgba_u8(&mut self, red: u8, green: u8, blue: u8, alpha: u8) {
        self.color.set_color_rgba_u8(red, green, blue, alpha);
    }

    /// Set border color
    pub fn set_color_str(&mut self, color_name: &str) -> Result<(), Error> {
        self.color.set_color_str(color_name)?;
        Ok(())
    }

    /// Set the line width of the frame border
    pub fn set_border_thickness(&mut self, val: f64) {
        self.border_thickness = val;
        if val > 0.0 {
            self.display_border = true;
        }
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
    pub fn relative_to(&self, reference: &Rectangle) -> Rectangle {
        let new_left = reference.left() + reference.width() * self.left();
        let new_right = reference.left() + reference.width() * self.right();
        let new_bottom = reference.bottom() + reference.height() * self.bottom();
        let new_top = reference.bottom() + reference.height() * self.top();
        //Rectangle::from_sides(new_left, new_right, new_bottom, new_top)

        Rectangle {
            left: new_left,
            right: new_right,
            bottom: new_bottom,
            top: new_top,
            is_left_updated: false,
            is_right_updated: false,
            is_bottom_updated: false,
            is_top_updated: false,
            display_border: self.display_border,
            color: self.color.clone(),
            border_thickness: self.border_thickness,
        }
    }

    /// Scale the frame border thickness with factor
    pub fn scale_size(&mut self, factor: f64) {
        self.border_thickness *= factor;
    }

    /// Draw a border around the frame
    pub fn draw(&self, cr: &Context, fig_rel_height: f64, fig_rel_width: f64) {
        if self.display_border {
            let border_color = self.color.as_srgba();
            cr.set_source_rgba(border_color.red as f64, border_color.green as f64,
                               border_color.blue as f64, border_color.alpha as f64);
            // Move to bottom left corner
            cr.move_to(self.left, self.bottom);
            // Bottom left to bottom right
            cr.set_line_width(self.border_thickness * fig_rel_width);
            cr.rel_line_to(self.width(), 0.0);
            // Bottom right to top right
            cr.set_line_width(self.border_thickness * fig_rel_height);
            cr.rel_line_to(0.0, self.height());
            // Top right to top left
            cr.set_line_width(self.border_thickness * fig_rel_width);
            cr.rel_line_to(-self.width(), 0.0);
            // Top left to bottom left
            cr.set_line_width(self.border_thickness * fig_rel_height);
            cr.close_path();
            //cr.rectangle(self.left(), self.bottom(), self.width(), self.height());
            cr.stroke();
            cr.move_to(self.left, self.bottom); // Needed because of close_path() (See cairo-rs docs)
        }
    }
}

/// ## Quadrilateral
///
/// A quadrilateral defined by its vertices
#[derive(Clone, Debug)]
pub struct Quadrilateral {
    vertex_a: coord::Coord,
    vertex_b: coord::Coord,
    vertex_c: coord::Coord,
    vertex_d: coord::Coord,
    centroid: coord::Coord,
}

impl Quadrilateral {
    pub fn new() -> Quadrilateral {
        Quadrilateral {
            vertex_a: coord::Coord::new_from(0.0, 0.0),
            vertex_b: coord::Coord::new_from(1.0, 0.0),
            vertex_c: coord::Coord::new_from(1.0, 1.0),
            vertex_d: coord::Coord::new_from(0.0, 1.0),
            centroid: coord::Coord::new_from(0.5, 0.5),
        }
    }
}
