//! # Astrup
//!
//! A rust plotting library.
//!
//! ## Gloals:
//! - Input `Vec<Num>` and `ndarray Array*` types
//! - It should be intuitive to build plots, but not as "easy as possible". It will probably be
//! quite verbose.
//! - Very modular.
//!
//!
//! ## Structurs
//!
//! ### Figure
//!
//! The main image window. Can contain multiple plots.
//!
//! ### Plot
//!
//! An area defined by an x and y axis. Can contain multiple drawable objects
//!
//! ### Drawable objects
//!
//! These methods draw whatever they specify onto its plot. It
//! should be possible to combine as many as you want of any combination.
//!
//! The variants currently intended are
//!
//! | Variant       | Supported |
//! | ------------- | --------- |
//! | Scatter       | Partially |
//! | Line          | Parially  |
//! | Histogram     | No        |
//! | BoxPlot       | No        |
//! | MatrixHeatmap | No        |
//! | Image         | No        |
//!
//! ## TODO:

extern crate cairo;
extern crate gio;
extern crate gtk;
extern crate num;

pub mod utils;
pub mod figure;
pub mod plot;
pub mod axis;
pub mod scatter;
pub mod line;
pub mod point;
pub mod color;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
