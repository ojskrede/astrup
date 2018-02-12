//! Example of how to build multiple figures
//!

extern crate ndarray;
extern crate astrup;

use std::f64::consts::PI;

use ndarray::Array;

use astrup::view::View;
use astrup::figure::Figure;
use astrup::plot::Plot;
use astrup::chart::Chart;
use astrup::chart::line::Line;

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

    return values
}

fn main() {

    // Figure 1

    // Plot 1
    let init_val: u64 = 123;
    let y_data: Vec<f64> = collatz(init_val);
    let x_data: Vec<f64> = (0u64..y_data.len() as u64).map(|x| x as f64).collect();
    let mut line = Line::new(&x_data, &y_data);
    line.set_color(0.9, 0.2, 0.2, 0.9);

    let mut plot11 = Plot::new();
    plot11.set_local_frame(0.0, 0.49, 0.0, 0.69);
    plot11.add(Chart::Line(line));

    // Plot 2
    let init_val: u64 = 237;
    let y_data: Vec<f64> = collatz(init_val);
    let x_data: Vec<f64> = (0u64..y_data.len() as u64).map(|x| x as f64).collect();
    let mut line = Line::new(&x_data, &y_data);
    line.set_stroke_style("dashed");

    let mut plot12 = Plot::new();
    plot12.set_local_frame(0.5, 0.99, 0.3, 0.99);
    plot12.add(Chart::Line(line));

    let mut fig1 = Figure::new();
    fig1.add(plot11);
    fig1.add(plot12);

    // Figure 2

    let x_data = Array::from_iter((0..100).map(|x| (x as f64) * 2.0 * PI / 100.0));
    let y_data1 = Array::from_iter((0..100).map(|i| x_data[i].sin()));
    let y_data2 = Array::from_iter((0..100).map(|i| (x_data[i] - PI / 2.0).sin()));

    let line1 = Line::new(&x_data, &y_data1);
    let mut line2 = Line::new(&x_data, &y_data2);
    line2.set_color(0.9, 0.2, 0.2, 0.9);

    let mut plot21 = Plot::new();
    plot21.add(Chart::Line(line1));
    plot21.add(Chart::Line(line2));
    plot21.set_y_min(-1.2);

    let mut fig2 = Figure::new();
    fig2.add(plot21);

    // Display on screen
    let mut view = View::new_from(fig1).expect("Could not add fig1 to view");
    view.add(fig2).expect("Could not add fig2 to view");
    view.show();
}

