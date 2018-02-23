//! Example of how to build multiple figures
//!

extern crate ndarray;
extern crate astrup;

use std::f64::consts::PI;

use ndarray::Array;

use astrup::{View, Figure, Plot, Chart, Line, StrokeStyle};

/// Produce a collatz sequence
///
/// [Collatz conjecture](https://en.wikipedia.org/wiki/Collatz_conjecture)
///
/// An init value of 837799 produces longest collatz sequence of init values < 1 000 000
///
fn collatz(init_val: u64) -> Vec<f64> {
    let mut progression: u64 = init_val;
    let mut values: Vec<f64> = vec![init_val as f64];

    while progression != 1 {
        if progression % 2 == 0 {
            progression /= 2;
        }
        else {
            progression = progression*3 + 1;
        }
        values.push(progression as f64);
    }

    values
}

fn main() {

    // Figure 1

    // Plot 1
    let init_val: u64 = 123;
    let y_data: Vec<f64> = collatz(init_val);
    let x_data: Vec<f64> = (0u64..y_data.len() as u64).map(|x| x as f64).collect();
    let line = Line::new(&x_data, &y_data).set_color_rgba(0.9, 0.2, 0.2, 0.9);

    let plot11 = Plot::new().add_chart(&Chart::Line(line))
                            .set_local_frame(0.0, 0.49, 0.0, 0.69);

    // Plot 2
    let init_val: u64 = 237;
    let y_data: Vec<f64> = collatz(init_val);
    let x_data: Vec<f64> = (0u64..y_data.len() as u64).map(|x| x as f64).collect();
    let line = Line::new(&x_data, &y_data).set_stroke_style(StrokeStyle::Dashed);

    let plot12 = Plot::new().add_chart(&Chart::Line(line))
                            .set_local_frame(0.5, 0.99, 0.3, 0.99);

    let fig1 = Figure::new().add_plot(&plot11)
                            .add_plot(&plot12);

    // Figure 2

    let x_data = Array::from_iter((0..100).map(|x| (f64::from(x)) * 2.0 * PI / 100.0));
    let y_data1 = Array::from_iter((0..100).map(|i| x_data[i].sin()));
    let y_data2 = Array::from_iter((0..100).map(|i| (x_data[i] - PI / 2.0).sin()));

    let line1 = Line::new(&x_data, &y_data1);
    let line2 = Line::new(&x_data, &y_data2).set_color_rgba(0.9, 0.2, 0.2, 0.9);

    let plot21 = Plot::new().add_chart(&Chart::Line(line1))
                            .add_chart(&Chart::Line(line2))
                            .set_y_min(-1.2);

    let fig2 = Figure::new().add_plot(&plot21);
                            //.save("assets/multiple_figures.png").expect("Could not save multiple_figures.png");

    // Display on screen
    View::with_figure(fig1).expect("Could not add fig1 to view")
                           .add_figure(fig2).expect("Could not add fig2 to view")
                           .show();
}

