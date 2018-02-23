//! Definition of the Label struct
//!

use palette::Srgba;
use cairo::{Context, Matrix, MatrixTrait, FontWeight, FontSlant};

use ::{shape, text, coord};

/// ## Label
///
/// A struct to hold a single label. Each label is some text, centered at `(center_x, center_y)`, and
/// with a certain angle (with zero at horisontal, and increasing counterclockwise). Every label
/// have a gap in each direction. This gap is currently only used to display a frame for debug
/// reasons, but can be useful in the future for spacing.
///
/// Example for a label with angle = 0:
///
/// ```text,no_run
///
///     --------------------------------------------                 -----
///     |                                          |                  ↑
///     |                                          |                  | rel_top_gap
///     |                                          |                  ↓
///     |             ----------------             |                 -----
///     |             |text.content()|             | <- centroid.y()  ◊ text.height()
///     |             ----------------             |                 -----
///     |                                          |                  ↑
///     |                                          |                  | rel_bottom_gap
///     |                                          |                  ↓
///     --------------------------------------------                 -----
///                          ↑
///                     centroid.x()
///     |             |              |             |
///     |<----------->|<------------>|<----------->|
///     |rel_left_gap |text.length() |rel_right_gap|
/// ```
///
#[derive(Clone, Debug)]
pub struct Label {
    text: text::Text,
    local_centroid: coord::Coord,
    global_centroid: coord::Coord,
    angle: f64,
    rel_left_gap: f64,
    rel_right_gap: f64,
    rel_bottom_gap: f64,
    rel_top_gap: f64,
    border_thickness: f64,
}

impl Label {
    pub fn new() -> Label {
        Label {
            text: text::Text::new(),
            local_centroid: coord::Coord::new(),
            global_centroid: coord::Coord::new(),
            angle: 0.0,
            rel_left_gap: 0.0,
            rel_right_gap: 0.0,
            rel_bottom_gap: 0.0,
            rel_top_gap: 0.0,
            border_thickness: 0.0,
        }
    }

    #[allow(dead_code)] // TODO: When axis becomes public
    pub fn with_content(content: &str) -> Label {
        Label {
            text: text::Text::with_content(content),
            local_centroid: coord::Coord::new(),
            global_centroid: coord::Coord::new(),
            angle: 0.0,
            rel_left_gap: 0.0,
            rel_right_gap: 0.0,
            rel_bottom_gap: 0.0,
            rel_top_gap: 0.0,
            border_thickness: 0.0,
        }
    }

    pub fn with_centroid(x_coord: f64, y_coord: f64) -> Label {
        Label {
            text: text::Text::new(),
            local_centroid: coord::Coord::with_coordinates(x_coord, y_coord),
            global_centroid: coord::Coord::new(),
            angle: 0.0,
            rel_left_gap: 0.0,
            rel_right_gap: 0.0,
            rel_bottom_gap: 0.0,
            rel_top_gap: 0.0,
            border_thickness: 0.0,
        }
    }

    pub fn set_content(&mut self, content: &str) {
        self.text.set_content(content);
    }

    pub fn set_font_size(&mut self, val: f64) {
        self.text.set_font_size(val);
    }

    pub fn set_font_slant(&mut self, font_slant: FontSlant) {
        self.text.set_font_slant(font_slant);
    }

    pub fn set_font_weight(&mut self, font_weight: FontWeight) {
        self.text.set_font_weight(font_weight);
    }

    pub fn set_font_family(&mut self) {
        // TODO:
        self.text.set_font_family();
    }

    /// Set text color
    pub fn set_color_internal(&mut self, color: Srgba) {
        self.text.set_color_internal(color);
    }

    pub fn set_angle(&mut self, angle: f64) {
        self.angle = angle
    }

    pub fn set_centroid(&mut self, x_coord: f64, y_coord: f64) {
        self.local_centroid.set_coordinates(x_coord, y_coord);
    }

    pub fn set_frame_gaps(&mut self, left: f64, right: f64, bottom: f64, top: f64) {
        self.rel_left_gap = left;
        self.rel_right_gap = right;
        self.rel_bottom_gap = bottom;
        self.rel_top_gap = top;
    }

    #[allow(dead_code)] // TODO: Issue #13
    pub fn set_border_thickness(&mut self, val: f64) {
        self.border_thickness = val;
    }

    pub fn content(&self) -> String {
        self.text.content()
    }

    #[allow(dead_code)] // TODO: Issue #13
    pub fn rel_left_gap(&self) -> f64 {
        self.rel_left_gap
    }

    #[allow(dead_code)] // TODO: Issue #13
    pub fn rel_right_gap(&self) -> f64 {
        self.rel_right_gap
    }

    #[allow(dead_code)] // TODO: Issue #13
    pub fn rel_bottom_gap(&self) -> f64 {
        self.rel_bottom_gap
    }

    #[allow(dead_code)] // TODO: Issue #13
    pub fn rel_top_gap(&self) -> f64 {
        self.rel_top_gap
    }

    pub fn scale_size(&mut self, factor: f64) {
        self.text.scale_size(factor);
        self.border_thickness *= factor;
        self.rel_left_gap *= factor;
        self.rel_right_gap *= factor;
        self.rel_bottom_gap *= factor;
        self.rel_top_gap *= factor;
    }

    pub fn fit(&mut self, parent_frame: &shape::Rectangle) {
        self.global_centroid = self.local_centroid.relative_to(parent_frame);

        self.scale_size(parent_frame.diag_len())
    }

    pub fn draw(&self, cr: &Context, fig_rel_height: f64, fig_rel_width: f64) {

        cr.move_to(self.global_centroid.x(), self.global_centroid.y());

        // First, we must flip the y-axis again, to make the text be the right side up.
        // Then, we need to change the sign of the angle as we want it to increase counter
        // clockwise.
        cr.transform(Matrix::new(1.0, 0.0, 0.0, -1.0, 0.0, 0.0));
        cr.rotate(-self.angle);
        self.text.draw(cr, fig_rel_height, fig_rel_width, self.angle, self.rel_left_gap,
                       self.rel_right_gap, self.rel_bottom_gap, self.rel_top_gap,
                       self.border_thickness);
        cr.rotate(self.angle);
        cr.transform(Matrix::new(1.0, 0.0, 0.0, -1.0, 0.0, 0.0));

    }
}
