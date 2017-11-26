//! Examples
//!

extern crate astrup;

use astrup::figure::Figure;
use astrup::plot::Plot;
use astrup::scatter::Scatter;

fn main() {
    let data_x = vec![1.0, 2.0, 4.0, 8.0];
    let data_y = vec![1.0, 2.0, 3.0, 4.0];

    let scatter = Scatter::new(&data_x, &data_y);

    let mut plot = Plot::new();
    plot.x_label("x");
    plot.y_label("y");
    plot.draw(scatter);

    let mut fig = Figure::new();
    fig.draw(plot);
    fig.show();
}
