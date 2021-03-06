//! Definition of the Coord struct
//!

use {shape, utils};

/// ## Coord
///
/// A simple container for an (x, y) coordinate
#[derive(Clone, Debug)]
pub struct Coord {
    x: f64,
    y: f64,
}

impl Coord {
    /// Create and return a new Coord
    pub fn new() -> Coord {
        Coord { x: 0.0, y: 0.0 }
    }

    /// Create and return a new Coord
    pub fn with_coordinates(x: f64, y: f64) -> Coord {
        Coord { x: x, y: y }
    }

    /// Update coordinates
    pub fn set_coordinates(&mut self, x: f64, y: f64) {
        self.x = x;
        self.y = y;
    }

    /// Update the first coordinate
    pub fn set_x(&mut self, x: f64) {
        self.x = x;
    }

    /// Update the second coordinate
    pub fn set_y(&mut self, y: f64) {
        self.y = y;
    }

    /// Return the first coordinate
    pub fn x(&self) -> f64 {
        self.x
    }

    /// Return the second coordinate
    pub fn y(&self) -> f64 {
        self.y
    }

    /// Return the distance between this point and the origin
    #[allow(dead_code)]
    pub fn len(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    /// Maps this coordinate, which is assumed to be relative to a unit square, to a different
    /// reference system defined by the input frame.
    pub fn relative_to(&self, frame: &shape::Rectangle) -> Coord {
        let x = utils::map_range(self.x, 0.0, 1.0, frame.left(), frame.right());
        let y = utils::map_range(self.y, 0.0, 1.0, frame.bottom(), frame.top());
        Coord::with_coordinates(x, y)
    }

    /// Returns a coordinate that is in the middle between self and other, and shifted a distance
    /// to the left of the line going from self to other.
    #[allow(dead_code)]
    pub fn perp_bisector(&self, other: &Coord, scale_factor: f64) -> Coord {
        let dx = other.x() - self.x();
        let dy = other.y() - self.y();
        let mid_x = (other.x() + self.x()) / 2.0;
        let mid_y = (other.y() + self.y()) / 2.0;
        //let norm = Coord::new(-dy, dx); A point normal on (start, end)
        //let mid = Coord::new(mid_x, mid_y); A point in the middle of (start, end)
        Coord::with_coordinates(mid_x - dy * scale_factor, mid_y + dx * scale_factor)
    }

    /// Returns a unit length direction from self to other
    pub fn unit_direction_to(&self, other: &Coord) -> Coord {
        let dx = other.x() - self.x();
        let dy = other.y() - self.y();
        let magnitude = (dx * dx + dy * dy).sqrt();
        Coord::with_coordinates(dx / magnitude, dy / magnitude)
    }

    /// Returns a unit normal vector that is perpendicular on the vector from self to other.
    pub fn perp_direction(&self, other: &Coord) -> Coord {
        let dx = other.x() - self.x();
        let dy = other.y() - self.y();
        let magnitude = (dx * dx + dy * dy).sqrt();
        Coord::with_coordinates(-dy / magnitude, dx / magnitude)
    }
}
