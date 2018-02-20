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
//! fn main() {
//!
//!     // Create data contained in ndarray
//!     let num_samples = 1000;
//!     let x_data = Array::from_iter((0..num_samples).map(|x| -5.0 + 10.0 * (x as f64) / num_samples as f64));
//!     let y_data1 = Array::from_iter((0..num_samples).map(|i| (x_data[i] - 0.0 * PI / 8.0).sin()));
//!     let y_data2 = Array::from_iter((0..num_samples).map(|i| (x_data[i] - 1.0 * PI / 8.0).sin()));
//!     let y_data3 = Array::from_iter((0..num_samples).map(|i| (x_data[i] - 2.0 * PI / 8.0).sin()));
//!     let y_data4 = Array::from_iter((0..num_samples).map(|i| (x_data[i] - 3.0 * PI / 8.0).sin()));
//!     let y_data5 = Array::from_iter((0..num_samples).map(|i| (x_data[i] - 4.0 * PI / 8.0).sin()));
//!     let y_data6 = Array::from_iter((0..num_samples).map(|i| (x_data[i] - 5.0 * PI / 8.0).sin()));
//!     let y_data7 = Array::from_iter((0..num_samples).map(|i| (x_data[i] - 6.0 * PI / 8.0).sin()));
//!     let y_data8 = Array::from_iter((0..num_samples).map(|i| (x_data[i] - 7.0 * PI / 8.0).sin()));
//!
//!     // Plot lines
//!     let line1 = Line::new(&x_data, &y_data1);
//!     let line2 = Line::new(&x_data, &y_data2);
//!     let line3 = Line::new(&x_data, &y_data3);
//!     let line4 = Line::new(&x_data, &y_data4);
//!     let line5 = Line::new(&x_data, &y_data5);
//!     let line6 = Line::new(&x_data, &y_data6);
//!     let line7 = Line::new(&x_data, &y_data7);
//!     let line8 = Line::new(&x_data, &y_data8);
//!
//!     // Add lines to a plot
//!     let line_plot = Plot::new().add(&Chart::Line(line1))
//!                                .add(&Chart::Line(line2))
//!                                .add(&Chart::Line(line3))
//!                                .add(&Chart::Line(line4))
//!                                .add(&Chart::Line(line5))
//!                                .add(&Chart::Line(line6))
//!                                .add(&Chart::Line(line7))
//!                                .add(&Chart::Line(line8))
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
//!     let normal_1 = Normal::new(-3.0, 1.0);
//!     let normal_2 = Normal::new(0.0, 2.0);
//!     let normal_3 = Normal::new(3.0, 2.0);
//!     let normal_4 = Normal::new(0.0, 1.0);
//!     let num_points = 10_000;
//!     let x_data_1: Vec<f64> = (0..num_points)
//!                              .map(|_| normal_1.ind_sample(&mut seeded_rng) as f64)
//!                              .collect();
//!     let y_data_1: Vec<f64> = (0..num_points)
//!                              .map(|_| normal_2.ind_sample(&mut seeded_rng) as f64)
//!                              .collect();
//!     let x_data_2: Vec<f64> = (0..num_points)
//!                              .map(|_| normal_3.ind_sample(&mut seeded_rng) as f64)
//!                              .collect();
//!     let y_data_2: Vec<f64> = (0..num_points)
//!                              .map(|_| normal_4.ind_sample(&mut seeded_rng) as f64)
//!                              .collect();
//!     let scatter_1 = Scatter::new(&x_data_1, &y_data_1).set_color_str("lightskyblue")
//!                                                       .set_point_size(0.002);
//!     let scatter_2 = Scatter::new(&x_data_2, &y_data_2).set_color_str("orangered")
//!                                                       .set_point_size(0.002);
//!
//!     // Add scatter points to a new plot
//!     let scatter_plot = Plot::new().set_local_frame(0.3, 1.0, 0.0, 0.49)
//!                                   .set_x_label("x")
//!                                   .set_y_label("y")
//!                                   .set_y_label_angle(0.0)
//!                                   .add(&Chart::Scatter(scatter_1))
//!                                   .add(&Chart::Scatter(scatter_2));
//!
//!     // Add the plots to a figure, and save it
//!     let fig = Figure::new().add(&line_plot)
//!                            .add(&scatter_plot)
//!                            .set_width(1000)
//!                            .set_height(800)
//!                            .set_border_thickness(0.001)
//!                           # .save("target/doc/astrup/frontpage_example.png").expect("Could not save doc frontpage_example.png")
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
//! ### Colors
//! Astrup uses the crate [palette]() to manage colors behind the scenes. In the future, there
//! should possibly be an even tighter coupling. For now, Astrup prowides the following methods for
//! defining the color of an `object`:
//!
//! #### `fn set_object_color(mut self, color_name: &str) -> Self`
//! This lets the user select one of a set of predefined default colors. The following table shows
//! the mappings between an accepted `color_name` value and a color
//!
//! | `color_name`                       | red | green | blue | alpha | hex       |                                                                                                                   |
//! | :--------------------------------- | --: | ----: | ---: | ----: | --------: | ----------------------------------------------------------------------------------------------------------------- |
//! | "blue" or "b"                      |  23 |   108 |  190 |   255 | `#176CBE` | <div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: #176CBE;"></div> |
//! | "red"  or "r"                      | 224 |    52 |   11 |   255 | `#E0340B` | <div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: #E0340B;"></div> |
//! | "green" or "g"                     |  34 |   174 |   51 |   255 | `#22AE33` | <div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: #22AE33;"></div> |
//! | "yellow" or "y"                    | 255 |   200 |   14 |   255 | `#FFC80E` | <div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: #FFC80E;"></div> |
//! | "violet" or "purple" or "v" or "p" | 136 |    60 |  177 |   255 | `#883CB1` | <div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: #883CB1;"></div> |
//! | "cyan" or "c"                      |   0 |   198 |  198 |   255 | `#00C6C6` | <div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: #00C6C6;"></div> |
//! | "orange" or "o"                    | 255 |   102 |    7 |   255 | `#FF6607` | <div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: #FF6607;"></div> |
//! | "magenta" or "m"                   | 194 |    58 |  160 |   255 | `#C23AA0` | <div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: #C23AA0;"></div> |
//! | "black" or "k"                     |   0 |     0 |    0 |   255 | `#000000` | <div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: #000000;"></div> |
//! | "gray" or "grey"                   | 127 |   127 |  127 |   255 | `#7F7F7F` | <div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: #7F7F7F;"></div> |
//! | "white" or "w"                     | 255 |   255 |  255 |   255 | `#FFFFFF` | <div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: #FFFFFF;"></div> |
//! | Another `&str` gives blue          |  23 |   108 |  190 |   255 | `#176CBE` | <div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: #176CBE;"></div> |
//!
//! #### `fn set_object_color_rgb(mut self, red: f32, green: f32, blue: f32) -> Self`
//! Gives the color specified by the amount of red, green, and blue, all which take values in
//! `[0.0, 1.0]`. This color is completely opaque (alpha is set to 1.0).
//!
//! #### `fn set_object_color_rgba(mut self, red: f32, green: f32, blue: f32, alpha: f32) -> Self`
//! As `set_object_color_rgb(...)` but where you can adjust the transparency `alpha` which takes
//! values in `[0.0, 1.0]`.
//!
//! #### `fn set_object_color_rgb_u8(mut self, red: u8, green: u8, blue: u8) -> Self`
//! As `set_object_color_rgb(...)` but where color channel intensities are specified with values in
//! `[0, 255]`.
//!
//! #### `fn set_object_color_rgba_u8(mut self, red: u8, green: u8, blue: u8, alpha: u8) -> Self`
//! As `set_object_color_rgb_u8(...)` but where you can adjust the transparency `alpha` which takes
//! values in `[0, 255]`.
//!
//! #### `fn set_object_color_rgba_u8(mut self, color_name: &str) -> Result<Self, Error>**
//! The argument is one of the color keywords in a set of [SVG
//! colors](https://www.w3.org/TR/SVG/types.html#ColorKeywords). See more information about the
//! color names and other things at the [palette
//! documentation](https://docs.rs/palette/0.3.0/palette/named/index.html).
//!
//!
extern crate cairo;
extern crate gio;
extern crate gtk;
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
