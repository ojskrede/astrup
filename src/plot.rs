//! Definition of the Plot struct
//!

use std::f64;
use failure::Error;

use cairo::Context;
use palette::Rgba;

use ::{canvas, chart, shape, text};


/// ## Plot
///
/// Determines a single plot. A plot is part of a figure, and contains a canvas where things are
/// drawn. By default, there is some space around the canvas, to make space for labels, ticks, and
/// tick labels.
#[derive(Clone, Debug)]
pub struct Plot {
    title: text::Text,
    color: Rgba,
    local_frame: shape::Rectangle,
    canvas: canvas::Canvas,
}

impl Plot {
    /// Create and return a plot
    pub fn new() -> Plot {
        let mut local_frame = shape::Rectangle::new();
        local_frame.display_border(true);
        local_frame.set_thickness(0.001);
        Plot {
            title: text::Text::new(""),
            color: Rgba::new(240.0/255.0, 242.0/255.0, 255.0/255.0, 1.0),
            local_frame: local_frame,
            canvas: canvas::Canvas::new(),
        }
    }

    /// Set plot title
    pub fn set_title(mut self, title: &str) -> Self {
        self.title.set_content(title);
        self
    }

    /// Set plot background color. **Note**: This is different from the canvas background color.
    pub fn set_color(mut self, color: Rgba) -> Self {
        self.color = color;
        self
    }

    /// Set the plot background color. **Note**: This is different from the canvas background color.
    pub fn set_color_rgb(mut self, red: f32, green: f32, blue: f32) -> Self {
        let red = red.max(0.0);
        let red = red.min(1.0);
        let green = green.max(0.0);
        let green = green.min(1.0);
        let blue = blue.max(0.0);
        let blue = blue.min(1.0);
        self.color = Rgba::new(red, green, blue, 1.0);
        self
    }

    /// Set the plot background color. **Note**: This is different from the canvas background color.
    pub fn set_color_rgba(mut self, red: f32, green: f32, blue: f32, alpha: f32) -> Self {
        let red = red.max(0.0);
        let red = red.min(1.0);
        let green = green.max(0.0);
        let green = green.min(1.0);
        let blue = blue.max(0.0);
        let blue = blue.min(1.0);
        let alpha = alpha.max(0.0);
        let alpha = alpha.min(1.0);
        self.color = Rgba::new(red, green, blue, alpha);
        self
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

    /// Set the color of the border around the plot
    pub fn set_border_color(mut self, color: Rgba) -> Self {
        self.local_frame.set_color(color);
        self
    }

    /// Set the line width of the border around the plot
    pub fn set_border_thickness(mut self, val: f64) -> Self {
        self.local_frame.set_thickness(val);
        self
    }

    /// Add a canvas to the plot
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
    /// The function scales various elements within the plot, and calls a similar plot for its
    /// canvasses.
    pub fn fit(&mut self) -> Result<(), Error> {
        let scale_factor = self.local_frame.diag_len() / 2f64.sqrt();
        self.scale_size(scale_factor);
        self.canvas.fit(&self.local_frame)?;

        Ok(())
    }

    /// Do the actual drawing of the plot
    pub fn draw(&self, cr: &Context, fig_rel_height: f64, fig_rel_width: f64) {

        // Fill background
        cr.set_source_rgba(self.color.red as f64, self.color.green as f64, self.color.blue as f64,
                           self.color.alpha as f64);
        cr.rectangle(self.local_frame.left(), self.local_frame.bottom(),
                     self.local_frame.width(), self.local_frame.height());
        cr.fill();

        // Draw frame border
        self.local_frame.draw(cr, fig_rel_height, fig_rel_width);

        // Draw canvas
        self.canvas.draw(cr, fig_rel_height, fig_rel_width);
    }

}
