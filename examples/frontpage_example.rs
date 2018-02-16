//! Example shown on the docs frontpage and on the github readme
//!

extern crate ndarray;
extern crate rand;
extern crate astrup;

use std::f64::consts::PI;

use ndarray::Array;
use rand::distributions::{IndependentSample, Normal};
use rand::{StdRng, SeedableRng};

use astrup::view::View;
use astrup::figure::Figure;
use astrup::plot::Plot;
use astrup::chart::Chart;
use astrup::chart::scatter::Scatter;
use astrup::chart::line::Line;

fn main() {

    // Create data contained in ndarray
    let x_data = Array::from_iter((0..100).map(|x| (x as f64) * 2.0 * PI / 100.0));
    let y_data1 = Array::from_iter((0..100).map(|i| x_data[i].sin()));
    let y_data2 = Array::from_iter((0..100).map(|i| (x_data[i] - PI / 2.0).sin()));

    // Plot lines
    let line1 = Line::new(&x_data, &y_data1).set_stroke_style("dotted");
    let line2 = Line::new(&x_data, &y_data2).set_color_rgba(0.9, 0.2, 0.2, 0.9);

    // Add lines to a plot
    let line_plot = Plot::new().add(&Chart::Line(line1))
                               .add(&Chart::Line(line2))
                               .set_y_min(-1.2)
                               .set_x_label("x")
                               .set_y_label("y")
                               .set_local_frame(0.0, 0.7, 0.51, 1.0);

    // Create a seedable rng so that the scatter points are equal from run to run
    let seed: Vec<usize> = vec![8, 8, 8, 8, 8, 8, 8, 8];
    let mut seeded_rng: StdRng = SeedableRng::from_seed(seed.as_slice());

    // Create scatter points
    let normal_0_1 = Normal::new(0.0, 1.0);
    let normal_0_2 = Normal::new(0.0, 2.0);
    let x_data: Vec<f64> = (0..1000)
                           .map(|_| normal_0_1.ind_sample(&mut seeded_rng) as f64)
                           .collect();
    let y_data: Vec<f64> = (0..1000)
                           .map(|_| normal_0_2.ind_sample(&mut seeded_rng) as f64)
                           .collect();
    let scatter = Scatter::new(&x_data, &y_data).set_color_rgba(0.1, 0.8, 0.3, 0.9)
                                                .set_point_size(0.005);

    // Add scatter points to a new plot
    let scatter_plot = Plot::new().set_local_frame(0.3, 1.0, 0.0, 0.49)
                                  .set_x_label("x")
                                  .set_y_label("y")
                                  .add(&Chart::Scatter(scatter));

    // Add the plots to a figure, and save it
    let fig = Figure::new().add(&line_plot)
                           .add(&scatter_plot)
                           .set_width(1000)
                           .set_height(800)
                           .set_border_thickness(0.001)
                           .save("assets/frontpage_example.png").expect("Could not save frontpage_example.png")
                           .save("target/doc/astrup/frontpage_example.png").expect("Could not save doc frontpage_example.png");

    // Display the result on screen
    View::new_from(fig).expect("Could not add figure to view")
                       .show();
}
