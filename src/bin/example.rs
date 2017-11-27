//! Examples
//!

extern crate astrup;

use astrup::figure::Figure;
use astrup::plot::Plot;
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
    //let x_data = vec![1.0, 2.0, 4.0, 8.0];
    //let y_data = vec![1.0, 2.0, 3.0, 4.0];
    //let scatter = Scatter::new(&data_x, &data_y);

    //let init_val: u64 = 837799; // Produces longest sequence of init values < 1 000 000
    let init_val: u64 = 123;
    let y_data: Vec<f64> = collatz(init_val);
    let x_data: Vec<f64> = (0u64..y_data.len() as u64).map(|x| x as f64).collect();
    let line = Line::new(&x_data, &y_data);

    let mut plot = Plot::new();
    plot.x_label("x");
    plot.y_label("y");
    plot.draw(line);

    let mut fig = Figure::new();
    fig.draw(plot);
    fig.show();
}
