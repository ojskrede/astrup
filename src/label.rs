//! Definition of the Label struct
//!

use ::{shape, text, coord};

/// ## Label
///
/// A struct to hold a single label. Each label is some text, centered at (center_x, center_y), and
/// with a certain angle (with zero at horisontal, and increasing counterclockwise). Every label
/// have a gap in each direction.
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
///
///
/// This box is used to determine the location of the label relative to the things it serves as a
/// label for (axis, ticks, etc).
#[derive(Clone, Debug)]
pub struct Label {
    bbox: shape::Rectangle,
    centroid: coord::Coord,
    text: text::Text,
    angle: f64,
    flip_perp: bool,
    rel_top_gap: f64,
    rel_bottom_gap: f64,
    rel_right_gap: f64,
    rel_left_gap: f64,
}
