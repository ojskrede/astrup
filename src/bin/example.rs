//! Examples
//!

extern crate astrup;

use astrup::view::View;
use astrup::figure::Figure;
use astrup::plot::Plot;
use astrup::chart::Chart;
use astrup::chart::scatter::Scatter;
use astrup::chart::line::Line;

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
    plot1.set_local_frame(0.0, 0.49, 0.0, 0.69);
    plot1.add(Chart::Line(line));
    plot1.add(Chart::Scatter(scatter));


    let init_val: u64 = 237;
    let y_data: Vec<f64> = collatz(init_val);
    let x_data: Vec<f64> = (0u64..y_data.len() as u64).map(|x| x as f64).collect();
    let line = Line::new(&x_data, &y_data);

    let mut plot2 = Plot::new();
    plot2.set_local_frame(0.5, 0.99, 0.3, 0.99);
    plot2.add(Chart::Line(line));

    let mut fig = Figure::new();
    fig.add(plot1);
    fig.add(plot2);
    fig.save("example.png").expect("Could not create example.png");

    // TODO: Add support for this kind of short-hand thing
    //Figure::new().add(Plot::new().add(Chart::Line(Line::new(&x_data, &y_data))));


    let init_val: u64 = 4327;
    let y_data: Vec<f64> = collatz(init_val);
    let x_data: Vec<f64> = (0u64..y_data.len() as u64).map(|x| x as f64).collect();
    let line = Line::new(&x_data, &y_data);

    let mut plot3 = Plot::new();
    plot3.add(Chart::Line(line));

    let mut fig2 = Figure::new();
    fig2.add(plot3);
    fig2.save("example2.png").expect("Could not create example2.png");

    // Display on screen
    let mut view = View::new(fig);
    view.add(fig2);
    view.show();
}
