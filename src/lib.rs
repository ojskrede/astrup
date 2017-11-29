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
//!
//! ### Major
//! - One window for each figure
//! - Implement the above plot variants
//! - Fix the use of PlotType in the api, hide it somehow.
//! - Implement minor ticks and grid lines
//! - Add legend.
//! - Add dashed lines.
//!
//! ### Minor
//! - Refactor fit and scaling functions, e.g. one fit_fig, and fit_plot for all drawables
//! - Axis label and tick label sizes seems to scale differently
//! - Use palette crate
//! - Reorganize, organize existing modules into smalle set of collections. E.g. a plottype module.
//! - Make gridlines a part of tick struct. This is because a gridline is just an extension of a tick.
//! - Also have a flag or something to indicate major / minor ticks.
//! - Only display ticks that are 10^k * {1, 2, 5}
//! - For each plot, include a canvas which is a smaller frame within the plot (determined by
//! fig_frame) that can display data. The default would be to attach axes around this frame, and a
//! grid inside it, but only labels, ticks and legends should be visible outside. That is, no data
//! or gridlines. With this, the user can add multiple axes wherever inside this draw frame.
//! Set plot_frame (in fig coords) => set canvas (in fig coords) => draw elements.

extern crate cairo;
extern crate gio;
extern crate gtk;
extern crate num;
extern crate failure;

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
