//! Definition of the Mark, Tick, and GridLine structs.
//!

use cairo::{Context, LineCap, FontWeight, FontSlant};
use palette::Srgba;

use ::{shape, coord, label, color};

/// Mark
///
/// This struct is used to determine ticks and gridlines (if they are visible). It sits in the
/// background, and is used when building the canvas and the axis. Grids and ticks are ``visible
/// versions'' of a mark, in that they are used to visualise where a mark is located.
#[derive(Clone, Debug)]
pub struct Mark {
    local: coord::Coord,
    global: coord::Coord,
    label: label::Label,
    label_offset: f64,
    tick: Tick,
}

impl Mark {
    /// Create and return a new mark
    #[allow(dead_code)]
    pub fn new() -> Mark {
        let mut label = label::Label::new();
        label.set_color_internal(color::CustomColor::TickLabel.as_srgba());
        Mark {
            local: coord::Coord::new(),
            global: coord::Coord::new(),
            label: label,
            label_offset: 0.0,
            tick: Tick::new(),
        }
    }

    /// Create and return a new mark
    pub fn with_location(coord: coord::Coord) -> Mark {
        let mut label = label::Label::new();
        label.set_color_internal(color::CustomColor::TickLabel.as_srgba());
        Mark {
            local: coord,
            global: coord::Coord::new(),
            label: label,
            label_offset: 0.0,
            tick: Tick::new(),
        }
    }

    /// Set local mark coordinate
    #[allow(dead_code)]
    pub fn set_local(&mut self, coord: coord::Coord) {
        self.local = coord;
    }

    /// Set global mark coordinate
    #[allow(dead_code)]
    pub fn set_global(&mut self, coord: coord::Coord) {
        self.global = coord;
    }

    /// Set label content
    pub fn set_label_content(&mut self, content: &str) {
        self.label.set_content(content);
    }

    /// Set label offset, relative to a local frame. It specifies where to put the centroid of the
    /// tick label. A positive offset puts it along the tick in the positive x / y direction,
    /// relative to the mark position, and vice versa for a negative offset.
    pub fn set_label_offset(&mut self, val: f64) {
        self.label_offset = val;
    }

    /// Set label font size
    pub fn set_font_size(&mut self, val: f64) {
        self.label.set_font_size(val);
    }

    pub fn set_font_slant(&mut self, font_slant: FontSlant) {
        self.label.set_font_slant(font_slant);
    }

    pub fn set_font_weight(&mut self, font_weight: FontWeight) {
        self.label.set_font_weight(font_weight);
    }

    pub fn set_font_family(&mut self) {
        // TODO:
        self.label.set_font_family();
    }

    /// Set the color of the tick label
    pub fn set_label_color_internal(&mut self, color: Srgba) {
        self.label.set_color_internal(color);
    }

    /// Set gaps around the label frame
    pub fn set_label_frame_gaps(&mut self, left: f64, right: f64, bottom: f64, top: f64) {
        self.label.set_frame_gaps(left, right, bottom, top);
    }

    /// Set the centroid of the associated tick label.
    pub fn set_label_centroid(&mut self, x_coord: f64, y_coord: f64) {
        self.label.set_centroid(x_coord, y_coord);
    }

    /// Set the tick color
    pub fn set_tick_color_internal(&mut self, color: Srgba) {
        self.tick.set_color_internal(color);
    }

    #[allow(dead_code)] // TODO: When axis becomes public
    pub fn set_tick_width(&mut self, val: f64) {
        self.tick.set_width(val);
    }

    /// Set the length of the tick, in both positive and negative extent
    #[allow(dead_code)] // TODO: When axis becomes public
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

    /// Return the local coordinate
    #[allow(dead_code)] // TODO: When axis becomes public
    pub fn local_coord(&self) -> coord::Coord {
        self.local.clone()
    }

    /// Return the first element of the local coordinate
    pub fn local_x(&self) -> f64 {
        self.local.x()
    }

    /// Return the second element of the local coordinate
    pub fn local_y(&self) -> f64 {
        self.local.y()
    }

    /// Return the global coordinate
    pub fn global_coord(&self) -> coord::Coord {
        self.global.clone()
    }

    /// Return the first element of the global coordinate
    #[allow(dead_code)] // TODO: When axis becomes public
    pub fn global_x(&self) -> f64 {
        self.global.x()
    }

    /// Return the second element of the global coordinate
    #[allow(dead_code)] // TODO: When axis becomes public
    pub fn global_y(&self) -> f64 {
        self.global.y()
    }

    /// Return the label
    #[allow(dead_code)] // TODO: Issue #13
    pub fn label(&self) -> label::Label {
        self.label.clone()
    }

    /// Return the label
    pub fn label_offset(&self) -> f64 {
        self.label_offset
    }

    /// Return the gap to the left of the label
    #[allow(dead_code)] // TODO: Issue #13
    pub fn label_left_gap(&self) -> f64 {
        self.label.rel_left_gap()
    }

    /// Return the gap to the right of the label
    #[allow(dead_code)] // TODO: Issue #13
    pub fn label_right_gap(&self) -> f64 {
        self.label.rel_right_gap()
    }

    /// Return the gap below the label
    #[allow(dead_code)] // TODO: Issue #13
    pub fn label_bottom_gap(&self) -> f64 {
        self.label.rel_bottom_gap()
    }

    /// Return the gap above the label
    #[allow(dead_code)] // TODO: Issue #13
    pub fn label_top_gap(&self) -> f64 {
        self.label.rel_top_gap()
    }

    /// Scale the size of the label
    fn scale_size(&mut self, factor: f64) {
        self.tick.scale_size(factor);
    }

    /// Fit the mark to the parent frame
    pub fn fit(&mut self, parent_frame: &shape::Rectangle) {
        self.global = self.local.relative_to(parent_frame);
        self.scale_size(parent_frame.diag_len());
        self.label.fit(parent_frame);
        self.tick.fit(parent_frame);
    }

    /// Draw ticks and labels
    pub fn draw(&self, cr: &Context, fig_rel_height: f64, fig_rel_width: f64) {
        self.tick.draw(cr, fig_rel_height, fig_rel_width, self.global.x(), self.global.y());

        self.label.draw(cr, fig_rel_height, fig_rel_width);
    }
}


/// ## Tick
///
/// Indicator used by an axis to serve as a reference for the displayed data. This can extend to
/// both sides of the mark it is associated with.
#[derive(Clone, Debug)]
pub struct Tick {
    color: color::Color,
    width: f64,
    positive_length: f64, // Length from root mark in the direction of increasing x and/or y
    negative_length: f64, // Length from root mark in the direction of decreasing x and/or y
    direction: coord::Coord,
}

impl Tick {
    /// Create and return a new Tick
    pub fn new() -> Tick {
        Tick {
            color: color::Color::with_custom(color::CustomColor::Tick),
            width: 0.0025,
            positive_length: 0.005,
            negative_length: 0.005,
            direction: coord::Coord::new(),
        }
    }

    /// Set the tick width
    #[allow(dead_code)] // TODO: When axis becomes public
    pub fn set_width(&mut self, val: f64) {
        self.width = val;
    }

    /// Set both the positive and negative tick length
    #[allow(dead_code)] // TODO: When axis becomes public
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
    pub fn set_color_internal(&mut self, color: Srgba) {
        self.color.set_color(color);
    }

    /// Return the tick color
    #[allow(dead_code)] // TODO: Implement tick coloring
    pub fn color(&self) -> Srgba {
        self.color.as_srgba()
    }

    /// Return the tick direction
    #[allow(dead_code)] // TODO: Why
    pub fn direction(&self) -> coord::Coord {
        self.direction.clone()
    }

    /// Return the tick width
    #[allow(dead_code)] // TODO: Why
    pub fn width(&self) -> f64 {
        self.width
    }

    /// Return the positive tick length
    #[allow(dead_code)] // TODO: Why
    pub fn positive_length(&self) -> f64 {
        self.positive_length
    }

    /// Return the negative tick length
    #[allow(dead_code)] // TODO: Why
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
    pub fn fit(&mut self, mark_frame: &shape::Rectangle) {
        self.scale_size(mark_frame.diag_len());
    }

    /// Draw the tick mark
    pub fn draw(&self, cr: &Context, fig_rel_height: f64, fig_rel_width: f64, x_root: f64, y_root: f64) {
        cr.move_to(x_root, y_root);
        cr.set_line_cap(LineCap::Square);
        let tick_color = self.color.as_srgba();
        cr.set_source_rgba(tick_color.red as f64, tick_color.green as f64,
                           tick_color.blue as f64, tick_color.alpha as f64);

        // Perpendicular on the tick direction
        let width = self.width * (self.direction.y().abs() * fig_rel_height +
                                  self.direction.x().abs() * fig_rel_width);
        cr.set_line_width(width);

        // With the tick direction
        let pos_length = self.positive_length * (self.direction.x().abs() * fig_rel_height +
                                                 self.direction.y().abs() * fig_rel_width);
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
    color: color::Color,
}

impl GridLine {
    /// Create and return a new GridLine
    #[allow(dead_code)]
    pub fn new() -> GridLine {
        GridLine {
            global_start: coord::Coord::new(),
            global_end: coord::Coord::new(),
            direction: coord::Coord::new(),
            width: 0.001,
            color: color::Color::with_custom(color::CustomColor::GridLine),
        }
    }

    /// Create and return a new GridLine
    pub fn with_boundaries(x_start: f64, y_start: f64, x_end: f64, y_end: f64) -> GridLine {
        let start = coord::Coord::with_coordinates(x_start, y_start);
        let end = coord::Coord::with_coordinates(x_end, y_end);
        GridLine {
            direction: start.unit_direction_to(&end),
            global_start: start,
            global_end: end,
            width: 0.001,
            color: color::Color::with_custom(color::CustomColor::GridLine),
        }
    }

    /// Set the line width of a gridline
    pub fn set_width(&mut self, width: f64) {
        self.width = width;
    }

    /// Set grid line color
    pub fn set_color_internal(&mut self, color: Srgba) {
        self.color.set_color(color);
    }

    /// Scale the width of a gridline
    pub fn scale_size(&mut self, factor: f64) {
        self.width *= factor;
    }

    /// Draw the grid line
    pub fn draw(&self, cr: &Context, fig_rel_height: f64, fig_rel_width: f64) {
        let line_color = self.color.as_srgba();
        cr.set_source_rgba(line_color.red as f64, line_color.green as f64, line_color.blue as f64,
                           line_color.alpha as f64);

        let width = self.width * (self.direction.x().abs() * fig_rel_width +
                                  self.direction.y().abs() * fig_rel_height);
        cr.set_line_width(width);
        cr.move_to(self.global_start.x(), self.global_start.y());
        cr.line_to(self.global_end.x(), self.global_end.y());
        cr.stroke();
    }
}
