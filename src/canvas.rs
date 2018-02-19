//! Definition of the Canvas struct
//!

use std::f64;
use failure::Error;

use cairo::Context;

use ::{axis, mark, chart, shape, coord, label, color};
use utils::{Drawable, Plottable};


/// ## Canvas
///
/// This is the area of the plot where the data is actually displayed, and it is enclosed by the
/// ``main axes''.
#[derive(Clone, Debug)]
pub struct Canvas {
    color: color::Color,
    local_frame: shape::Rectangle,
    global_frame: shape::Rectangle,
    data_frame: shape::Rectangle,
    user_data_frame: shape::Rectangle,
    display_axes: bool,
    display_grid: bool,
    grid_width: f64,
    grid_color: color::Color,
    grid: Vec<mark::GridLine>,
    hor_marks: Vec<mark::Mark>, // TODO: Use these in stead of axis
    ver_marks: Vec<mark::Mark>,
    axes: Vec<axis::Axis>,
    charts: Vec<chart::Chart>,
    default_x_axis_label: label::Label, // For convenience: plot.set_label() etc.
    default_y_axis_label: label::Label,
}

impl Canvas {
    /// Create and return a new canvas
    pub fn new() -> Canvas {
        let mut x_axis_label = label::Label::new_from_centroid(0.5, -0.11);
        x_axis_label.set_font_size(0.025);
        let mut y_axis_label = label::Label::new_from_centroid(-0.13, 0.5);
        y_axis_label.set_font_size(0.025);
        y_axis_label.set_angle(f64::consts::PI / 2.0);
        Canvas {
            color: color::Color::new_rgb_u8(230, 235, 245),
            local_frame: shape::Rectangle::new_from(0.15, 0.95, 0.15, 0.95),
            global_frame: shape::Rectangle::new(),
            data_frame: shape::Rectangle::new(),
            user_data_frame: shape::Rectangle::new(),
            display_axes: true,
            display_grid: true,
            grid_width: 0.0025,
            grid_color: color::Color::new_rgba(1.0, 1.0, 1.0, 0.9),
            grid: Vec::<mark::GridLine>::new(),
            hor_marks: Vec::<mark::Mark>::new(),
            ver_marks: Vec::<mark::Mark>::new(),
            axes: Vec::<axis::Axis>::new(),
            charts: Vec::<chart::Chart>::new(),
            default_x_axis_label: x_axis_label,
            default_y_axis_label: y_axis_label,
        }
    }

    /// Set the canvas background color using the default, built in colors
    pub fn set_color(&mut self, color_name: &str) {
        self.color.set_color_default(color_name);
    }

    /// Set the canvas background color
    pub fn set_color_rgb(&mut self, red: f32, green: f32, blue: f32) {
        self.color.set_color_rgb(red, green, blue);
    }

    /// Set the canvas background color
    pub fn set_color_rgba(&mut self, red: f32, green: f32, blue: f32, alpha: f32) {
        self.color.set_color_rgba(red, green, blue, alpha);
    }

    /// Set the canvas background color
    pub fn set_color_rgb_u8(&mut self, red: u8, green: u8, blue: u8) {
        self.color.set_color_rgb_u8(red, green, blue);
    }

    /// Set the canvas background color
    pub fn set_color_rgba_u8(&mut self, red: u8, green: u8, blue: u8, alpha: u8) {
        self.color.set_color_rgba_u8(red, green, blue, alpha);
    }

    /// Set the canvas background color from name. See the [palette
    /// documentation](https://docs.rs/palette/0.3.0/palette/named/index.html) for more info.
    pub fn set_color_str(&mut self, color_name: &str) -> Result<(), Error> {
        self.color.set_color_str(color_name)?;
        Ok(())
    }

    /// Set local frame coordinates.
    pub fn set_local_frame(&mut self, frame: shape::Rectangle) {
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

    /// Set the label content of the default horisontal axis
    pub fn set_default_x_axis_label_content(&mut self, content: &str) {
        self.default_x_axis_label.set_content(content);
    }

    /// Set the label content of the default vertical axis
    pub fn set_default_y_axis_label_content(&mut self, content: &str) {
        self.default_y_axis_label.set_content(content);
    }

    /// Set the label content of the default horisontal axis
    pub fn set_default_x_axis_label_angle(&mut self, val: f64) {
        self.default_x_axis_label.set_angle(val);
    }

    /// Set the label content of the default vertical axis
    pub fn set_default_y_axis_label_angle(&mut self, val: f64) {
        self.default_y_axis_label.set_angle(val);
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

    pub fn set_grid_color(&mut self, color_name: &str) {
        self.grid_color.set_color_default(color_name);
    }

    pub fn set_grid_color_rgb(&mut self, red: f32, green: f32, blue: f32) {
        self.grid_color.set_color_rgb(red, green, blue);
    }

    pub fn set_grid_color_rgba(&mut self, red: f32, green: f32, blue: f32, alpha: f32) {
        self.grid_color.set_color_rgba(red, green, blue, alpha);
    }

    pub fn set_grid_color_rgb_u8(&mut self, red: u8, green: u8, blue: u8) {
        self.grid_color.set_color_rgb_u8(red, green, blue);
    }

    pub fn set_grid_color_rgba_u8(&mut self, red: u8, green: u8, blue: u8, alpha: u8) {
        self.grid_color.set_color_rgba_u8(red, green, blue, alpha);
    }

    pub fn set_grid_color_str(&mut self, color_name: &str) -> Result<(), Error> {
        self.grid_color.set_color_str(color_name)?;
        Ok(())
    }

    /// Add an additional axis to the canvas
    pub fn add_axis(&mut self, axis: axis::Axis) {
        self.axes.push(axis);
    }

    /// Add an additional chart to the canvas
    pub fn add_chart(&mut self, chart: chart::Chart) {
        self.charts.push(chart);
    }

    /// Compute grid lines given a vertical and a horisontal axis
    fn compute_grid(&mut self, ver_axis: &axis::Axis, hor_axis: &axis::Axis) {
        let scale_factor = self.global_frame.diag_len();
        //let scale_factor = self.global_frame.height().min(self.global_frame.width());
        let grid_color = self.grid_color.as_srgba();
        for coord in ver_axis.mark_coords() {
            let mut gridline = mark::GridLine::new_from(coord.clone(),
                                                        coord::Coord::new_from(self.global_frame.right(), coord.y()));
            gridline.set_color_rgba(grid_color.red, grid_color.green, grid_color.blue,
                                    grid_color.alpha);
            gridline.set_width(self.grid_width);
            gridline.scale_size(scale_factor);
            self.grid.push(gridline);
        }
        for coord in hor_axis.mark_coords() {
            let mut gridline = mark::GridLine::new_from(coord.clone(),
                                                        coord::Coord::new_from(coord.x(), self.global_frame.top()));
            gridline.set_color_rgba(grid_color.red, grid_color.green, grid_color.blue,
                                    grid_color.alpha);
            gridline.set_width(self.grid_width);
            gridline.scale_size(scale_factor);
            self.grid.push(gridline);
        }
    }

    /// Find the smallest data frame including all data points from all charts
    fn find_largest_chart_data_frame(&self) -> Option<shape::Rectangle> {
        if self.charts.len() == 0 { return None }
        let mut largest_data_frame = shape::Rectangle::new_from(f64::MAX, f64::MIN, f64::MAX, f64::MIN);
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
    fn compute_data_frame(&self) -> shape::Rectangle {
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
    fn set_default_axes(&mut self, data_frame: shape::Rectangle) -> Result<(axis::Axis, axis::Axis), Error> {
        let mut hor_axis = axis::Axis::new_from(coord::Coord::new_from(0.0, 0.0), coord::Coord::new_from(1.0, 0.0));
        hor_axis.set_data_range(data_frame.left(), data_frame.right());
        hor_axis.compute_marks()?;

        hor_axis.set_positive_tick_length(0.0);
        hor_axis.set_negative_tick_length(0.01);
        hor_axis.set_tick_label_font_size(0.02);
        hor_axis.set_tick_label_frame_gaps(0.0, 0.0, 0.0, 0.0);
        hor_axis.set_tick_label_offset(-0.05);

        hor_axis.set_label(&self.default_x_axis_label);

        let mut ver_axis = axis::Axis::new_from(coord::Coord::new_from(0.0, 0.0), coord::Coord::new_from(0.0, 1.0));
        ver_axis.set_data_range(data_frame.bottom(), data_frame.top());
        ver_axis.compute_marks()?;

        ver_axis.set_positive_tick_length(0.0);
        ver_axis.set_negative_tick_length(0.01);
        ver_axis.set_tick_label_font_size(0.02);
        ver_axis.set_tick_label_frame_gaps(0.0, 0.0, 0.0, 0.0);
        ver_axis.set_tick_label_offset(-0.05);

        ver_axis.set_label(&self.default_y_axis_label);

        Ok((hor_axis, ver_axis))
    }

    /// Fit this canvas to its plot
    pub fn fit(&mut self, plot_frame: &shape::Rectangle)
    -> Result<(), Error> {
        // First, we update the global_frame relative to the parent's global_frame.
        // After this is called, both local_frame and global_frame should not be altered.
        self.local_frame.scale_size(plot_frame.diag_len()); //JIC we want to display the border
        self.global_frame = self.local_frame.relative_to(plot_frame);

        // Second, we update the data_frame
        let data_frame = self.compute_data_frame();

        // Then we compute one horizontal and one vertical axis.
        let (mut hor_axis, mut ver_axis) = self.set_default_axes(data_frame)?;

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
        self.data_frame = shape::Rectangle::new_from(data_left, data_right, data_bottom, data_top);

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
    pub fn draw(&self, cr: &Context, fig_rel_height: f64, fig_rel_width: f64) {

        // Background
        let bg_color = self.color.as_srgba();
        cr.set_source_rgba(bg_color.red as f64, bg_color.green as f64, bg_color.blue as f64,
                           bg_color.alpha as f64);
        cr.rectangle(self.global_frame.left(), self.global_frame.bottom(),
                     self.global_frame.width(), self.global_frame.height());
        cr.fill();

        if self.display_grid {
            for gridline in self.grid.iter() {
                gridline.draw(cr, fig_rel_height, fig_rel_width);
            }
        }

        if self.display_axes {
            for axis in self.axes.iter() {
                axis.draw(cr, fig_rel_height, fig_rel_width);
            }
        }

        let mut color_generator = color::DefaultColors::new();
        for chart in self.charts.clone().iter_mut() {
            if !chart.is_color_updated() {
                let color = color_generator.next();
                chart.set_color_internal(color.expect("Something wrong"));
            }
            chart.draw(cr, fig_rel_height, fig_rel_width);
        }

    }
}

