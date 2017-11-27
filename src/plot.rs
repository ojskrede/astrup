//! ## Plot
//!
//! Module that defines the Plot structure.
//!

use std::f64::{MAX, MIN};

use cairo::Context;

use utils::{Drawable, Frame};
use axis::{Orientation, Axis};
use scatter::Scatter;
use line::Line;
//use style::Style;

#[derive(Clone, Debug)]
pub enum PlotType {
    Scatter(Scatter),
    Line(Line),
}

/*
impl PlotType {
    fn new(&self) -> PlotType {
        match *self {
            PlotType::Point(ref p) => p.new(),
            PlotType::Scatter(ref s) => s.new(),
            PlotType::Line(ref l) => l.new(),
        }
    }
}
*/

impl Drawable for PlotType {
    fn draw_fn(&self, cr: &Context) {
        match *self {
            PlotType::Scatter(ref s) => s.draw_fn(cr),
            PlotType::Line(ref l) => l.draw_fn(cr),
        }
    }

    fn fit(&mut self, plot_frame: &Frame) {
        match *self {
            PlotType::Scatter(ref mut s) => s.fit(plot_frame),
            PlotType::Line(ref mut l) => l.fit(plot_frame),
        }
    }

    fn data_frame(&self) -> Frame {
        match *self {
            PlotType::Scatter(ref s) => s.data_frame(),
            PlotType::Line(ref l) => l.data_frame(),
        }
    }

    fn set_data_frame(&mut self, new_data_frame: Frame) {
        match *self {
            PlotType::Scatter(ref mut s) => s.set_data_frame(new_data_frame),
            PlotType::Line(ref mut l) => l.set_data_frame(new_data_frame),
        }
    }

    fn data_x_min(&self) -> f64 {
        match *self {
            PlotType::Scatter(ref s) => s.data_x_min(),
            PlotType::Line(ref l) => l.data_x_min(),
        }
    }

    fn data_x_max(&self) -> f64 {
        match *self {
            PlotType::Scatter(ref s) => s.data_x_max(),
            PlotType::Line(ref l) => l.data_x_max(),
        }
    }

    fn data_y_min(&self) -> f64 {
        match *self {
            PlotType::Scatter(ref s) => s.data_y_min(),
            PlotType::Line(ref l) => l.data_y_min(),
        }
    }

    fn data_y_max(&self) -> f64 {
        match *self {
            PlotType::Scatter(ref s) => s.data_y_max(),
            PlotType::Line(ref l) => l.data_y_max(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Plot {
    size: [usize; 2],
    //style: Style,
    title: String,
    bg_color: [f64; 4],
    grid: bool,
    border: bool,
    x_axis: Axis,
    y_axis: Axis,
    drawables: Vec<PlotType>,
    x_axis_plot_start: f64,
    x_axis_plot_end: f64,
    y_axis_plot_start: f64,
    y_axis_plot_end: f64,
    fig_frame: Frame, // Frame in figure coordinates
}

impl Plot {
    pub fn new() -> Plot {
        let x_axis_plot_start = 0.2;
        let x_axis_plot_end = 0.9;
        let y_axis_plot_start = 0.8;
        let y_axis_plot_end = 0.1;

        Plot {
            size: [200, 300],
            title: String::from("Plot"),
            bg_color: [0.9, 0.9, 0.9, 0.9],
            grid: false,
            border: true,
            x_axis_plot_start: x_axis_plot_start,
            x_axis_plot_end: x_axis_plot_end,
            y_axis_plot_start: y_axis_plot_start,
            y_axis_plot_end: y_axis_plot_end,
            x_axis: Axis::new(Orientation::Horizontal,
                              x_axis_plot_start, x_axis_plot_end,
                              y_axis_plot_start, y_axis_plot_start),
            y_axis: Axis::new(Orientation::Vertical,
                              x_axis_plot_start, x_axis_plot_start,
                              y_axis_plot_start, y_axis_plot_end),
            drawables: Vec::<PlotType>::new(),
            fig_frame: Frame::new(0.0, 1.0, 0.0, 1.0),
        }
    }

    pub fn grid(&mut self) {
        self.grid = true;
    }

    /// This method specifies this plot's location in the (0, 1) x (0, 1) figure's coordinate
    /// system
    pub fn set_fig_frame(&mut self, x_min: f64, x_max: f64, y_min: f64, y_max: f64) {
        self.fig_frame = Frame::new(x_min, x_max, y_min, y_max);
        self.update_axes();
    }

    fn update_axes(&mut self) {

        self.x_axis_plot_start = self.fig_frame.x_min() + 0.2*(self.fig_frame.x_max() - self.fig_frame.x_min());
        self.x_axis_plot_end = self.fig_frame.x_max() - 0.1*(self.fig_frame.x_max() - self.fig_frame.x_min());
        self.y_axis_plot_start = self.fig_frame.y_max() - 0.2*(self.fig_frame.y_max() - self.fig_frame.y_min());
        self.y_axis_plot_end = self.fig_frame.y_min() + 0.1*(self.fig_frame.y_max() - self.fig_frame.y_min());

        self.x_axis = Axis::new(Orientation::Horizontal,
                                self.x_axis_plot_start, self.x_axis_plot_end,
                                self.y_axis_plot_start, self.y_axis_plot_start);
        self.x_axis.set_plot_frame(Frame::new(self.x_axis_plot_start, self.x_axis_plot_end,
                                              self.y_axis_plot_end, self.y_axis_plot_start));

        self.y_axis = Axis::new(Orientation::Vertical,
                               self.x_axis_plot_start, self.x_axis_plot_start,
                               self.y_axis_plot_start, self.y_axis_plot_end);
        self.y_axis.set_plot_frame(Frame::new(self.x_axis_plot_start, self.x_axis_plot_end,
                                              self.y_axis_plot_end, self.y_axis_plot_start));

    }

    pub fn bg_color(&mut self, bg_color: &[f64; 4]) {
        self.bg_color = bg_color.to_owned();
    }

    pub fn x_label(&mut self, label: &str) {
        self.x_axis.set_label(label);
    }

    pub fn y_label(&mut self, label: &str) {
        self.y_axis.set_label(label);
    }

    pub fn draw(&mut self, drawable: PlotType) {
        self.drawables.push(drawable);
    }

    pub fn fit(&mut self) {

        let mut largest_data_frame = Frame::new(MAX, MIN, MAX, MIN);
        for drawable in self.drawables.iter() {
            if drawable.data_x_min() < largest_data_frame.x_min() {
                largest_data_frame.set_x_min(drawable.data_x_min());
            }
            if drawable.data_x_max() > largest_data_frame.x_max() {
                largest_data_frame.set_x_max(drawable.data_x_max());
            }
            if drawable.data_y_min() < largest_data_frame.y_min() {
                largest_data_frame.set_y_min(drawable.data_y_min());
            }
            if drawable.data_y_max() > largest_data_frame.y_max() {
                largest_data_frame.set_y_max(drawable.data_y_max());
            }
        }

        self.x_axis.set_data_range(largest_data_frame.x_min(), largest_data_frame.x_max());
        self.y_axis.set_data_range(largest_data_frame.y_min(), largest_data_frame.y_max());

        //let frame = Frame::new(0.2, 0.9, 0.1, 0.8);
        for drawable in self.drawables.iter_mut() {
            drawable.set_data_frame(largest_data_frame.clone());
            drawable.fit(&Frame::new(self.x_axis_plot_start, self.x_axis_plot_end,
                                     self.y_axis_plot_start, self.y_axis_plot_end));
        }
    }

    pub fn draw_fn(&self, cr: &Context) {

        //cr.scale(self.size[1] as f64, self.size[0] as f64); // TODO: Why does this ruin things

        // Background
        cr.set_source_rgba(self.bg_color[0], self.bg_color[1], self.bg_color[2], self.bg_color[3]);
        cr.rectangle(self.fig_frame.x_min(), self.fig_frame.y_min(),
                     self.fig_frame.x_max() - self.fig_frame.x_min(),
                     self.fig_frame.y_max() - self.fig_frame.y_min());
        cr.fill();

        if self.border {
            cr.set_source_rgb(0.0, 0.0, 0.0);
            cr.set_line_width(0.005);
            cr.move_to(self.fig_frame.x_min(), self.fig_frame.y_min());
            cr.line_to(self.fig_frame.x_min(), self.fig_frame.y_max());
            cr.line_to(self.fig_frame.x_max(), self.fig_frame.y_max());
            cr.line_to(self.fig_frame.x_max(), self.fig_frame.y_min());
            cr.line_to(self.fig_frame.x_min(), self.fig_frame.y_min());
            cr.stroke();
        }

        // Horizontal axis
        self.x_axis.draw_fn(cr);
        self.y_axis.draw_fn(cr);

        for drawable in self.drawables.iter() {
            drawable.draw_fn(cr);
        }
    }

}

/*
impl Drawable for Plot {
    fn draw_fn(&self, cr: &Context) {

        //cr.scale(self.size[1] as f64, self.size[0] as f64); // TODO: Why does this ruin things

        // Background
        cr.set_source_rgba(self.bg_color[0], self.bg_color[1], self.bg_color[2], self.bg_color[3]);
        cr.paint();

        // if self.grid {}

        if self.border {
            cr.set_source_rgb(0.0, 0.0, 0.0);
            cr.set_line_width(0.01);
            cr.move_to(0.0, 0.0);
            cr.line_to(0.0, 1.0);
            cr.line_to(1.0, 1.0);
            cr.line_to(1.0, 0.0);
            cr.line_to(0.0, 0.0);
            cr.stroke();
        }

        // Horizontal axis
        cr.set_source_rgba(self.x_axis.color[0], self.x_axis.color[1], self.x_axis.color[2],
                           self.x_axis.color[3]);
        cr.set_line_width(self.x_axis.lw);
        cr.move_to(self.x_axis.x_start, self.x_axis.y_start);
        cr.line_to(self.x_axis.x_end, self.x_axis.y_end);
        cr.stroke();

        // Vertical axis
        cr.set_source_rgba(self.y_axis.color[0], self.y_axis.color[1], self.y_axis.color[2],
                           self.y_axis.color[3]);
        cr.set_line_width(self.y_axis.lw);
        cr.move_to(self.y_axis.x_start, self.y_axis.y_start);
        cr.line_to(self.y_axis.x_end, self.y_axis.y_end);
        cr.stroke();
    }
}
*/
