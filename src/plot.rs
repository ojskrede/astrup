//! ## Plot
//!
//! Module that defines the Plot structure.
//!

use std::f64::{MAX, MIN};

use cairo::Context;

use utils::{Plottable, Drawable, Frame};
use axis::{Orientation, Axis};
use chart::Chart;
//use style::Style;
//

#[derive(Clone, Debug)]
pub struct Canvas {
    color: [f64; 4],
    // A frame with (x, y) corners relative to the (0, 1)x(0, 1) system of its parent plot,
    local_frame: Frame,
    // A frame with (x, y) corners relative to the (0, 1) x (0, 1) global figure system
    // A frame with (x, y) corners relative to the data coordinate system. This is determined by
    // the union of the data range of its charts and the data range of its axes.
    data_frame: Frame,
    grid: bool,
    axes: Vec<Axis>,
    charts: Vec<Chart>,
}

impl Canvas {
    pub fn new() -> Canvas {
        Canvas {
            color: [1.0, 1.0, 1.0, 1.0],
            local_frame: Frame::new(),
            global_frame: Frame::new(),
            data_frame: Frame::new(),
            axes: Vec::<Axis>::new(),
            charts: Vec::<Chart>::new(),
        }
    }

    pub fn add_axis(&mut self, axis: Axis) {
        self.axes.push(axis);
    }

    pub fn add_chart(&mut self, chart: Chart) {
        self.charts.push(chart);
    }

    pub fn set_fig_frame(&mut self, frame: Frame) {
        self.fig_frame = frame;
    }

    pub fn set_data_frame(&mut self, frame: Frame) {
        self.data_frame = frame;
    }

    fn update_axes(&mut self) {

        self.x_axis_plot_start = self.fig_frame.x_min() + 0.2*(self.fig_frame.x_max() - self.fig_frame.x_min());
        self.x_axis_plot_end = self.fig_frame.x_max() - 0.1*(self.fig_frame.x_max() - self.fig_frame.x_min());
        self.y_axis_plot_start = self.fig_frame.y_max() - 0.2*(self.fig_frame.y_max() - self.fig_frame.y_min());
        self.y_axis_plot_end = self.fig_frame.y_min() + 0.1*(self.fig_frame.y_max() - self.fig_frame.y_min());

        self.x_axis.set_plot_coords(Frame::new(self.x_axis_plot_start, self.x_axis_plot_end,
                                               self.y_axis_plot_start, self.y_axis_plot_start));
        self.x_axis.set_plot_frame(Frame::new(self.x_axis_plot_start, self.x_axis_plot_end,
                                              self.y_axis_plot_end, self.y_axis_plot_start));

        self.y_axis.set_plot_coords(Frame::new(self.x_axis_plot_start, self.x_axis_plot_start,
                                               self.y_axis_plot_start, self.y_axis_plot_end));
        self.y_axis.set_plot_frame(Frame::new(self.x_axis_plot_start, self.x_axis_plot_end,
                                              self.y_axis_plot_end, self.y_axis_plot_start));


        let x_scale_factor = self.fig_frame.x_max() - self.fig_frame.x_min();
        let y_scale_factor = self.fig_frame.y_max() - self.fig_frame.y_min();
        let scale_factor = x_scale_factor.max(y_scale_factor);

        self.x_axis.scale_size(scale_factor);
        self.y_axis.scale_size(scale_factor);

    }

    fn find_largest_axes_data_frame(&self) -> Frame {
        let mut largest_axis_frame = Frame::from_corners(MAX, MIN, MAX, MIN);
        for axis in self.axis.iter() {
            match axis.orientation {
                Orientation::Horizontal => {
                    if chart.data_min() < largest_data_frame.left() {
                        largest_axis_frame.set_left(chart.data_min());
                    }
                    if chart.data_max() > largest_data_frame.right() {
                        largest_axis_frame.set_right(chart.data_max());
                    }
                Orientation::Vertical => {
                    if chart.data_min() < largest_data_frame.bottom() {
                        largest_axis_frame.set_bottom(chart.data_min());
                    }
                    if chart.data_max() > largest_data_frame.top() {
                        largest_axis_frame.set_top(chart.data_max());
                    }
                },
                },
            }
        }
        largest_data_frame
    }

    fn find_largest_chart_data_frame(&self) -> Frame {
        let mut largest_data_frame = Frame::from_corners(MAX, MIN, MAX, MIN);
        for chart in self.charts.iter() {
            if chart.data_frame().left() < largest_data_frame.left() {
                largest_data_frame.set_left(chart.data_frame().left());
            }
            if chart.data_frame().right() > largest_data_frame.right() {
                largest_data_frame.set_right(chart.data_frame().right());
            }
            if chart.data_frame().bottom() < largest_data_frame.y_bottom() {
                largest_data_frame.set_bottom(chart.data_frame().y_bottom());
            }
            if chart.data_frame().top() > largest_data_frame.y_top() {
                largest_data_frame.set_top(chart.data_frame().y_top());
            }
        }
        largest_data_frame
    }

    pub fn global_to_local(&mut self, fig_frame: Frame) {
        // This is a bit different from the other fit functions as it includes data, and a common
        // canvas for all charts.
        //
        // First, we update the current geometry, and axis and charts based on the change from
        // above in the hierarchy, that is, with x_factor and y_factor.
        //
        // Then, we update the geometry based on all charts and axes found in this canvas
        //
        // Then,

        // First, we update the fig_frame w.r.t. the scaling given in the arguments.
        self.fig_frame.set(self.fig_frame.left() * x_factor, self.fig_frame.right() * x_factor,
                           self.fig_frame.bottom() * y_factor, self.fig_frame.top() * y_factor);

        // Second, we update the data_frame to be the union of the largest data frames from our axes
        // and our charts.
        let frame_from_axes = self.find_largest_axes_data_frame();
        let frame_from_charts = self.find_largest_chart_data_frame();
        self.data_frame.set(frame_from_axes.left().min(frame_from_chart.left()),
                            frame_from_axes.right().max(frame_from_chart.right()),
                            frame_from_axes.bottom().min(frame_from_chart.bottom()),
                            frame_from_axes.top().max(frame_from_chart.top()))

        // Then, we update the axis, and charts based on this updated configuration
        for axis in self.axes.iter_mut() {
            axis.fit(fig_frame.clone(), data_frame.clone());
        }

        for chart in self.charts.iter_mut() {
            chart.fit(fig_frame.clone(), data_frame.clone());
        }
    }

    pub fn draw(&self, cr: &Context) {

        // Background
        cr.set_source_rgba(self.color[0], self.color[1], self.color[2], self.color[3]);
        cr.rectangle(self.fig_frame.x_min(), self.fig_frame.y_min(),
                     self.fig_frame.x_max() - self.fig_frame.x_min(),
                     self.fig_frame.y_max() - self.fig_frame.y_min());
        cr.fill();

        for chart in self.charts {
            chart.draw(cr);
        }

        for axis in self.axes {
            axis.draw(cr);
        }

    }
}

#[derive(Clone, Debug)]
pub struct Plot {
    //style: Style,
    title: Text,
    color: [f64; 4],
    fig_frame: Frame,
    border: bool,
    border_color: [f64; 4],
    border_width: f64,
    canvasses: Vec<Canvas>,
}

impl Plot {
    pub fn new(&self) -> Plot {
        Plot {
            title: Text::new("");
            color: [0.9, 0.9, 0.9, 0.9],
            fig_frame: Frame::new(),
            border: true,
            border_color: [0.0, 0.0, 0.0, 1.0],
            border_width: 0.005;
            canvasses: Vec::<Canvas>::new(),
        }
    }

    pub fn add(&mut self, chart: Chart) {
        self.charts.push(chart);
    }

    fn scale_size(&mut self, factor) {

        self.border_width *= factor;
        self.title.scale_size(factor);
    }

    /// This method is called by figure after all plots are added, and all plot adjustment is made.
    /// This happend right before the plot is drawn on the figure.
    ///
    /// The func
    pub fn fit(&mut self) {
        let delta_x = self.local_frame.right() - self.local_frame.left();
        let delta_y = self.local_frame.top() - self.local_frame.bottom();
        let scale_factor = (delta_x * delta_x + delta_y * delta_y).sqrt();

        self.scale_size(scale_factor);
        for canvas in self.canvasses.iter_mut() {
            canvas.fit(&self.local_frame);
        }

    }

    pub fn draw(&self, cr: &Context) {

        // Background
        cr.set_source_rgba(self.color[0], self.color[1], self.color[2], self.color[3]);
        cr.rectangle(self.fig_frame.x_min(), self.fig_frame.y_min(),
                     self.fig_frame.x_max() - self.fig_frame.x_min(),
                     self.fig_frame.y_max() - self.fig_frame.y_min());
        cr.fill();

        if self.border {
            cr.set_source_rgba(self.border_color[0], self.border_color[1],
                               self.border_color[2], self.border_color[3]);
            cr.move_to(self.fig_frame.x_min(), self.fig_frame.y_min());
            cr.set_line_width(self.border_width);
            cr.line_to(self.fig_frame.x_min(), self.fig_frame.y_max());
            cr.set_line_width(self.border_width);
            cr.line_to(self.fig_frame.x_max(), self.fig_frame.y_max());
            cr.set_line_width(self.border_width);
            cr.line_to(self.fig_frame.x_max(), self.fig_frame.y_min());
            cr.set_line_width(self.border_width);
            cr.line_to(self.fig_frame.x_min(), self.fig_frame.y_min());
            cr.stroke();
        }

        for canvas in self.canvasses.iter() {
            canvas.draw(cr);
        }
    }

}
