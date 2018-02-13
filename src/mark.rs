//! Definition of the Mark, Tick, and GridLine structs.
//!

use cairo::Context;
use palette::Rgba;

use ::{frame, coord, text};

/// Mark
///
/// This struct is used to determine ticks and gridlines (if they are visible). It sits in the
/// background, and is used when building the canvas and the axis. Grids and ticks are ``visible
/// versions'' of a mark, in that they are used to visualise where a mark is located.
#[derive(Clone, Debug)]
pub struct Mark {
    local: coord::Coord,
    global: coord::Coord,
    label: text::Text,
}

impl Mark {
    /// Create and return a new mark
    pub fn new(coord: coord::Coord) -> Mark {
        Mark {
            local: coord,
            global: coord::Coord::new(0.0, 0.0),
            label: text::Text::new(""),
        }
    }

    /// Set local mark coordinate
    pub fn set_local(&mut self, coord: coord::Coord) {
        self.local = coord;
    }

    /// Set global mark coordinate
    pub fn set_global(&mut self, coord: coord::Coord) {
        self.global = coord;
    }

    /// Set label content
    pub fn set_label_content(&mut self, content: &str) {
        self.label.set_content(content);
    }

    /// Set label font size
    pub fn set_font_size(&mut self, val: f64) {
        self.label.set_font_size(val);
    }

    /// Set label offset in vertical and horisontal direction
    pub fn set_label_offset(&mut self, hor: f64, ver: f64) {
        self.label.set_offset(hor, ver);
    }

    /// Return the global coordinate
    pub fn global_coord(&self) -> coord::Coord {
        self.global.clone()
    }

    /// Return the first element of the global coordinate
    pub fn global_x(&self) -> f64 {
        self.global.x()
    }

    /// Return the second element of the global coordinate
    pub fn global_y(&self) -> f64 {
        self.global.y()
    }

    /// Return the label
    pub fn label(&self) -> text::Text {
        self.label.clone()
    }

    /// Return the label horisontal offset
    pub fn label_hor_offset(&self) -> f64 {
        self.label.hor_offset()
    }

    /// Return the label vertical offset
    pub fn label_ver_offset(&self) -> f64 {
        self.label.ver_offset()
    }

    /// Scale the vertical and horisontal label offset
    pub fn scale_label_offset(&mut self, factor: f64) {
        self.label.scale_offset(factor);
    }

    /// Scale the size of the label
    fn scale_size(&mut self, factor: f64) {
        self.label.scale_size(factor);
    }

    /// Fit the mark to the parent frame
    pub fn fit(&mut self, parent_frame: &frame::Frame) {
        self.global = self.local.relative_to(parent_frame);
        self.scale_size(parent_frame.diag_len() / 2f64.sqrt());
    }
}


/// ## Tick
///
/// Indicator used by an axis to serve as a reference for the displayed data
#[derive(Clone, Debug)]
pub struct Tick {
    color: Rgba,
    line_width: f64,
    length: f64,
}

impl Tick {
    /// Create and return a new Tick
    pub fn new() -> Tick {
        Tick {
            color: Rgba::new(0.0, 0.0, 0.0, 1.0),
            line_width: 0.005,
            length: 0.01,
        }
    }

    /// Set the tick color
    pub fn set_color(&mut self, color: Rgba) {
        self.color = color;
    }

    /// Set the tick color
    pub fn set_color_rgb(&mut self, red: f32, green: f32, blue: f32) {
        let red = red.max(0.0);
        let red = red.min(1.0);
        let green = green.max(0.0);
        let green = green.min(1.0);
        let blue = blue.max(0.0);
        let blue = blue.min(1.0);
        self.color = Rgba::new(red, green, blue, 1.0);
    }

    /// Set the tick color
    pub fn set_color_rgba(&mut self, red: f32, green: f32, blue: f32, alpha: f32) {
        let red = red.max(0.0);
        let red = red.min(1.0);
        let green = green.max(0.0);
        let green = green.min(1.0);
        let blue = blue.max(0.0);
        let blue = blue.min(1.0);
        let alpha = alpha.max(0.0);
        let alpha = alpha.min(1.0);
        self.color = Rgba::new(red, green, blue, alpha);
    }

    /// Scale the line width and lenght of a tick
    fn scale_size(&mut self, factor: f64) {
        self.line_width *= factor;
        self.length *= factor;
    }

    /// Fit the tick to a parent mark frame
    pub fn fit(&mut self, mark_frame: frame::Frame) {
        self.scale_size(mark_frame.diag_len() / 2f64.sqrt());
    }
}

/// ## GridLine
///
/// Indicator used by axis to serve as a reference for the displayed data
#[derive(Clone, Debug)]
pub struct GridLine {
    global_start: coord::Coord,
    global_end: coord::Coord,
    width: f64,
    color: Rgba,
}

impl GridLine {
    /// Create and return a new GridLine
    pub fn new(start: coord::Coord, end: coord::Coord) -> GridLine {
        GridLine {
            global_start: start,
            global_end: end,
            width: 0.005,
            color: Rgba::new(1.0, 1.0, 1.0, 1.0),
        }
    }

    /// Set the line width of a gridline
    pub fn set_width(&mut self, width: f64) {
        self.width = width;
    }

    /// Set the color of a gridline
    pub fn set_color(&mut self, color: Rgba) {
        self.color = color;
    }

    /// Set the color of a gridline
    pub fn set_color_rgb(&mut self, red: f32, green: f32, blue: f32) {
        let red = red.max(0.0);
        let red = red.min(1.0);
        let green = green.max(0.0);
        let green = green.min(1.0);
        let blue = blue.max(0.0);
        let blue = blue.min(1.0);
        self.color = Rgba::new(red, green, blue, 1.0);
    }

    /// Set the color of a gridline
    pub fn set_color_rgba(&mut self, red: f32, green: f32, blue: f32, alpha: f32) {
        let red = red.max(0.0);
        let red = red.min(1.0);
        let green = green.max(0.0);
        let green = green.min(1.0);
        let blue = blue.max(0.0);
        let blue = blue.min(1.0);
        let alpha = alpha.max(0.0);
        let alpha = alpha.min(1.0);
        self.color = Rgba::new(red, green, blue, alpha);
    }

    /// Scale the width of a gridline
    pub fn scale_size(&mut self, factor: f64) {
        self.width *= factor;
    }

    /// Draw the gridline
    pub fn draw(&self, cr: &Context) {
        cr.set_source_rgba(self.color.red as f64, self.color.green as f64, self.color.blue as f64,
                           self.color.alpha as f64);
        cr.set_line_width(self.width);
        cr.move_to(self.global_start.x(), self.global_start.y());
        cr.line_to(self.global_end.x(), self.global_end.y());
        cr.stroke();
    }
}
