//! Examples
//!

extern crate astrup;

use astrup::figure::Figure;
use astrup::plot::{Plot, PlotType};
use astrup::scatter::Scatter;
use astrup::line::Line;

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
    let x_data = vec![50.0, 51.0, 46.0, 40.0];
    let y_data = vec![590.0, 510.0, 600.0, 450.0];
    let scatter = Scatter::new(&x_data, &y_data);

    //let init_val: u64 = 837799; // Produces longest sequence of init values < 1 000 000
    let init_val: u64 = 123;
    let y_data: Vec<f64> = collatz(init_val);
    let x_data: Vec<f64> = (0u64..y_data.len() as u64).map(|x| x as f64).collect();
    let line = Line::new(&x_data, &y_data);

    let mut plot1 = Plot::new();
    plot1.x_label("x");
    plot1.y_label("y");
    plot1.set_fig_frame(0.0, 0.49, 0.0, 0.49);
    plot1.draw(PlotType::Line(line));
    plot1.draw(PlotType::Scatter(scatter));


    let init_val: u64 = 237;
    let y_data: Vec<f64> = collatz(init_val);
    let x_data: Vec<f64> = (0u64..y_data.len() as u64).map(|x| x as f64).collect();
    let line = Line::new(&x_data, &y_data);

    let mut plot2 = Plot::new();
    plot2.x_label("x");
    plot2.y_label("y");
    plot2.set_fig_frame(0.5, 0.99, 0.5, 0.99);
    plot2.draw(PlotType::Line(line));

    let mut fig = Figure::new();
    fig.draw(plot1);
    fig.draw(plot2);
    fig.show();

    //let mut fig2 = Figure::new();
    //fig2.draw(plot);
    //fig2.show();
}
