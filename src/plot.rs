//! ## Plot
//!
//! Module that defines the Plot structure.
//!

use std::f64::{MAX, MIN};

use cairo::Context;

use utils;
use utils::{Plottable, Drawable, Frame, Text, Mark};
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
    global_frame: Frame,
    // A frame with (x, y) corners relative to the data coordinate system. This is determined by
    // the union of the data range of its charts and the data range of its axes.
    data_frame: Frame,
    ref_num_marks: usize,
    hor_marks: Vec<Mark>,
    ver_marks: Vec<Mark>,
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
            ref_num_marks: 5,
            hor_marks: Vec::<Mark>::new(),
            ver_marks: Vec::<Mark>::new(),
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

    pub fn set_local_frame(&mut self, frame: Frame) {
        self.local_frame = frame;
    }

    pub fn set_data_frame(&mut self, frame: Frame) {
        self.data_frame = frame;
    }

    fn find_largest_axes_data_frame(&self) -> Frame {
        let mut largest_data_frame = Frame::from_sides(MAX, MIN, MAX, MIN);
        for axis in self.axes.iter() {
            match axis.orientation() {
                Orientation::Horizontal => {
                    if axis.data_min() < largest_data_frame.left() {
                        largest_data_frame.set_left(axis.data_min());
                    }
                    if axis.data_max() > largest_data_frame.right() {
                        largest_data_frame.set_right(axis.data_max());
                    }
                },
                Orientation::Vertical => {
                    if axis.data_min() < largest_data_frame.bottom() {
                        largest_data_frame.set_bottom(axis.data_min());
                    }
                    if axis.data_max() > largest_data_frame.top() {
                        largest_data_frame.set_top(axis.data_max());
                    }
                },
            }
        }
        largest_data_frame
    }

    fn find_largest_chart_data_frame(&self) -> Frame {
        let mut largest_data_frame = Frame::from_sides(MAX, MIN, MAX, MIN);
        for chart in self.charts.iter() {
            if chart.data_frame().left() < largest_data_frame.left() {
                largest_data_frame.set_left(chart.data_frame().left());
            }
            if chart.data_frame().right() > largest_data_frame.right() {
                largest_data_frame.set_right(chart.data_frame().right());
            }
            if chart.data_frame().bottom() < largest_data_frame.bottom() {
                largest_data_frame.set_bottom(chart.data_frame().bottom());
            }
            if chart.data_frame().top() > largest_data_frame.top() {
                largest_data_frame.set_top(chart.data_frame().top());
            }
        }
        largest_data_frame
    }

    pub fn fit(&mut self, plot_frame: Frame) {
        // First, we update the global_frame relative to the parent's global_frame.
        // After this is called, both local_frame and global_frame should not be altered.
        self.global_frame = self.local_frame.relative_to(&plot_frame);

        // Second, we update the data_frame. This is done in two stages.
        //
        // We first find a frame that is the union of the largest data frames from our axes
        // and our charts. This takes into account the possible user input (xrange() and yrange()),
        // as this defines the ranges of the axes.
        let frame_from_axes = self.find_largest_axes_data_frame();
        let frame_from_charts = self.find_largest_chart_data_frame();
        let largest_data_frame = Frame::from_sides(
                                    frame_from_axes.left().min(frame_from_charts.left()),
                                    frame_from_axes.right().max(frame_from_charts.right()),
                                    frame_from_axes.bottom().min(frame_from_charts.bottom()),
                                    frame_from_axes.top().max(frame_from_charts.top()));

        // With this, we compute marks on the vertical and horizontal sides. The boundary marks
        // will define the final data_frame.
        self.hor_marks = utils::compute_mark_locations(self.ref_num_marks,
                                                       self.global_frame.left(),
                                                       self.global_frame.right(),
                                                       largest_data_frame.left(),
                                                       largest_data_frame.right());
        self.ver_marks = utils::compute_mark_locations(self.ref_num_marks,
                                                       self.global_frame.bottom(),
                                                       self.global_frame.top(),
                                                       largest_data_frame.bottom(),
                                                       largest_data_frame.top());

        // We can now define our updated data_frame.
        // TODO: Ord for f64 equivalent
        //let data_left = self.hor_marks.iter().map(|x| x.data_mark()).min();
        //let data_right = self.hor_marks.iter().map(|x| x.data_mark()).max();
        //let data_bottom = self.ver_marks.iter().map(|x| x.data_mark()).min();
        //let data_top = self.ver_marks.iter().map(|x| x.data_mark()).max();
        let data_left = self.hor_marks[0].data_mark();
        let data_right = self.hor_marks.last().unwrap().data_mark();
        let data_bottom = self.ver_marks[0].data_mark();
        let data_top = self.ver_marks.last().unwrap().data_mark();
        self.data_frame = Frame::from_sides(data_left, data_right, data_bottom, data_top);

        // Then, we update the axis, and charts based on this updated configuration
        for axis in self.axes.iter_mut() {
            axis.fit(self.global_frame.clone(), self.data_frame.clone());
        }

        for chart in self.charts.iter_mut() {
            chart.fit(self.global_frame.clone(), self.data_frame.clone());
        }
    }

    pub fn draw(&self, cr: &Context) {

        // Background
        cr.set_source_rgba(self.color[0], self.color[1], self.color[2], self.color[3]);
        cr.rectangle(self.global_frame.left(), self.global_frame.bottom(),
                     self.global_frame.width(), self.global_frame.height());
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
    local_frame: Frame,
    border: bool,
    border_color: [f64; 4],
    border_width: f64,
    canvasses: Vec<Canvas>,
}

impl Plot {
    pub fn new(&self) -> Plot {
        Plot {
            title: Text::new(""),
            color: [0.9, 0.9, 0.9, 0.9],
            local_frame: Frame::new(),
            border: true,
            border_color: [0.0, 0.0, 0.0, 1.0],
            border_width: 0.005,
            canvasses: Vec::<Canvas>::new(),
        }
    }

    pub fn add(&mut self, canvas: Canvas) {
        self.canvasses.push(canvas);
    }

    fn scale_size(&mut self, factor: f64) {
        self.border_width *= factor;
        self.title.scale_size(factor);
    }

    /// This method is called by figure after all plots are added, and all plot adjustment is made.
    /// This happend right before the plot is drawn on the figure.
    ///
    /// The function scales various elements within the plot, and calls a similar plot for its
    /// canvasses. Since the figure is its closest parent, no additional global_frame is needed.
    pub fn fit(&mut self) {
        let delta_x = self.local_frame.right() - self.local_frame.left();
        let delta_y = self.local_frame.top() - self.local_frame.bottom();
        let scale_factor = (delta_x * delta_x + delta_y * delta_y).sqrt();

        self.scale_size(scale_factor);
        for canvas in self.canvasses.iter_mut() {
            canvas.fit(self.local_frame);
        }
    }

    pub fn draw(&self, cr: &Context) {

        // Background
        cr.set_source_rgba(self.color[0], self.color[1], self.color[2], self.color[3]);
        cr.rectangle(self.local_frame.left(), self.local_frame.bottom(),
                     self.local_frame.height(), self.local_frame.width());
        cr.fill();

        if self.border {
            cr.set_source_rgba(self.border_color[0], self.border_color[1],
                               self.border_color[2], self.border_color[3]);
            cr.move_to(self.local_frame.left(), self.local_frame.bottom());
            cr.set_line_width(self.border_width);
            cr.line_to(self.local_frame.right(), self.local_frame.bottom());
            cr.set_line_width(self.border_width);
            cr.line_to(self.local_frame.right(), self.local_frame.top());
            cr.set_line_width(self.border_width);
            cr.line_to(self.local_frame.left(), self.local_frame.top());
            cr.set_line_width(self.border_width);
            cr.line_to(self.local_frame.left(), self.local_frame.bottom());
            cr.stroke();
        }

        for canvas in self.canvasses.iter() {
            canvas.draw(cr);
        }
    }

}
