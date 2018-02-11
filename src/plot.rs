//! Module that defines the Plot structure.

use std::f64;
use failure::Error;

use cairo::Context;
use palette::Rgba;

use utils::{Plottable, Drawable, Frame, Text, Coord};
use axis::{Axis};
use mark::{Mark, GridLine};
use chart::Chart;
//use style::Style;
//

/// ## Canvas
///
/// This is the area of the plot where the data is actually displayed, and it is enclosed by the
/// ``main axes''.
#[derive(Clone, Debug)]
pub struct Canvas {
    color: Rgba,
    local_frame: Frame,
    global_frame: Frame,
    data_frame: Frame,
    user_data_frame: Frame,
    display_axes: bool,
    display_grid: bool,
    grid_width: f64,
    grid_color: Rgba,
    grid: Vec<GridLine>,
    hor_marks: Vec<Mark>, // TODO: Use these in stead of axis
    ver_marks: Vec<Mark>,
    axes: Vec<Axis>,
    charts: Vec<Chart>,
}

impl Canvas {
    /// Create and return a new canvas
    pub fn new() -> Canvas {
        Canvas {
            color: Rgba::new(235.0/255.0, 230.0/255.0, 242.0/255.0, 0.8),
            //local_frame: Frame::new(),
            local_frame: Frame::from_sides(0.15, 0.95, 0.15, 0.95), // TODO: Update w.r.t. the width of axis labels and tick labels
            global_frame: Frame::new(),
            data_frame: Frame::new(),
            user_data_frame: Frame::new(),
            display_axes: true,
            display_grid: true,
            grid_width: 0.005,
            grid_color: Rgba::new(1.0, 1.0, 1.0, 0.9),
            grid: Vec::<GridLine>::new(),
            hor_marks: Vec::<Mark>::new(),
            ver_marks: Vec::<Mark>::new(),
            axes: Vec::<Axis>::new(),
            charts: Vec::<Chart>::new(),
        }
    }

    /// Set the background color of the canvas.
    pub fn set_color(&mut self, color: Rgba) {
        self.color = color;
    }

    /// Set local frame coordinates.
    pub fn set_local_frame(&mut self, frame: Frame) {
        self.local_frame = frame;
    }

    /// Set data range.
    ///
    /// *Note*:
    /// This is a soft suggestion, and can be overwritten before the final result for aestethics.
    /// The data range is determined by data to be plotted, or user input (like this
    /// function). But the final range is also determined by: the number of marks (ticks), and that
    /// each tick should be one of *n x {1, 2, 5} x 10^p* for some integer *n* and power *p*. See
    /// more of how this is actually determined [here](struct.Axis.html#method.compute_marks).
    pub fn set_data_range(&mut self, x_min: f64, x_max: f64, y_min: f64, y_max: f64) {
        self.user_data_frame.set(x_min, x_max, y_min, y_max);
    }

    /// Set horisontal data range
    ///
    /// *Note*:
    /// This is a soft suggestion, and can be overwritten before the final result for aestethics.
    /// See more [here](struct.Canvas.html#method.set_data_range).
    pub fn set_x_range(&mut self, x_min: f64, x_max: f64) {
        self.user_data_frame.set_left(x_min);
        self.user_data_frame.set_right(x_max);
    }

    /// Set vertical data range
    ///
    /// *Note*:
    /// This is a soft suggestion, and can be overwritten before the final result for aestethics.
    /// See more [here](struct.Canvas.html#method.set_data_range).
    pub fn set_y_range(&mut self, y_min: f64, y_max: f64) {
        self.user_data_frame.set_bottom(y_min);
        self.user_data_frame.set_top(y_max);
    }

    /// Set left horisontal coordinate end
    ///
    /// *Note*:
    /// This is a soft suggestion, and can be overwritten before the final result for aestethics.
    /// See more [here](struct.Canvas.html#method.set_data_range).
    pub fn set_x_min(&mut self, x_min: f64) {
        self.user_data_frame.set_left(x_min);
    }

    /// Set right horisontal coordinte end
    ///
    /// *Note*:
    /// This is a soft suggestion, and can be overwritten before the final result for aestethics.
    /// See more [here](struct.Canvas.html#method.set_data_range).
    pub fn set_x_max(&mut self, x_max: f64) {
        self.user_data_frame.set_right(x_max);
    }

    /// Set bottom vertical coordinate end
    ///
    /// *Note*:
    /// This is a soft suggestion, and can be overwritten before the final result for aestethics.
    /// See more [here](struct.Canvas.html#method.set_data_range).
    pub fn set_y_min(&mut self, y_min: f64) {
        self.user_data_frame.set_bottom(y_min);
    }

    /// Set top vertical coordinate end
    ///
    /// *Note*:
    /// This is a soft suggestion, and can be overwritten before the final result for aestethics.
    /// See more [here](struct.Canvas.html#method.set_data_range).
    pub fn set_y_max(&mut self, y_max: f64) {
        self.user_data_frame.set_top(y_max);
    }

    /// Whether or not to display axes
    pub fn display_axes(&mut self, val: bool) {
        self.display_axes = val;
    }

    /// Whether or not to display grid
    pub fn display_grid(&mut self, val: bool) {
        self.display_grid = val;
    }

    /// Set the line width of the gridlines
    pub fn set_gridline_width(&mut self, val: f64) {
        self.grid_width = val;
    }

    /// Set the color of the grid lines
    pub fn set_grid_color(&mut self, color: Rgba) {
        self.grid_color = color;
    }

    /// Add an additional axis to the canvas
    pub fn add_axis(&mut self, axis: Axis) {
        self.axes.push(axis);
    }

    /// Add an additional chart to the canvas
    pub fn add_chart(&mut self, chart: Chart) {
        self.charts.push(chart);
    }

    /// Compute grid lines given a vertical and a horisontal axis
    fn compute_grid(&mut self, ver_axis: &Axis, hor_axis: &Axis) {
        for coord in ver_axis.mark_coords() {
            let mut gridline = GridLine::new(coord.clone(),
                                             Coord::new(self.global_frame.right(), coord.y()));
            gridline.set_color(self.grid_color);
            gridline.set_width(self.grid_width);
            gridline.scale_size(self.global_frame.diag_len() / 2f64.sqrt());
            self.grid.push(gridline);
        }
        for coord in hor_axis.mark_coords() {
            let mut gridline = GridLine::new(coord.clone(),
                                             Coord::new(coord.x(), self.global_frame.top()));
            gridline.set_color(self.grid_color);
            gridline.set_width(self.grid_width);
            gridline.scale_size(self.global_frame.diag_len() / 2f64.sqrt());
            self.grid.push(gridline);
        }
    }

    /// Find the smallest data frame including all data points from all charts
    fn find_largest_chart_data_frame(&self) -> Option<Frame> {
        if self.charts.len() == 0 { return None }
        let mut largest_data_frame = Frame::from_sides(f64::MAX, f64::MIN, f64::MAX, f64::MIN);
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
        Some(largest_data_frame)
    }

    /// Compute the data frame used by this canvas.
    ///
    /// First priority is user input. For the entries where there is no user input, the value
    /// should be the smallest value larger than or equal to all respective chart values. If there
    /// are no chart values, the default is chosen.
    fn compute_data_frame(&self) -> Frame {
        let mut return_this_data_frame = match self.find_largest_chart_data_frame() {
            Some(val) => val,
            None => self.data_frame.clone(),
        };

        if self.user_data_frame.is_left_updated() {
            return_this_data_frame.set_left(self.user_data_frame.left());
        }

        if self.user_data_frame.is_right_updated() {
            return_this_data_frame.set_right(self.user_data_frame.right());
        }

        if self.user_data_frame.is_bottom_updated() {
            return_this_data_frame.set_bottom(self.user_data_frame.bottom());
        }

        if self.user_data_frame.is_top_updated() {
            return_this_data_frame.set_top(self.user_data_frame.top());
        }
        return return_this_data_frame
    }

    /// Sets a default horizontal and vertical axis. This is important in order to determine the
    /// data_frame of this canvas. The reason for this is that the data frame changes with these
    /// axes, because of nice tick labeling.
    ///
    /// There is really no reason for these to be axis, but we need to compute the marks in order
    /// to determine the data_frame of the canvas. In the future, there might be a more elegant
    /// solution to this.
    ///
    /// In the meantime, the possibility to not draw the axes have to suffice.
    fn set_default_axis(&mut self, data_frame: Frame) -> Result<(Axis, Axis), Error> {
        let mut hor_axis = Axis::from_coord(Coord::new(0.0, 0.0), Coord::new(1.0, 0.0));
        hor_axis.set_data_range(data_frame.left(), data_frame.right());
        hor_axis.set_label("x");
        //hor_axis.scale_label_offset(-1.5);
        hor_axis.scale_tick_length(-1.0);
        hor_axis.compute_marks()?;
        hor_axis.set_label_offset(-0.01, -0.13);
        hor_axis.set_tick_label_offset(-0.02, -0.07);
        hor_axis.set_tick_font_size(0.025);

        let mut ver_axis = Axis::from_coord(Coord::new(0.0, 0.0), Coord::new(0.0, 1.0));
        ver_axis.set_data_range(data_frame.bottom(), data_frame.top());
        ver_axis.set_label("y");
        //ver_axis.scale_label_offset(1.5);
        //ver_axis.set_label_angle(-PI / 2.0);
        ver_axis.compute_marks()?;
        //ver_axis.scale_tick_label_offset(1.7);
        ver_axis.set_label_offset(-0.14, -0.01);
        ver_axis.set_tick_label_offset(-0.10, -0.01);
        ver_axis.set_tick_font_size(0.025);

        Ok((hor_axis, ver_axis))
    }

    /// Fit the this canvas to its plot
    pub fn fit(&mut self, plot_frame: Frame) -> Result<(), Error> {
        // First, we update the global_frame relative to the parent's global_frame.
        // After this is called, both local_frame and global_frame should not be altered.
        self.global_frame = self.local_frame.relative_to(&plot_frame);

        // Second, we update the data_frame
        let data_frame = self.compute_data_frame();

        // Then we compute one horizontal and one vertical axis.
        let (mut hor_axis, mut ver_axis) = self.set_default_axis(data_frame)?;

        // We can now define our updated data_frame.
        // TODO: Ord for f64 equivalent
        //let data_left = self.hor_marks.iter().map(|x| x.data_mark()).min();
        //let data_right = self.hor_marks.iter().map(|x| x.data_mark()).max();
        //let data_bottom = self.ver_marks.iter().map(|x| x.data_mark()).min();
        //let data_top = self.ver_marks.iter().map(|x| x.data_mark()).max();
        let data_left = hor_axis.data_min();
        let data_right = hor_axis.data_max();
        let data_bottom = ver_axis.data_min();
        let data_top = ver_axis.data_max();
        self.data_frame = Frame::from_sides(data_left, data_right, data_bottom, data_top);

        // Then, we update the axis, and charts based on this updated configuration
        ver_axis.fit(&self.global_frame);
        hor_axis.fit(&self.global_frame);

        // Set grid
        if self.display_grid {
            self.compute_grid(&ver_axis, &hor_axis);
        }

        self.axes = vec![hor_axis, ver_axis];

        for chart in self.charts.iter_mut() {
            chart.fit(&self.global_frame, &self.data_frame);
        }

        Ok(())
    }

    /// Draw the canvas
    pub fn draw(&self, cr: &Context) {

        // Background
        cr.set_source_rgba(self.color.red as f64, self.color.green as f64, self.color.blue as f64,
                           self.color.alpha as f64);
        cr.rectangle(self.global_frame.left(), self.global_frame.bottom(),
                     self.global_frame.width(), self.global_frame.height());
        cr.fill();

        if self.display_grid {
            for gridline in self.grid.iter() {
                gridline.draw(cr);
            }
        }

        if self.display_axes {
            for axis in self.axes.iter() {
                axis.draw(cr);
            }
        }

        for chart in self.charts.iter() {
            chart.draw(cr);
        }

    }
}

/// ## Plot
///
/// Determines a single plot. A plot is part of a figure, and contains a canvas where things are
/// drawn, and some space around the canvas, to make space for labels, ticks, and tick labels.
#[derive(Clone, Debug)]
pub struct Plot {
    //style: Style,
    title: Text,
    color: Rgba,
    local_frame: Frame,
    display_border: bool,
    border_color: Rgba,
    border_width: f64,
    canvas: Canvas,
}

impl Plot {
    /// Create and return a plot
    pub fn new() -> Plot {
        Plot {
            title: Text::new(""),
            color: Rgba::new(230.0/255.0, 231.0/255.0, 242.0/255.0, 0.8),
            local_frame: Frame::from_sides(0.0, 1.0, 0.0, 1.0),
            display_border: true,
            border_color: Rgba::new(0.0, 0.0, 0.0, 1.0),
            border_width: 0.005,
            canvas: Canvas::new(),
        }
    }

    /// Set plot title
    pub fn set_title(&mut self, title: &str) {
        self.title.set_content(title);
    }

    /// Set plot background color. **Note**: This is different from the canvas background color.
    pub fn set_color(&mut self, color: Rgba) {
        self.color = color;
    }

    /// Set local plot coordinates, relative to the figure it belongs to.
    ///
    /// A value of 0.0 is the minimum figure coordinate, and a value of 1.0 is the maximum figure
    /// coordinate.
    pub fn set_local_frame(&mut self, left: f64, right: f64, bottom: f64, top: f64) {
        self.local_frame.set(left, right, bottom, top);
    }

    /// Set the data range of the plot
    ///
    /// *Note*:
    /// This is a soft suggestion, and can be overwritten before the final result for aestethics.
    /// See more [here](struct.Canvas.html#method.set_data_range).
    pub fn set_data_range(&mut self, x_min: f64, x_max: f64, y_min: f64, y_max: f64) {
        self.canvas.set_data_range(x_min, x_max, y_min, y_max);
    }

    /// Set the horisontal data range of the plot
    ///
    /// *Note*:
    /// This is a soft suggestion, and can be overwritten before the final result for aestethics.
    /// See more [here](struct.Canvas.html#method.set_data_range).
    pub fn set_x_range(&mut self, x_min: f64, x_max: f64) {
        self.canvas.set_x_range(x_min, x_max);
    }

    /// Set the vertical data range of the plot
    ///
    /// *Note*:
    /// This is a soft suggestion, and can be overwritten before the final result for aestethics.
    /// See more [here](struct.Canvas.html#method.set_data_range).
    pub fn set_y_range(&mut self, y_min: f64, y_max: f64) {
        self.canvas.set_y_range(y_min, y_max);
    }

    /// Set the left horisontal data range end of the plot
    ///
    /// *Note*:
    /// This is a soft suggestion, and can be overwritten before the final result for aestethics.
    /// See more [here](struct.Canvas.html#method.set_data_range).
    pub fn set_x_min(&mut self, x_min: f64) {
        self.canvas.set_x_min(x_min);
    }

    /// Set the right horisintal data range end of the plot
    ///
    /// *Note*:
    /// This is a soft suggestion, and can be overwritten before the final result for aestethics.
    /// See more [here](struct.Canvas.html#method.set_data_range).
    pub fn set_x_max(&mut self, x_max: f64) {
        self.canvas.set_x_max(x_max);
    }

    /// Set the bottom vertical data range end of the plot
    ///
    /// *Note*:
    /// This is a soft suggestion, and can be overwritten before the final result for aestethics.
    /// See more [here](struct.Canvas.html#method.set_data_range).
    pub fn set_y_min(&mut self, y_min: f64) {
        self.canvas.set_y_min(y_min);
    }

    /// Set the top vertical data range end of the plot
    ///
    /// *Note*:
    /// This is a soft suggestion, and can be overwritten before the final result for aestethics.
    /// See more [here](struct.Canvas.html#method.set_data_range).
    pub fn set_y_max(&mut self, y_max: f64) {
        self.canvas.set_y_max(y_max);
    }

    /// Whether or not to display a border around the plot
    pub fn display_border(&mut self, val: bool) {
        self.display_border = val;
    }

    /// Set the color of the border around the plot
    pub fn set_border_color(&mut self, color: Rgba) {
        self.border_color = color;
    }

    /// Set the line width of the border around the plot
    pub fn set_border_width(&mut self, val: f64) {
        self.border_width = val;
    }

    /// Add a canvas to the plot
    pub fn add(&mut self, chart: Chart) {
        self.canvas.add_chart(chart);
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
    pub fn fit(&mut self) -> Result<(), Error> {
        let scale_factor = self.local_frame.diag_len() / 2f64.sqrt();
        self.scale_size(scale_factor);
        self.canvas.fit(self.local_frame.clone())?;

        Ok(())
    }

    /// Do the actual drawing of the plot
    pub fn draw(&self, cr: &Context) {

        // Background
        cr.set_source_rgba(self.color.red as f64, self.color.green as f64, self.color.blue as f64,
                           self.color.alpha as f64);
        cr.rectangle(self.local_frame.left(), self.local_frame.bottom(),
                     self.local_frame.width(), self.local_frame.height());
        cr.fill();

        if self.display_border {
            cr.set_source_rgba(self.border_color.red as f64, self.border_color.green as f64,
                               self.border_color.blue as f64, self.border_color.alpha as f64);
            cr.set_line_width(self.border_width);
            cr.rectangle(self.local_frame.left(), self.local_frame.bottom(),
                         self.local_frame.width(), self.local_frame.height());
            cr.stroke();
        }

        self.canvas.draw(cr);
    }

}
