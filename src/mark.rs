//! Definition of the Mark, Tick, and GridLine structs.
//!

use cairo::{Context, LineCap};
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
    tick: Tick,
}

impl Mark {
    /// Create and return a new mark
    pub fn new(coord: coord::Coord) -> Mark {
        Mark {
            local: coord,
            global: coord::Coord::new(0.0, 0.0),
            label: text::Text::new(""),
            tick: Tick::new(),
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

    pub fn set_tick_color(&mut self, color: Rgba) {
        self.tick.set_color(color);
    }

    pub fn set_tick_width(&mut self, val: f64) {
        self.tick.set_width(val);
    }

    /// Set the length of the tick, in both positive and negative extent
    pub fn set_tick_length(&mut self, val: f64) {
        self.tick.set_length(val);
    }

    /// Set the positive length of the tick
    pub fn set_positive_tick_length(&mut self, val: f64) {
        self.tick.set_positive_length(val);
    }

    /// Set the negative length of the tick
    pub fn set_negative_tick_length(&mut self, val: f64) {
        self.tick.set_negative_length(val);
    }

    /// Set the direction of the tick. It is only the unsigned version of the direction that is
    /// used, that is, its angle. The extension of the tick is controlled by its positive_length
    /// and negative_length.
    pub fn set_tick_direction(&mut self, direction: &coord::Coord) {
        self.tick.set_direction(direction);
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
        self.tick.scale_size(factor);
    }

    /// Fit the mark to the parent frame
    pub fn fit(&mut self, parent_frame: &frame::Frame) {
        self.global = self.local.relative_to(parent_frame);
        self.scale_size(parent_frame.diag_len() / 2f64.sqrt());
    }

    /// Draw ticks and labels
    pub fn draw(&self, cr: &Context, fig_rel_height: f64, fig_rel_width: f64) {
        // Draw tick
        self.tick.draw(cr, fig_rel_height, fig_rel_width, self.global_x(), self.global_y());

        // Draw label
        cr.move_to(self.global_x() + self.label.hor_offset(),
                   self.global_y() + self.label.ver_offset());

        self.label.draw(cr, fig_rel_height, fig_rel_width);
    }
}


/// ## Tick
///
/// Indicator used by an axis to serve as a reference for the displayed data
#[derive(Clone, Debug)]
pub struct Tick {
    color: Rgba,
    width: f64,
    positive_length: f64, // Length from root mark in the direction of increasing x and/or y
    negative_length: f64, // Length from root mark in the direction of decreasing x and/or y
    direction: coord::Coord,
}

impl Tick {
    /// Create and return a new Tick
    pub fn new() -> Tick {
        Tick {
            color: Rgba::new(0.0, 0.0, 0.0, 1.0),
            width: 0.005,
            positive_length: 0.01,
            negative_length: 0.01,
            direction: coord::Coord::new(0.0, 0.0),
        }
    }

    /// Set the tick width
    pub fn set_width(&mut self, val: f64) {
        self.width = val;
    }

    /// Set both the positive and negative tick length
    pub fn set_length(&mut self, val: f64) {
        self.positive_length = val;
        self.negative_length = val;
    }

    /// Set the tick positive length
    pub fn set_positive_length(&mut self, val: f64) {
        self.positive_length = val;
    }

    /// Set the tick negative length
    pub fn set_negative_length(&mut self, val: f64) {
        self.negative_length = val;
    }

    /// Set the tick direction
    pub fn set_direction(&mut self, direction: &coord::Coord) {
        self.direction = direction.clone()
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

    /// Return the tick color
    pub fn color(&self) -> Rgba {
        self.color
    }

    /// Return the tick direction
    pub fn direction(&self) -> coord::Coord {
        self.direction.clone()
    }

    /// Return the tick width
    pub fn width(&self) -> f64 {
        self.width
    }

    /// Return the positive tick length
    pub fn positive_length(&self) -> f64 {
        self.positive_length
    }

    /// Return the negative tick length
    pub fn negative_length(&self) -> f64 {
        self.negative_length
    }

    /// Scale the line width and lenght of a tick
    fn scale_size(&mut self, factor: f64) {
        self.width *= factor;
        self.positive_length *= factor;
        self.negative_length *= factor;
    }

    /// Fit the tick to a parent mark frame
    pub fn fit(&mut self, mark_frame: frame::Frame) {
        self.scale_size(mark_frame.diag_len() / 2f64.sqrt());
    }

    /// Draw the tick mark
    pub fn draw(&self, cr: &Context, fig_rel_height: f64, fig_rel_width: f64, x_root: f64, y_root: f64) {
        cr.set_line_cap(LineCap::Square);
        cr.set_source_rgba(self.color.red as f64, self.color.green as f64,
                           self.color.blue as f64, self.color.alpha as f64);

        // Perpendicular on the tick direction
        let width = self.width * (self.direction.y().abs() * fig_rel_height +
                                  self.direction.x().abs() * fig_rel_width);
        cr.set_line_width(width);

        // With the tick direction
        let pos_length = self.positive_length * (self.direction.x().abs() * fig_rel_height +
                                                 self.direction.y().abs() * fig_rel_width);
        cr.move_to(x_root, y_root);
        cr.line_to(x_root + self.direction.x().abs() * pos_length,
                   y_root + self.direction.y().abs() * pos_length);
        cr.stroke();

        // Against the tick direction
        let neg_length = self.negative_length * (self.direction.x().abs() * fig_rel_height +
                                                 self.direction.y().abs() * fig_rel_width);
        cr.move_to(x_root, y_root);
        cr.line_to(x_root - self.direction.x().abs() * neg_length,
                   y_root - self.direction.y().abs() * neg_length);
        cr.stroke();
    }
}

/// ## GridLine
///
/// Indicator used by axis to serve as a reference for the displayed data
#[derive(Clone, Debug)]
pub struct GridLine {
    global_start: coord::Coord,
    global_end: coord::Coord,
    direction: coord::Coord,
    width: f64,
    color: Rgba,
}

impl GridLine {
    /// Create and return a new GridLine
    pub fn new(start: coord::Coord, end: coord::Coord) -> GridLine {
        GridLine {
            global_start: start.clone(),
            global_end: end.clone(),
            direction: start.unit_direction_to(&end),
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
    pub fn draw(&self, cr: &Context, fig_rel_height: f64, fig_rel_width: f64) {
        cr.set_source_rgba(self.color.red as f64, self.color.green as f64, self.color.blue as f64,
                           self.color.alpha as f64);

        let width = self.width * (self.direction.x().abs() * fig_rel_width + self.direction.y().abs() * fig_rel_height);
        cr.set_line_width(width);
        cr.move_to(self.global_start.x(), self.global_start.y());
        cr.line_to(self.global_end.x(), self.global_end.y());
        cr.stroke();
    }
}
