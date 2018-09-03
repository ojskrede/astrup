//! Binary used for development and testing of experimental implementations
//!

extern crate ndarray;
extern crate astrup;

use std::f64::consts::PI;

use ndarray::Array;

use astrup::{View, Figure, Plot, Chart, Line};

fn main() {

    // Create data contained in ndarray
    let x_data = Array::from_iter((0..100).map(|x| (f64::from(x)) * 2.0 * PI / 100.0));

    // Add lines to a plot
    let mut line_plot = Plot::new();
    line_plot.set_title("Plot title")
             .set_y_min(-1.2)
             .set_y_label_angle(PI / 2.0)
             .set_local_frame(0.0, 1.0, 0.0, 1.0);

    for denom in 1..9 {
        let y_data = Array::from_iter((0..100).map(|i| (x_data[i] - PI / (denom as f64)).sin()));
        let line = Line::new(&x_data, &y_data);
        line_plot.add_chart(&Chart::Line(line));
    }

    // Add the plots to a figure, and save it
    let fig = Figure::new().add_plot(&line_plot)
                           .set_width(1000)
                           .set_height(1000)
                           .set_title("Figure title")
                           .set_border_thickness(0.01);
                           //.save("development_example.png").expect("Could not save devel example");

    // Display the result on screen
    View::with_figure(fig).expect("Could not add figure to view")
                          .show();
}
