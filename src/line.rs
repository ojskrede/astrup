//! ## Line
//!
//! Module that defines the Line struct
//!

use point::Point;

enum LineType {
    Continuous,
    Dashes,
    Dots,
}

pub struct Line {
    points: Vec<Point>,
    color: [u8, u8, u8, u8], // RGBA
    thickness: f32,
    typ: LineType,
}
