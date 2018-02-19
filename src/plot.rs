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
        Plot {
            title: label::Label::new(),
            color: color::Color::new_rgb_u8(240, 242, 255),
            local_frame: local_frame,
            canvas: canvas::Canvas::new(),
        }
    }

    /// Set plot title
    pub fn set_title(mut self, title: &str) -> Self {
        self.title.set_content(title);
        self
    }

    /// Set the background color using the default, built in colors
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

    /// Set the background color from name. See the [palette
    /// documentation](https://docs.rs/palette/0.3.0/palette/named/index.html) for more info.
    pub fn set_color_str(mut self, color_name: &str) -> Result<Self, Error> {
        self.color.set_color_str(color_name)?;
        Ok(self)
    }

    /// Set local plot coordinates, relative to the figure it belongs to.
    ///
    /// A value of 0.0 is the minimum figure coordinate, and a value of 1.0 is the maximum figure
    /// coordinate.
    pub fn set_local_frame(mut self, left: f64, right: f64, bottom: f64, top: f64) -> Self {
        self.local_frame.set(left, right, bottom, top);
        self
    }

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

    /// Whether or not to display a border around the plot
    pub fn display_border(mut self, val: bool) -> Self {
        self.local_frame.display_border(val);
        self
    }

    /// Set the border color using the default, built in colors
    pub fn set_border_color(mut self, color_name: &str) -> Self {
        self.local_frame.set_color(color_name);
        self
    }

    /// Set the border color
    pub fn set_border_color_rgb(mut self, red: f32, green: f32, blue: f32) -> Self {
        self.local_frame.set_color_rgb(red, green, blue);
        self
    }

    /// Set the border color
    pub fn set_border_color_rgba(mut self, red: f32, green: f32, blue: f32, alpha: f32) -> Self {
        self.local_frame.set_color_rgba(red, green, blue, alpha);
        self
    }

    /// Set the border color
    pub fn set_border_color_rgb_u8(mut self, red: u8, green: u8, blue: u8) -> Self {
        self.local_frame.set_color_rgb_u8(red, green, blue);
        self
    }

    /// Set the border color
    pub fn set_border_color_rgba_u8(mut self, red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        self.local_frame.set_color_rgba_u8(red, green, blue, alpha);
        self
    }

    /// Set the border color from name. See the [palette
    /// documentation](https://docs.rs/palette/0.3.0/palette/named/index.html) for more info.
    pub fn set_border_color_str(mut self, color_name: &str) -> Result<Self, Error> {
        self.local_frame.set_color_str(color_name)?;
        Ok(self)
    }

    /// Set the line width of the border around the plot
    pub fn set_border_thickness(mut self, val: f64) -> Self {
        self.local_frame.set_border_thickness(val);
        self
    }

    /// Set the label content on the default horisontal axis
    pub fn set_x_label(mut self, content: &str) -> Self {
        self.canvas.set_default_x_axis_label_content(content);
        self
    }

    /// Set the label content on the default vertical axis
    pub fn set_y_label(mut self, content: &str) -> Self {
        self.canvas.set_default_y_axis_label_content(content);
        self
    }

    /// Set the angle of the label on the default horisontal axis
    pub fn set_x_label_angle(mut self, val: f64) -> Self {
        self.canvas.set_default_x_axis_label_angle(val);
        self
    }

    /// Set the angle of the label on the default vertical axis
    pub fn set_y_label_angle(mut self, val: f64) -> Self {
        self.canvas.set_default_y_axis_label_angle(val);
        self
    }

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
    pub fn fit(&mut self) -> Result<(), Error> {
        let scale_factor = self.local_frame.diag_len();
        self.scale_size(scale_factor);
        self.canvas.fit(&self.local_frame)?;

        Ok(())
    }

    /// Do the actual drawing of the plot
    pub fn draw(&self, cr: &Context, fig_rel_height: f64, fig_rel_width: f64) {

        // Fill background
        let bg_color = self.color.as_rgba();
        cr.set_source_rgba(bg_color.red as f64, bg_color.green as f64, bg_color.blue as f64,
                           bg_color.alpha as f64);
        cr.rectangle(self.local_frame.left(), self.local_frame.bottom(),
                     self.local_frame.width(), self.local_frame.height());
        cr.fill();

        // Draw frame border
        self.local_frame.draw(cr, fig_rel_height, fig_rel_width);



        // Draw canvas
        self.canvas.draw(cr, fig_rel_height, fig_rel_width);
    }

}
