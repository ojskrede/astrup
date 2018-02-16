//! Binary used for development and testing of experimental implementations
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
                               .set_x_label("x axis label")
                               .set_y_label("y axis label")
                               .set_y_label_angle(PI / 2.0)
                               .set_local_frame(0.1, 0.9, 0.2, 0.8);

    // Add the plots to a figure, and save it
    let fig = Figure::new().add(&line_plot)
                           .set_width(1000)
                           .set_height(1000)
                           .set_border_thickness(0.001);

    // Display the result on screen
    View::new_from(fig).expect("Could not add figure to view")
                       .show();
}
