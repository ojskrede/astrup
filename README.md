# Astrup

A rust plotting library using [gtk-rs](https://github.com/gtk-rs/gtk) as a backend. This is *still*
very much a small hobby project. I believe it is ok for simple use, but lots of changes (also
breaking ones) should be expected.

For alternative rust plotting libraries, see e.g.

- [rustplotlib](https://github.com/ubnt-intrepid/rustplotlib)
- [RustGnuplot](https://github.com/SiegeLord/RustGnuplot)
- [dataplotlib](https://github.com/coder543/dataplotlib)


## Build:

- [Install rust](https://www.rust-lang.org/en-US/install.html), and make sure `~/.cargo/bin/` is in
  your `PATH`.
- Make sure you have **GTK+**, **GLib**, and **Cairo** development files on your system.
  Install instructions can be found [here](http://gtk-rs.org/docs/requirements.html).
- Clone the repository, and build with `cargo build`.

## Goals:
- Plot data from multiple containers (first priority: `Vec` and `ndarray`).
- Building plots should be intuitive.
- Plots should look nice and informative.
- It should be very configurable.

## Planned charts

The table below lists, by priority, the planned chart variants. Currently, only simple scatter and
line plots are implemented.

- [x] Scatter chart
- [x] Line chart
- [ ] Bar chart
- [ ] Histogram
- [ ] Matrix heatmap
- [ ] Image
- [ ] Filled curves
- [ ] Box plot
- [ ] Pie chart

## Example

```rust
extern crate astrup;
extern crate ndarray;
extern crate rand;

use std::f64::consts::PI;

use ndarray::Array;
use rand::distributions::{IndependentSample, Normal};
use rand::{SeedableRng, StdRng};

use astrup::{Chart, Figure, HtmlColor, Line, Plot, Scatter, View};

fn main() {
    // Create data contained in ndarray
    let num_samples = 1000;
    let x_data =
        Array::from_iter((0..num_samples).map(|x| -5.0 + 10.0 * (x as f64) / num_samples as f64));
    let y_data1 = Array::from_iter((0..num_samples).map(|i| (x_data[i] - 0.0 * PI / 8.0).sin()));
    let y_data2 = Array::from_iter((0..num_samples).map(|i| (x_data[i] - 1.0 * PI / 8.0).sin()));
    let y_data3 = Array::from_iter((0..num_samples).map(|i| (x_data[i] - 2.0 * PI / 8.0).sin()));
    let y_data4 = Array::from_iter((0..num_samples).map(|i| (x_data[i] - 3.0 * PI / 8.0).sin()));
    let y_data5 = Array::from_iter((0..num_samples).map(|i| (x_data[i] - 4.0 * PI / 8.0).sin()));
    let y_data6 = Array::from_iter((0..num_samples).map(|i| (x_data[i] - 5.0 * PI / 8.0).sin()));
    let y_data7 = Array::from_iter((0..num_samples).map(|i| (x_data[i] - 6.0 * PI / 8.0).sin()));
    let y_data8 = Array::from_iter((0..num_samples).map(|i| (x_data[i] - 7.0 * PI / 8.0).sin()));

    // Plot lines
    let line1 = Line::new(&x_data, &y_data1);
    let line2 = Line::new(&x_data, &y_data2);
    let line3 = Line::new(&x_data, &y_data3);
    let line4 = Line::new(&x_data, &y_data4);
    let line5 = Line::new(&x_data, &y_data5);
    let line6 = Line::new(&x_data, &y_data6);
    let line7 = Line::new(&x_data, &y_data7);
    let line8 = Line::new(&x_data, &y_data8);

    // Add lines to a plot
    let line_plot = Plot::new()
        .add_chart(&Chart::Line(line1))
        .add_chart(&Chart::Line(line2))
        .add_chart(&Chart::Line(line3))
        .add_chart(&Chart::Line(line4))
        .add_chart(&Chart::Line(line5))
        .add_chart(&Chart::Line(line6))
        .add_chart(&Chart::Line(line7))
        .add_chart(&Chart::Line(line8))
        .set_y_min(-1.2)
        .set_local_frame(0.0, 0.7, 0.51, 1.0);

    // Create a seedable rng so that the scatter points are equal from run to run
    let seed: Vec<usize> = vec![8, 8, 8, 8, 8, 8, 8, 8];
    let mut seeded_rng: StdRng = SeedableRng::from_seed(seed.as_slice());

    // Create scatter points
    let normal_1 = Normal::new(-3.0, 1.0);
    let normal_2 = Normal::new(0.0, 2.0);
    let normal_3 = Normal::new(3.0, 2.0);
    let normal_4 = Normal::new(0.0, 1.0);
    let num_points = 10_000;
    let x_data_1: Vec<f64> = (0..num_points)
        .map(|_| normal_1.ind_sample(&mut seeded_rng) as f64)
        .collect();
    let y_data_1: Vec<f64> = (0..num_points)
        .map(|_| normal_2.ind_sample(&mut seeded_rng) as f64)
        .collect();
    let x_data_2: Vec<f64> = (0..num_points)
        .map(|_| normal_3.ind_sample(&mut seeded_rng) as f64)
        .collect();
    let y_data_2: Vec<f64> = (0..num_points)
        .map(|_| normal_4.ind_sample(&mut seeded_rng) as f64)
        .collect();
    let scatter_1 = Scatter::new(&x_data_1, &y_data_1)
        .set_color_html(&HtmlColor::Lightskyblue)
        .set_point_size(0.002);
    let scatter_2 = Scatter::new(&x_data_2, &y_data_2)
        .set_color_rgba_u8(255, 69, 0, 200)
        .set_point_size(0.002);

    // Add scatter points to a new plot
    let scatter_plot = Plot::new()
        .set_local_frame(0.3, 1.0, 0.0, 0.49)
        .set_x_label("x")
        .set_y_label("y")
        .set_y_label_angle(0.0)
        .set_title("Scatter plot")
        .add_chart(&Chart::Scatter(scatter_1))
        .add_chart(&Chart::Scatter(scatter_2));

    // Add the plots to a figure, and save it
    let fig = Figure::new()
        .add_plot(&line_plot)
        .add_plot(&scatter_plot)
        .set_width(1000)
        .set_height(800)
        .set_border_thickness(0.001)
        .save("assets/frontpage_example.png")
        .expect("Could not save frontpage_example.png");

    // Display the result on screen
    View::with_figure(fig)
        .expect("Could not add figure to view")
        .show();
}
```

![Plot](assets/frontpage_example.png)
