//! Definition of the Plot struct
//!

use std::f64;
use failure::Error;

use cairo::Context;

use ::{canvas, chart, shape, label, color};


/// Determines a single plot. A plot is part of a figure, and contains a canvas where things are
/// drawn. By default, there is some space around the canvas, to make space for labels, ticks,
/// tick labels and plot title.
#[derive(Clone, Debug)]
pub struct Plot {
    title: label::Label,
    color: color::Color,
    local_frame: shape::Rectangle,
    canvas: canvas::Canvas,
}

impl Plot {
    /// Create and return a plot
    pub fn new() -> Plot {
        let mut local_frame = shape::Rectangle::new();
        local_frame.display_border(true);
        local_frame.set_border_thickness(0.001);
        local_frame.set_color_internal(color::Color::new_default("plot_border").as_srgba());
        let mut title = label::Label::new();
        title.set_color_internal(color::Color::new_default("plot_title").as_srgba());
        title.set_centroid(0.5, 0.97);
        title.set_font_size(0.02);
        Plot {
            title: title,
            color: color::Color::new_default("plot_background"),
            local_frame: local_frame,
            canvas: canvas::Canvas::new(),
        }
    }

    // ----------------- PLOT TITLE ---------------------------------------- //

    /// Set plot title
    pub fn set_title(mut self, title: &str) -> Self {
        self.title.set_content(title);
        self
    }

    /// Set plot title font size
    pub fn set_title_font_size(mut self, val: f64) -> Self {
        self.title.set_font_size(val);
        self
    }

    /// Set the angle of the plot title
    pub fn set_title_angle(mut self, val: f64) -> Self {
        self.title.set_angle(val);
        self
    }

    /// Set the location of the plot title, relative to the plot frame
    pub fn set_title_centroid(mut self, x_coord: f64, y_coord: f64) -> Self {
        self.title.set_centroid(x_coord, y_coord);
        self
    }

    /// Set gaps around plot title.
    ///
    /// NOTE: This has currently no visible effect
    pub fn set_title_frame_gaps(mut self, left: f64, right: f64, bottom: f64, top: f64) -> Self {
        self.title.set_frame_gaps(left, right, bottom, top);
        self
    }

    /// Set the title color
    pub fn set_title_color(mut self, color_name: &str) -> Self {
        let color = color::Color::new_default(color_name);
        self.title.set_color_internal(color.as_srgba());
        self
    }

    /// Set the title color
    pub fn set_title_color_rgb(mut self, red: f32, green: f32, blue: f32) -> Self {
        let color = color::Color::new_rgb(red, green, blue);
        self.title.set_color_internal(color.as_srgba());
        self
    }

    /// Set the title color
    pub fn set_title_color_rgba(mut self, red: f32, green: f32, blue: f32, alpha: f32) -> Self {
        let color = color::Color::new_rgba(red, green, blue, alpha);
        self.title.set_color_internal(color.as_srgba());
        self
    }

    /// Set the title color
    pub fn set_title_color_rgb_u8(mut self, red: u8, green: u8, blue: u8) -> Self {
        let color = color::Color::new_rgb_u8(red, green, blue);
        self.title.set_color_internal(color.as_srgba());
        self
    }

    /// Set the title color
    pub fn set_title_color_rgba_u8(mut self, red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        let color = color::Color::new_rgba_u8(red, green, blue, alpha);
        self.title.set_color_internal(color.as_srgba());
        self
    }

    /// Set the title color
    pub fn set_title_color_str(mut self, color_name: &str) -> Self {
        let color = color::Color::new_str(color_name);
        self.title.set_color_internal(color.as_srgba());
        self
    }

    // ----------------- PLOT APPEARANCE ----------------------------------- //

    /// Set the background color
    pub fn set_color(mut self, color_name: &str) -> Self {
        self.color.set_color_default(color_name);
        self
    }

    /// Set the background color
    pub fn set_color_rgb(mut self, red: f32, green: f32, blue: f32) -> Self {
        self.color.set_color_rgb(red, green, blue);
        self
    }

    /// Set the background color
    pub fn set_color_rgba(mut self, red: f32, green: f32, blue: f32, alpha: f32) -> Self {
        self.color.set_color_rgba(red, green, blue, alpha);
        self
    }

    /// Set the background color
    pub fn set_color_rgb_u8(mut self, red: u8, green: u8, blue: u8) -> Self {
        self.color.set_color_rgb_u8(red, green, blue);
        self
    }

    /// Set the background color
    pub fn set_color_rgba_u8(mut self, red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        self.color.set_color_rgba_u8(red, green, blue, alpha);
        self
    }

    /// Set the background color
    pub fn set_color_str(mut self, color_name: &str) -> Self {
        self.color.set_color_str(color_name);
        self
    }

    // ----------------- PLOT FRAME ---------------------------------------- //

    /// Set local plot coordinates, relative to the figure it belongs to.
    ///
    /// A value of 0.0 is the minimum figure coordinate, and a value of 1.0 is the maximum figure
    /// coordinate.
    pub fn set_local_frame(mut self, left: f64, right: f64, bottom: f64, top: f64) -> Self {
        self.local_frame.set(left, right, bottom, top);
        self
    }

    pub fn set_left(mut self, val: f64) -> Self {
        self.local_frame.set_left(val);
        self
    }

    pub fn set_right(mut self, val: f64) -> Self {
        self.local_frame.set_right(val);
        self
    }

    pub fn set_bottom(mut self, val: f64) -> Self {
        self.local_frame.set_bottom(val);
        self
    }

    pub fn set_top(mut self, val: f64) -> Self {
        self.local_frame.set_top(val);
        self
    }

    pub fn set_local_frame_mut_ref(&mut self, left: f64, right: f64, bottom: f64, top: f64) {
        self.local_frame.set(left, right, bottom, top);
    }

    pub fn set_left_mut_ref(&mut self, val: f64) {
        self.local_frame.set_left(val);
    }

    pub fn set_right_mut_ref(&mut self, val: f64) {
        self.local_frame.set_right(val);
    }

    pub fn set_bottom_mut_ref(&mut self, val: f64) {
        self.local_frame.set_bottom(val);
    }

    pub fn set_top_mut_ref(&mut self, val: f64) {
        self.local_frame.set_top(val);
    }


    /// Whether or not to display a border around the plot
    pub fn display_border(mut self, val: bool) -> Self {
        self.local_frame.display_border(val);
        self
    }

    /// Set the border color
    pub fn set_border_color(mut self, color_name: &str) -> Self {
        let color = color::Color::new_default(color_name);
        self.local_frame.set_color_internal(color.as_srgba());
        self
    }

    /// Set the border color
    pub fn set_border_color_rgb(mut self, red: f32, green: f32, blue: f32) -> Self {
        let color = color::Color::new_rgb(red, green, blue);
        self.local_frame.set_color_internal(color.as_srgba());
        self
    }

    /// Set the border color
    pub fn set_border_color_rgba(mut self, red: f32, green: f32, blue: f32, alpha: f32) -> Self {
        let color = color::Color::new_rgba(red, green, blue, alpha);
        self.local_frame.set_color_internal(color.as_srgba());
        self
    }

    /// Set the border color
    pub fn set_border_color_rgb_u8(mut self, red: u8, green: u8, blue: u8) -> Self {
        let color = color::Color::new_rgb_u8(red, green, blue);
        self.local_frame.set_color_internal(color.as_srgba());
        self
    }

    /// Set the border color
    pub fn set_border_color_rgba_u8(mut self, red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        let color = color::Color::new_rgba_u8(red, green, blue, alpha);
        self.local_frame.set_color_internal(color.as_srgba());
        self
    }

    /// Set the border color
    pub fn set_border_color_str(mut self, color_name: &str) -> Self {
        let color = color::Color::new_str(color_name);
        self.local_frame.set_color_internal(color.as_srgba());
        self
    }

    /// Set the line width of the border around the plot
    pub fn set_border_thickness(mut self, val: f64) -> Self {
        self.local_frame.set_border_thickness(val);
        self
    }

    // ----------------- DATA RANGE ---------------------------------------- //

    /// Set the data range of the plot
    ///
    /// *Note*:
    /// This is a soft suggestion, and can be overwritten before the final result for aestethics.
    /// See more [here](struct.Canvas.html#method.set_data_range).
    pub fn set_data_range(mut self, x_min: f64, x_max: f64, y_min: f64, y_max: f64) -> Self {
        self.canvas.set_data_range(x_min, x_max, y_min, y_max);
        self
    }

    /// Set the horisontal data range of the plot
    ///
    /// *Note*:
    /// This is a soft suggestion, and can be overwritten before the final result for aestethics.
    /// See more [here](struct.Canvas.html#method.set_data_range).
    pub fn set_x_range(mut self, x_min: f64, x_max: f64) -> Self {
        self.canvas.set_x_range(x_min, x_max);
        self
    }

    /// Set the vertical data range of the plot
    ///
    /// *Note*:
    /// This is a soft suggestion, and can be overwritten before the final result for aestethics.
    /// See more [here](struct.Canvas.html#method.set_data_range).
    pub fn set_y_range(mut self, y_min: f64, y_max: f64) -> Self {
        self.canvas.set_y_range(y_min, y_max);
        self
    }

    /// Set the left horisontal data range end of the plot
    ///
    /// *Note*:
    /// This is a soft suggestion, and can be overwritten before the final result for aestethics.
    /// See more [here](struct.Canvas.html#method.set_data_range).
    pub fn set_x_min(mut self, x_min: f64) -> Self {
        self.canvas.set_x_min(x_min);
        self
    }

    /// Set the right horisontal data range end of the plot
    ///
    /// *Note*:
    /// This is a soft suggestion, and can be overwritten before the final result for aestethics.
    /// See more [here](struct.Canvas.html#method.set_data_range).
    pub fn set_x_max(mut self, x_max: f64) -> Self {
        self.canvas.set_x_max(x_max);
        self
    }

    /// Set the bottom vertical data range end of the plot
    ///
    /// *Note*:
    /// This is a soft suggestion, and can be overwritten before the final result for aestethics.
    /// See more [here](struct.Canvas.html#method.set_data_range).
    pub fn set_y_min(mut self, y_min: f64) -> Self {
        self.canvas.set_y_min(y_min);
        self
    }

    /// Set the top vertical data range end of the plot
    ///
    /// *Note*:
    /// This is a soft suggestion, and can be overwritten before the final result for aestethics.
    /// See more [here](struct.Canvas.html#method.set_data_range).
    pub fn set_y_max(mut self, y_max: f64) -> Self {
        self.canvas.set_y_max(y_max);
        self
    }

    // ----------------- CANVAS -------------------------------------------- //

    /// Set local frame coordinates of the canvas (relative to its plot).
    pub fn set_canvas_local_frame(mut self, left: f64, right: f64, bottom: f64, top: f64) -> Self {
        self.canvas.set_local_frame(left, right, bottom, top);
        self
    }

    /// Set the canvas background color
    pub fn set_canvas_color(mut self, color_name: &str) -> Self {
        let color = color::Color::new_default(color_name);
        self.canvas.set_color_internal(color.as_srgba());
        self
    }

    /// Set the canvas background color
    pub fn set_canvas_color_rgb(mut self, red: f32, green: f32, blue: f32) -> Self {
        let color = color::Color::new_rgb(red, green, blue);
        self.canvas.set_color_internal(color.as_srgba());
        self
    }

    /// Set the canvas background color
    pub fn set_canvas_color_rgba(mut self, red: f32, green: f32, blue: f32, alpha: f32) -> Self {
        let color = color::Color::new_rgba(red, green, blue, alpha);
        self.canvas.set_color_internal(color.as_srgba());
        self
    }

    /// Set the canvas background color
    pub fn set_canvas_color_rgb_u8(mut self, red: u8, green: u8, blue: u8) -> Self {
        let color = color::Color::new_rgb_u8(red, green, blue);
        self.canvas.set_color_internal(color.as_srgba());
        self
    }

    /// Set the canvas background color
    pub fn set_canvas_color_rgba_u8(mut self, red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        let color = color::Color::new_rgba_u8(red, green, blue, alpha);
        self.canvas.set_color_internal(color.as_srgba());
        self
    }

    /// Set the canvas background color
    pub fn set_canvas_color_str(mut self, color_name: &str) -> Self {
        let color = color::Color::new_str(color_name);
        self.canvas.set_color_internal(color.as_srgba());
        self
    }

    // ----------------- AXES ---------------------------------------------- //

    /// Whether or not to display the default horizontal axis
    pub fn display_x_axis(mut self, val: bool) -> Self {
        self.canvas.display_horizontal_axis(val);
        self
    }

    /// Whether or not to display the default vertical axis
    pub fn display_y_axis(mut self, val: bool) -> Self {
        self.canvas.display_vertical_axis(val);
        self
    }

    /// Set the line width of all axes
    pub fn set_axes_line_width(mut self, val: f64) -> Self {
        self.canvas.set_axes_line_width(val);
        self
    }

    /// Set the font size of all axis labels
    pub fn set_axes_label_font_size(mut self, val: f64) -> Self {
        self.canvas.set_axes_label_font_size(val);
        self
    }

    /// Set the axes color
    pub fn set_axes_color(mut self, color_name: &str) -> Self {
        let color = color::Color::new_default(color_name);
        self.canvas.set_axes_color_internal(color.as_srgba());
        self
    }

    /// Set the axes color
    pub fn set_axes_color_rgb(mut self, red: f32, green: f32, blue: f32) -> Self {
        let color = color::Color::new_rgb(red, green, blue);
        self.canvas.set_axes_color_internal(color.as_srgba());
        self
    }

    /// Set the axes color
    pub fn set_axes_color_rgba(mut self, red: f32, green: f32, blue: f32, alpha: f32) -> Self {
        let color = color::Color::new_rgba(red, green, blue, alpha);
        self.canvas.set_axes_color_internal(color.as_srgba());
        self
    }

    /// Set the axes color
    pub fn set_axes_color_rgb_u8(mut self, red: u8, green: u8, blue: u8) -> Self {
        let color = color::Color::new_rgb_u8(red, green, blue);
        self.canvas.set_axes_color_internal(color.as_srgba());
        self
    }

    /// Set the axes color
    pub fn set_axes_color_rgba_u8(mut self, red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        let color = color::Color::new_rgba_u8(red, green, blue, alpha);
        self.canvas.set_axes_color_internal(color.as_srgba());
        self
    }

    /// Set the axes color
    pub fn set_axes_color_str(mut self, color_name: &str) -> Self {
        let color = color::Color::new_str(color_name);
        self.canvas.set_axes_color_internal(color.as_srgba());
        self
    }

    // ----------------- DEFAULT HORISONTAL AXIS --------------------------- //

    /// Set the label content on the default horisontal axis
    pub fn set_x_label(mut self, content: &str) -> Self {
        self.canvas.set_default_x_axis_label_content(content);
        self
    }

    /// Set the angle of the label on the default horisontal axis
    pub fn set_x_label_angle(mut self, val: f64) -> Self {
        self.canvas.set_default_x_axis_label_angle(val);
        self
    }

    /// Set the center location of the label on the default horisontal axis
    pub fn set_x_label_centroid(mut self, x_coord: f64, y_coord: f64) -> Self {
        self.canvas.set_default_x_axis_label_centroid(x_coord, y_coord);
        self
    }

    /// Set the frame gaps around the label of the default horisontal axis
    pub fn set_x_label_frame_gaps(mut self, left: f64, right: f64, bottom: f64, top: f64) -> Self {
        self.canvas.set_default_x_axis_label_frame_gaps(left, right, bottom, top);
        self
    }

    /// Set the x label color
    pub fn set_x_label_color(mut self, color_name: &str) -> Self {
        let color = color::Color::new_default(color_name);
        self.canvas.set_default_x_axis_label_color_internal(color.as_srgba());
        self
    }

    /// Set the x label color
    pub fn set_x_label_color_rgb(mut self, red: f32, green: f32, blue: f32) -> Self {
        let color = color::Color::new_rgb(red, green, blue);
        self.canvas.set_default_x_axis_label_color_internal(color.as_srgba());
        self
    }

    /// Set the x label color
    pub fn set_x_label_color_rgba(mut self, red: f32, green: f32, blue: f32, alpha: f32) -> Self {
        let color = color::Color::new_rgba(red, green, blue, alpha);
        self.canvas.set_default_x_axis_label_color_internal(color.as_srgba());
        self
    }

    /// Set the x label color
    pub fn set_x_label_color_rgb_u8(mut self, red: u8, green: u8, blue: u8) -> Self {
        let color = color::Color::new_rgb_u8(red, green, blue);
        self.canvas.set_default_x_axis_label_color_internal(color.as_srgba());
        self
    }

    /// Set the x label color
    pub fn set_x_label_color_rgba_u8(mut self, red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        let color = color::Color::new_rgba_u8(red, green, blue, alpha);
        self.canvas.set_default_x_axis_label_color_internal(color.as_srgba());
        self
    }

    /// Set the x label color
    pub fn set_x_label_color_str(mut self, color_name: &str) -> Self {
        let color = color::Color::new_str(color_name);
        self.canvas.set_default_x_axis_label_color_internal(color.as_srgba());
        self
    }

    // ----------------- DEFAULT VERTICAL AXIS ----------------------------- //

    /// Set the label content on the default vertical axis
    pub fn set_y_label(mut self, content: &str) -> Self {
        self.canvas.set_default_y_axis_label_content(content);
        self
    }

    /// Set the angle of the label on the default vertical axis
    pub fn set_y_label_angle(mut self, val: f64) -> Self {
        self.canvas.set_default_y_axis_label_angle(val);
        self
    }

    /// Set the center location of the label on the default vertical axis
    pub fn set_y_label_centroid(mut self, x_coord: f64, y_coord: f64) -> Self {
        self.canvas.set_default_y_axis_label_centroid(x_coord, y_coord);
        self
    }

    /// Set the frame gaps around the label of the default vertical axis
    pub fn set_y_label_frame_gaps(mut self, left: f64, right: f64, bottom: f64, top: f64) -> Self {
        self.canvas.set_default_y_axis_label_frame_gaps(left, right, bottom, top);
        self
    }

    /// Set the y label color
    pub fn set_y_label_color(mut self, color_name: &str) -> Self {
        let color = color::Color::new_default(color_name);
        self.canvas.set_default_y_axis_label_color_internal(color.as_srgba());
        self
    }

    /// Set the y label color
    pub fn set_y_label_color_rgb(mut self, red: f32, green: f32, blue: f32) -> Self {
        let color = color::Color::new_rgb(red, green, blue);
        self.canvas.set_default_y_axis_label_color_internal(color.as_srgba());
        self
    }

    /// Set the y label color
    pub fn set_y_label_color_rgba(mut self, red: f32, green: f32, blue: f32, alpha: f32) -> Self {
        let color = color::Color::new_rgba(red, green, blue, alpha);
        self.canvas.set_default_y_axis_label_color_internal(color.as_srgba());
        self
    }

    /// Set the y label color
    pub fn set_y_label_color_rgb_u8(mut self, red: u8, green: u8, blue: u8) -> Self {
        let color = color::Color::new_rgb_u8(red, green, blue);
        self.canvas.set_default_y_axis_label_color_internal(color.as_srgba());
        self
    }

    /// Set the y label color
    pub fn set_y_label_color_rgba_u8(mut self, red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        let color = color::Color::new_rgba_u8(red, green, blue, alpha);
        self.canvas.set_default_y_axis_label_color_internal(color.as_srgba());
        self
    }

    /// Set the y label color
    pub fn set_y_label_color_str(mut self, color_name: &str) -> Self {
        let color = color::Color::new_str(color_name);
        self.canvas.set_default_y_axis_label_color_internal(color.as_srgba());
        self
    }

    // ----------------- TICKS --------------------------------------------- //

    /// Set the tick color
    pub fn set_tick_color(mut self, color_name: &str) -> Self {
        let color = color::Color::new_default(color_name);
        self.canvas.set_tick_color_internal(color.as_srgba());
        self
    }

    /// Set the tick color
    pub fn set_tick_rgb(mut self, red: f32, green: f32, blue: f32) -> Self {
        let color = color::Color::new_rgb(red, green, blue);
        self.canvas.set_tick_color_internal(color.as_srgba());
        self
    }

    /// Set the tick color
    pub fn set_tick_color_rgba(mut self, red: f32, green: f32, blue: f32, alpha: f32) -> Self {
        let color = color::Color::new_rgba(red, green, blue, alpha);
        self.canvas.set_tick_color_internal(color.as_srgba());
        self
    }

    /// Set the y label color
    pub fn set_tick_color_rgb_u8(mut self, red: u8, green: u8, blue: u8) -> Self {
        let color = color::Color::new_rgb_u8(red, green, blue);
        self.canvas.set_tick_color_internal(color.as_srgba());
        self
    }

    /// Set the tick color
    pub fn set_tick_color_rgba_u8(mut self, red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        let color = color::Color::new_rgba_u8(red, green, blue, alpha);
        self.canvas.set_tick_color_internal(color.as_srgba());
        self
    }

    /// Set the tick color
    pub fn set_tick_color_str(mut self, color_name: &str) -> Self {
        let color = color::Color::new_str(color_name);
        self.canvas.set_tick_color_internal(color.as_srgba());
        self
    }

    /// Set the tick label color
    pub fn set_tick_label_color(mut self, color_name: &str) -> Self {
        let color = color::Color::new_default(color_name);
        self.canvas.set_tick_label_color_internal(color.as_srgba());
        self
    }

    /// Set the tick label color
    pub fn set_tick_label_color_rgb(mut self, red: f32, green: f32, blue: f32) -> Self {
        let color = color::Color::new_rgb(red, green, blue);
        self.canvas.set_tick_label_color_internal(color.as_srgba());
        self
    }

    /// Set the tick label color
    pub fn set_tick_label_color_rgba(mut self, red: f32, green: f32, blue: f32, alpha: f32) -> Self {
        let color = color::Color::new_rgba(red, green, blue, alpha);
        self.canvas.set_tick_label_color_internal(color.as_srgba());
        self
    }

    /// Set the tick label color
    pub fn set_tick_label_color_rgb_u8(mut self, red: u8, green: u8, blue: u8) -> Self {
        let color = color::Color::new_rgb_u8(red, green, blue);
        self.canvas.set_tick_label_color_internal(color.as_srgba());
        self
    }

    /// Set the tick label color
    pub fn set_tick_label_color_rgba_u8(mut self, red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        let color = color::Color::new_rgba_u8(red, green, blue, alpha);
        self.canvas.set_tick_label_color_internal(color.as_srgba());
        self
    }

    /// Set the tick label color
    pub fn set_tick_label_color_str(mut self, color_name: &str) -> Self {
        let color = color::Color::new_str(color_name);
        self.canvas.set_tick_label_color_internal(color.as_srgba());
        self
    }

    /// Set the tick font size
    pub fn set_tick_label_font_size(mut self, val: f64) -> Self {
        self.canvas.set_tick_label_font_size(val);
        self
    }

    // ----------------- CANVAS GRID --------------------------------------- //

    /// Whether or not to display horizontal gridlines
    pub fn display_horizontal_gridlines(mut self, val: bool) -> Self {
        self.canvas.display_horizontal_gridlines(val);
        self
    }

    /// Whether or not to display vertical gridlines
    pub fn display_vertical_gridlines(mut self, val: bool) -> Self {
        self.canvas.display_vertical_gridlines(val);
        self
    }

    /// Set the line width of the gridlines
    pub fn set_gridline_width(mut self, val: f64) -> Self {
        self.canvas.set_gridline_width(val);
        self
    }

    /// Set the grid line color
    pub fn set_grid_color(mut self, color_name: &str) -> Self {
        let color = color::Color::new_default(color_name);
        self.canvas.set_grid_color_internal(color.as_srgba());
        self
    }

    /// Set the grid line color
    pub fn set_grid_color_rgb(mut self, red: f32, green: f32, blue: f32) -> Self {
        let color = color::Color::new_rgb(red, green, blue);
        self.canvas.set_grid_color_internal(color.as_srgba());
        self
    }

    /// Set the grid line color
    pub fn set_grid_color_rgba(mut self, red: f32, green: f32, blue: f32, alpha: f32) -> Self {
        let color = color::Color::new_rgba(red, green, blue, alpha);
        self.canvas.set_grid_color_internal(color.as_srgba());
        self
    }

    /// Set the grid line color
    pub fn set_grid_color_rgb_u8(mut self, red: u8, green: u8, blue: u8) -> Self {
        let color = color::Color::new_rgb_u8(red, green, blue);
        self.canvas.set_grid_color_internal(color.as_srgba());
        self
    }

    /// Set the grid line color
    pub fn set_grid_color_rgba_u8(mut self, red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        let color = color::Color::new_rgba_u8(red, green, blue, alpha);
        self.canvas.set_grid_color_internal(color.as_srgba());
        self
    }

    /// Set the grid line color
    pub fn set_grid_color_str(mut self, color_name: &str) -> Self {
        let color = color::Color::new_str(color_name);
        self.canvas.set_grid_color_internal(color.as_srgba());
        self
    }

    // ----------------- GETTERS ------------------------------------------- //

    /// Return the frame of the plot, relative to the figure
    pub fn local_frame(&self) -> shape::Rectangle {
        self.local_frame.clone()
    }

    /// Return the height of the plot, relative to the figure
    pub fn height(&self) -> f64 {
        self.local_frame.height()
    }

    /// Return the width of the figure, relative to the figure
    pub fn width(&self) -> f64 {
        self.local_frame.width()
    }

    /// Return the leftmost plot location, relative to the figure
    pub fn left(&self) -> f64 {
        self.local_frame.left()
    }

    /// Return the rightmost plot location, relative to the figure
    pub fn right(&self) -> f64 {
        self.local_frame.right()
    }

    /// Return the bottommost plot location, relative to the figure
    pub fn bottom(&self) -> f64 {
        self.local_frame.bottom()
    }

    /// Return the topmost plot location, relative to the figure
    pub fn top(&self) -> f64 {
        self.local_frame.top()
    }

    // ----------------- GENERAL ------------------------------------------- //

    /// Add a chart to the plot
    pub fn add(mut self, chart: &chart::Chart) -> Self {
        self.canvas.add_chart(chart.clone());
        self
    }

    fn scale_size(&mut self, factor: f64) {
        self.local_frame.scale_size(factor);
        self.title.scale_size(factor);
    }

    /// This method is called by figure after all plots are added, and all plot adjustment is made.
    /// This happend right before the plot is drawn on the figure.
    ///
    /// The function scales various elements within the plot, and calls a similar function for its
    /// canvas.
    pub(crate) fn fit(&mut self) -> Result<(), Error> {
        let scale_factor = self.local_frame.diag_len();
        self.scale_size(scale_factor);

        self.title.fit(&self.local_frame);

        let has_title = self.title.content() != "";
        self.canvas.fit(&self.local_frame, has_title)?;

        Ok(())
    }

    /// Do the actual drawing of the plot
    pub(crate) fn draw(&self, cr: &Context, fig_rel_height: f64, fig_rel_width: f64) {

        // Fill background
        let bg_color = self.color.as_srgba();
        cr.set_source_rgba(bg_color.red as f64, bg_color.green as f64, bg_color.blue as f64,
                           bg_color.alpha as f64);
        cr.rectangle(self.local_frame.left(), self.local_frame.bottom(),
                     self.local_frame.width(), self.local_frame.height());
        cr.fill();

        // Draw frame border
        self.local_frame.draw(cr, fig_rel_height, fig_rel_width);

        // Draw title
        self.title.draw(cr, fig_rel_height, fig_rel_width);

        // Draw canvas
        self.canvas.draw(cr, fig_rel_height, fig_rel_width);
    }

}
