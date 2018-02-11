//! # Astrup
//!
//! A rust plotting library.
//!
//! ## Gloals:
//! - Input `Vec<Num>` and `ndarray::Array*<Num>` types
//! - It should be intuitive to build plots, but not as "easy as possible". It will probably be
//! quite verbose.
//! - Very modular.
//!
//!
//! ## Structures
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
//! | PieChart      | No        |
//! | MatrixHeatmap | No        |
//! | Image         | No        |
//!
//! ## TODO:
//!
//! ### More distant future
//! - Implement the above plot variants
//! - Fix the use of PlotType in the api, hide it somehow.
//! - Implement minor ticks and grid lines
//! - Add legend.
//! - Add possibility for colored area between curves.
//!
//! ### Close future
//! - Set axes data range explicitly. Currently it is determined to fit the range of the input
//! data.
//! - Set figure dimensions. Currently, non-square looks ugly. Non-square plots looks as expected,
//! and the figure should work in the same way.
//! - Let axis labels, tick labels and tics determine the default location of the axes
//! - Make more examples, and organise them sensibly
//! - Refactor fit and scaling functions, e.g. one fit_fig, and fit_plot for all drawables
//! - Make gridlines a part of tick struct. This is because a gridline is just an extension of a tick.
//! - Also have a flag or something to indicate major / minor ticks.
//! - Implement dotted gridlines
//! - Return self from &mut self methods such that you can write something like
//!
//!     let plot = Plot::new(...).set_color(...).set_line_width(...)
//!
//! - Name frames according to what they are. If it is natural for a struct to have a local frame,
//! name it struct.local_frame and let this allways be initialized to (0, 1)x(0, 1). Connected to
//! this local_frame is a global_frame, that holds coordinates relative to the global Figure
//! coordinate system. The global_frame is used for drawing. If the struct have data attached to
//! it, name this struct.data_frame.
//!
//! ## About
//!
//! There is a natural hierarchy to structs in this library
//!
//! ```
//! Level 0           View
//!                     |
//! Level 1           Figure
//!                     |
//! Level 2           Plot
//!                     |
//! Level 3           Canvas
//!                   /    \
//! Level 4        Chart   Axis
//!                         |
//! Level 5                Mark
//!                        /  \
//! Level 6             Tick  GridLine
//! ```
//!
//! The local frame of each struct is always set and initalized relative to the local frame of
//! the parent struct. A local frame of (0, 1)x(0, 1) is covering the entire local frame of
//! the parent struct. This local frame is used in drawing, and for this reason, this relative
//! system needs to be transformed to a global system. This is done by the fit() function.
//!
//! When the whole thing is set up, a cascade of loc2glob() functions will run from the top to the
//! bottom, starting from Figure. The purpose of loc2glob() is to transform the local frame from its
//! coordinate system relative to its parent, to a global coordinate system (relative to Figure).
//! This loc2glob() function shall only scale the local frame or data frame based on the possible
//! changes to the parent's local frame, and it is the last thing that is called before drawing.
//!
//! The local frame of Plot can be altered by user interaction.
//!
//! The local frame of Canvas is altered depending on its Charts and Axes to make space for
//! possible axis labels and legends inside its parent plot. This can be overridden by user
//! interaction. The data frame is determined by its charts: If it is an image or a matrix heatmap,
//! it should default to the data range of those charts. If it is a scatter or line plot it is
//! determined by its marks. Marks are computed such that there is a start mark and an end mark,
//! these determines the data_frame of the canvas. The marks shall cover the data, but is also set
//! to be 10^MagnOrder(data range) * {1, 2, 5}. For this reason, the data_frame determined by the
//! marks can be larger than the range of the data.
//!
//! The local frame of Chart is allways a change_range() transform of its data_frame.
//!
//! Even though an axis is 1-dimensional, we give it a local frame in order to determine the extent
//! of possible grid-lines, which run orthogonal on the axis. The major marks of all axis should
//! follow the marks of its canvas. It is possible to include minor marks. Its data allways follow
//! the data of its parent canvas.
//!
//! The mark sets locations of marks, but Tick and GridLine determine how to represent the marks.
//!
extern crate cairo;
extern crate gio;
extern crate gtk;
extern crate num;
extern crate failure;
extern crate palette;
extern crate ndarray;

pub mod utils;
pub mod view;
pub mod figure;
pub mod plot;
pub mod axis;
pub mod mark;
pub mod color;
pub mod chart;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
