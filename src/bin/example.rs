//! Examples
//!

extern crate astrup;

use astrup::figure::Figure;
use astrup::plot::Plot;
use astrup::scatter::Scatter;
use astrup::line::Line;

fn main() {
    let x_data = vec![1.0, 2.0, 4.0, 8.0];
    let y_data = vec![1.0, 2.0, 3.0, 4.0];

    //let scatter = Scatter::new(&data_x, &data_y);
    let line = Line::new(&x_data, &y_data);

    let mut plot = Plot::new();
    plot.x_label("x");
    plot.y_label("y");
    plot.draw(line);

    let mut fig = Figure::new();
    fig.draw(plot);
    fig.show();
}
