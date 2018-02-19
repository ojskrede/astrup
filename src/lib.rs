//! A rust plotting library using [gtk-rs](https://github.com/gtk-rs/gtk) as a backend. This is
//! *still* very much a small hobby project.
//!
//! GitHub repo: [https://github.com/ojskrede/astrup](https://github.com/ojskrede/astrup)
//!
//! For alternative rust plotting libraries, see e.g.
//!
//! - [rustplotlib](https://github.com/ubnt-intrepid/rustplotlib)
//! - [RustGnuplot](https://github.com/SiegeLord/RustGnuplot)
//! - [dataplotlib](https://github.com/coder543/dataplotlib)
//!
//! ## Example
//!
//! ```rust
//! extern crate ndarray;
//! extern crate rand;
//! extern crate astrup;
//!
//! use std::f64::consts::PI;
//!
//! use ndarray::Array;
//! use rand::distributions::{IndependentSample, Normal};
//! use rand::{StdRng, SeedableRng};
//!
//! use astrup::{View, Figure, Plot, Chart, Scatter, Line};
//!
//!
//! fn main() {
//!
//!     // Create data contained in ndarray
//!     let x_data = Array::from_iter((0..100).map(|x| (x as f64) * 2.0 * PI / 100.0));
//!     let y_data1 = Array::from_iter((0..100).map(|i| x_data[i].sin()));
//!     let y_data2 = Array::from_iter((0..100).map(|i| (x_data[i] - PI / 2.0).sin()));
//!
//!     // Plot lines
//!     let line1 = Line::new(&x_data, &y_data1).set_stroke_style("dotted");
//!     let line2 = Line::new(&x_data, &y_data2);
//!
//!     // Add lines to a plot
//!     let line_plot = Plot::new().add(&Chart::Line(line1))
//!                                .add(&Chart::Line(line2))
//!                                .set_y_min(-1.2)
//!                                .set_x_label("x")
//!                                .set_y_label("y")
//!                                .set_local_frame(0.0, 0.7, 0.51, 1.0);
//!
//!     // Create a seedable rng so that the scatter points are equal from run to run
//!     let seed: Vec<usize> = vec![8, 8, 8, 8, 8, 8, 8, 8];
//!     let mut seeded_rng: StdRng = SeedableRng::from_seed(seed.as_slice());
//!
//!     // Create scatter points
//!     let normal_0_1 = Normal::new(0.0, 1.0);
//!     let normal_0_2 = Normal::new(0.0, 2.0);
//!     let x_data: Vec<f64> = (0..1000)
//!                            .map(|_| normal_0_1.ind_sample(&mut seeded_rng) as f64)
//!                            .collect();
//!     let y_data: Vec<f64> = (0..1000)
//!                            .map(|_| normal_0_2.ind_sample(&mut seeded_rng) as f64)
//!                            .collect();
//!     let scatter = Scatter::new(&x_data, &y_data).set_color("green")
//!                                                 .set_point_size(0.003);
//!
//!     // Add scatter points to a new plot
//!     let scatter_plot = Plot::new().set_local_frame(0.3, 1.0, 0.0, 0.49)
//!                                   .set_x_label("x")
//!                                   .set_y_label("y")
//!                                   .set_y_label_angle(0.0)
//!                                   .add(&Chart::Scatter(scatter));
//!
//!     // Add the plots to a figure, and save it
//!     let fig = Figure::new().add(&line_plot)
//!                            .add(&scatter_plot)
//!                            .set_width(1000)
//!                            .set_height(800)
//!                            .set_border_thickness(0.001)
//!                          # .save("target/doc/astrup/frontpage_example.png").expect("Could not save doc frontpage_example.png")
//!                            .save("assets/frontpage_example.png").expect("Could not save frontpage_example.png");
//!
//!     // Display the result on screen
//!     View::new_from(fig).expect("Could not add figure to view")
//!                        .show();
//! }
//! ```
//!
//! ![Plot](frontpage_example.png)
//!
//! ## Goals:
//! - Input `Vec` and `ndarray` containers and slices
//! - It should be intuitive to build plots, but not as ``easy as possible''. It will probably be
//! quite verbose and explicit.
//! - Very configurable.
//!
//!
//! ### Drawable objects
//!
//! These methods draw whatever they specify onto its plot. It
//! should be possible to combine as many as you want of any combination.
//!
//! ## About
//!
//! ### Structure hierarchy
//!
//! There is a natural hierarchy to structs in this library
//!
//! ```text,no_run
//! Level 0           View
//!                     |
//! Level 1           Figure
//!                     |
//! Level 2           Plot
//!                     |
//! Level 3           Canvas
//!                 /   |   \
//! Level 4    Chart  Axis   GridLine
//!                     |
//! Level 5           Mark
//!                     |
//! Level 6           Tick
//! ```
//!
//! One can set the size (the location frame) of each structure, relative to its parent structure.
//! These local coordinates will be on a grid *(0, 1) х (0, 1)* relative to the size of its parent
//! structure size.
//!
//! #### [View](view/struct.View.html)
//! Used to display figures on the screen. A View can have multiple figures, but if you do not need
//! to display anything, you do not need to use it.
//!
//! #### [Figure](figure/struct.Figure.html)
//! The main structure, and can contain multiple plots. You can determine the size of the figure,
//! and also save the figure as a `.png` image.
//!
//! #### [Plot](plot/struct.Plot.html)
//! A structure that holds the visual that is to be displayed. It contains a canvas (and can
//! contain more, but this is rarely needed), and some space around the canvas, to make space for
//! titles, legends, labels, and tick-labels.
//!
//! #### [Canvas](plot/struct.Canvas.html)
//! The structure where the charts are drawn on. It can contain multiple charts and multiple axes.
//! Depending on the charts that are showed, different axes are placed at default.
//!
//! #### [Chart](chart/enum.Chart.html)
//! A struct that holds the different objects you want to use to display your data. Currently, the
//! following charts are planned, sorted by an approximate implementation priority. There are still
//! some hickups with the structure of the library that needs to be in order before the rest are
//! implemented.
//!
//! | Variant       | Supported |
//! | ------------- | --------- |
//! | Scatter       | Partially |
//! | Line          | Parially  |
//! | Histogram     | No        |
//! | MatrixHeatmap | No        |
//! | Image         | No        |
//! | FilledCurves  | No        |
//! | BoxPlot       | No        |
//! | PieChart      | No        |
//!
//! #### [Axis](axis/struct.Axis.html)
//! An axis is used to give context to the chart, as it displays a reference to the data points. It
//! is intended that a single canvas can have multiple axes, but currently, it is the safest to
//! only use the two default (ordinate and abcissa).
//!
//! #### [Mark](mark/struct.Mark.html)
//! A mark is the location on a axis where a data point reference can be placed. It is used
//! internally by the library to determine the data range of an axis, but can be visualised through
//! ticks and/or gridlines. Normally, one just want *< 10* marks on an axis (to reduce visual
//! noise), and also, it is common to let the marks be placed at data points *n х {1, 2, 5} х 10^p*
//! for some integer *n* and integer power *p*.
//!
//! **Example**:
//!
//! Data range in [-5.2345, 8.41234], gives something like the following default marks
//!
//! ```text,no_run
//!     |--------|--------|--------|--------|--------|--------|--------|--------|-->
//!   -6.0     -4.0     -2.0      0.0      2.0      4.0      6.0      8.0     10.0
//! ```
//!
//! **Note**
//! It is possible to set min and max data ranges, but they will (currently) be
//! considered as ``soft suggestions'', as the above aestetic rules will overrule the exact
//! suggestion.
//!
//! #### [Tick](mark/struct.Tick.html)
//! A small visualisation of the location of a mark, and is labeled with the data value of the
//! mark. Currently, only one type of tick is supported, but minor and major ticks will be
//! implemented sometime in the future.
//!
//! #### [GridLine](mark/struct.GridLine.html)
//! Much the same as a tick, but stretches across the whole canvas, perpendicular on the axis it
//! belongs to.
//!
extern crate cairo;
extern crate gio;
extern crate gtk;
extern crate num;
extern crate failure;
extern crate palette;
extern crate ndarray;

pub use view::View;
pub use figure::Figure;
pub use plot::Plot;
pub use chart::{Chart, Line, Scatter};

mod view;
mod figure;
mod plot;
mod canvas;
mod axis;
mod mark;
mod chart;
mod utils;
mod shape;
mod coord;
mod label;
mod text;
mod color;
