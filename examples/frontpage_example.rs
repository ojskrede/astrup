//! Example shown on the docs frontpage and on the github readme
//!

extern crate astrup;
extern crate ndarray;
extern crate rand;

use std::f64::consts::PI;

use ndarray::Array;
use rand::distributions::{IndependentSample, Normal};
use rand::{SeedableRng, StdRng};

use astrup::{Chart, Figure, HtmlColor, Line, Plot, Scatter, View};

fn main() {
    // Create data and collect them in an ndarray array
    let num_samples = 1000;
    let x_data =
        Array::from_iter((0..num_samples).map(|x| -5.0 + 10.0 * (x as f64) / num_samples as f64));

    // Construct a line plot
    let mut line_plot = Plot::new();
    line_plot.set_y_min(-1.2)
             .set_local_frame(0.0, 0.7, 0.51, 1.0);

    // Create multiple lines and append them to the line plot
    for shift in 0..8 {
        let y_data = Array::from_iter((0..num_samples).map(|i| (x_data[i] - shift as f64 * PI / 8.0).sin()));

        let line1 = Line::new(&x_data, &y_data);
        line_plot.add_chart(&Chart::Line(line1));
    }

    // Create a seedable rng so that the scatter points are equal from run to run
    let seed: Vec<usize> = vec![8, 8, 8, 8, 8, 8, 8, 8];
    let mut seeded_rng: StdRng = SeedableRng::from_seed(seed.as_slice());

    // Create scatter points to be plotted
    let normal_1 = Normal::new(-3.0, 1.0);
    let normal_2 = Normal::new(0.0, 2.0);
    let normal_3 = Normal::new(3.0, 2.0);
    let normal_4 = Normal::new(0.0, 1.0);
    let num_points = 10_000;
    let x_data_1: Vec<f64> = (0..num_points)
        .map(|_| normal_1.ind_sample(&mut seeded_rng) as f64)
        .collect();
    let y_data_1: Vec<f64> = (0..num_points)
        .map(|_| normal_2.ind_sample(&mut seeded_rng) as f64)
        .collect();
    let x_data_2: Vec<f64> = (0..num_points)
        .map(|_| normal_3.ind_sample(&mut seeded_rng) as f64)
        .collect();
    let y_data_2: Vec<f64> = (0..num_points)
        .map(|_| normal_4.ind_sample(&mut seeded_rng) as f64)
        .collect();
    let mut scatter_1 = Scatter::new(&x_data_1, &y_data_1);
    scatter_1.set_color_html(&HtmlColor::Lightskyblue)
             .set_point_size(0.002);
    let mut scatter_2 = Scatter::new(&x_data_2, &y_data_2);
    scatter_2.set_color_rgba_u8(255, 69, 0, 200)
             .set_point_size(0.002);

    // Create a scatter plot
    let mut scatter_plot = Plot::new();
    scatter_plot.set_local_frame(0.3, 1.0, 0.0, 0.49)
                .set_x_label("x")
                .set_y_label("y")
                .set_y_label_angle(0.0)
                .set_title("Scatter plot")
                .add_chart(&Chart::Scatter(scatter_1))
                .add_chart(&Chart::Scatter(scatter_2));

    // Add the plots to a figure, and save it
    let mut fig = Figure::new();
    fig.add_plot(&line_plot)
       .add_plot(&scatter_plot)
       .set_width(1000)
       .set_height(800)
       .set_border_thickness(0.001)
       .save("target/doc/astrup/frontpage_example.png")
       .expect("Could not save doc frontpage_example.png")
       .save("assets/frontpage_example.png")
       .expect("Could not save frontpage_example.png");

    // Display the result on screen
    View::with_figure(fig)
        .expect("Could not add figure to view")
        .show();
}
