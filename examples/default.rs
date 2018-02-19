//! Example showing the result using the default settings
//!

extern crate num_traits;
extern crate astrup;

use std::f64::consts::PI;
use num_traits::float::Float;

use astrup::{View, Figure, Plot, Chart, Line};

/// Create curves on the form *an - a* where *a* is the unit circle. The parameter t_vec is assumed
/// to hold values in [0, 1].
fn unit_circles(n: f64) -> (Vec<f64>, Vec<f64>) {
    let num_elements = 1000;
    let t_vec: Vec<f64> = (0..num_elements).map(|t| t as f64 / num_elements as f64).collect();
    let x_vec: Vec<f64> = t_vec.iter().map(|t| (2.0 * PI * n * t).cos() - (2.0 * PI * t).cos()).collect();
    let y_vec: Vec<f64> = t_vec.iter().map(|t| (2.0 * PI * n * t).sin() - (2.0 * PI * t).sin()).collect();
    (x_vec, y_vec)
}

/// Partial sums of Fourier series of a sawtooth-function
fn fourier_series(num_terms: f64) -> (Vec<f64>, Vec<f64>) {
    let num_elements = 1000;
    let x_vec: Vec<f64> = (0..num_elements).map(|x| -2.0 * PI + 4.0 * PI * x as f64 / num_elements as f64).collect();
    let mut y_vec: Vec<f64> = vec![0.0; num_elements];
    for n in 1..num_terms as usize {
        for (ind, x) in x_vec.iter().enumerate() {
            //let plus_minus = if (n + 1) / 2 == 0 { 1.0 } else { -1.0 };
            let term_val = 2.0 / PI * (-1.0).powi(n as i32 + 1) / (n as f64) * (n as f64 * x).sin();
            y_vec[ind] += term_val;
        }
    }

    (x_vec, y_vec)
}

/// Gaussian probability density function with mean = 0
fn gaussian_pdf(std: f64) -> (Vec<f64>, Vec<f64>) {
    let num_elements = 1000;
    let x_vec: Vec<f64> = (0..num_elements).map(|x| -2.0 * PI + 4.0 * PI * x as f64 / num_elements as f64).collect();
    let y_vec: Vec<f64> = x_vec.iter().map(|x| (- x * x / (2.0 * std * std)).exp() / (2.0 * PI * std * std).sqrt()).collect();

    (x_vec, y_vec)
}

fn get_data(n: f64) -> (Vec<f64>, Vec<f64>) {
    //unit_circles(n)
    //fourier_series(n)
    gaussian_pdf(n)
}

fn main() {

    let line_1 = Line::new(&get_data(0.2).0, &get_data(0.2).1);
    let line_2 = Line::new(&get_data(0.4).0, &get_data(0.4).1);
    let line_3 = Line::new(&get_data(0.7).0, &get_data(0.7).1);
    let line_4 = Line::new(&get_data(1.0).0, &get_data(1.0).1);
    let line_5 = Line::new(&get_data(1.5).0, &get_data(1.5).1);
    let line_6 = Line::new(&get_data(2.0).0, &get_data(2.0).1);
    let line_7 = Line::new(&get_data(3.0).0, &get_data(3.0).1);
    let line_8 = Line::new(&get_data(4.0).0, &get_data(4.0).1);

    let line_plot = Plot::new().add(&Chart::Line(line_1))
                               .add(&Chart::Line(line_2))
                               .add(&Chart::Line(line_3))
                               .add(&Chart::Line(line_4))
                               .add(&Chart::Line(line_5))
                               .add(&Chart::Line(line_6))
                               .add(&Chart::Line(line_7))
                               .add(&Chart::Line(line_8));

    let fig = Figure::new().add(&line_plot);

    View::new_from(fig).expect("Could not add figure to view")
                       .show();
}
