//! ## Figure
//!
//! Definition of the Figure struct
//!


use std::fs::File;
use failure::{Error, err_msg};
use palette::Rgba;

use cairo::{Context, Format, ImageSurface, Matrix, MatrixTrait};

use plot::Plot;
use utils::Frame;

#[derive(Clone)]
pub struct Figure {
    plots: Vec<Plot>,
    title: String,
    height: usize,
    width: usize,
    color: Rgba,
    local_frame: Frame, // Currently only used for displaying border or not
}

impl Figure {
    pub fn new() -> Figure {
        let mut local_frame = Frame::new();
        local_frame.display_border(true);
        local_frame.set_thickness(0.001);
        Figure {
            plots: Vec::<Plot>::new(),
            title: String::from("Figure"),
            height: 800,
            width: 800,
            color: Rgba::new(1.0, 1.0, 1.0, 1.0),
            local_frame: local_frame,
        }
    }

    pub fn set_title(mut self, title: &str) -> Self {
        self.title = String::from(title);
        self
    }

    pub fn set_height(mut self, val: usize) -> Self {
        self.height = val;
        self
    }

    pub fn set_width(mut self, val: usize) -> Self {
        self.width = val;
        self
    }

    pub fn set_color(mut self, color: Rgba) -> Self {
        self.color = color;
        self
    }

    /// Set the figure background color
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

    /// Set the figure background color
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

    pub fn get_title(&self) -> String {
        self.title.clone()
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn width(&self) -> usize {
        self.width
    }

    /// Whether or not to display a border around the figure
    pub fn display_border(mut self, val: bool) -> Self {
        self.local_frame.display_border(val);
        self
    }

    /// Set the color of the border around the figure
    pub fn set_border_color(mut self, color: Rgba) -> Self {
        self.local_frame.set_color(color);
        self
    }

    /// Set the line width of the border around the figure
    pub fn set_border_thickness(mut self, val: f64) -> Self {
        self.local_frame.set_thickness(val);
        self
    }

    pub fn add(mut self, plot: Plot) -> Self {
        self.plots.push(plot);
        self
    }

    pub fn fit(&mut self) -> Result<(), Error> {
        for plot in self.plots.iter_mut() {
            plot.fit()?;
        }

        Ok(())
    }

    pub fn save(self, filename: &str) -> Result<(Self), Error> {
        // Since both save() and show() can be called, and since all drawing is happening in both,
        // multiple calls to fit() will be made, and this can mess up things if we call it on self.
        // The simplest solution is to clone self. But one should perhaps make fit() idempotent?.
        let mut fig = self.clone();
        fig.fit()?;
        let surface = match ImageSurface::create(Format::ARgb32, fig.width as i32, fig.height as i32) {
            Ok(val) => val,
            Err(msg) => return Err(err_msg(format!("{:?}", msg))),
        };
        let cr = Context::new(&surface);

        fig.draw(&cr);

        let mut file = File::create(filename)?;
        surface.write_to_png(&mut file)?;

        Ok((self))
    }

    pub fn draw(&self, cr: &Context) {
        cr.scale(self.width as f64, self.height as f64);

        cr.set_source_rgba(self.color.red as f64, self.color.green as f64, self.color.blue as f64,
                           self.color.alpha as f64);
        cr.paint();

        // Frame border
        self.local_frame.draw(cr);

        // By default, the origin is in the top left corner, x is increasing to the right, and y is
        // increasing downwards. This transforms the origin to the bottom left, and increasing y
        // upwards.
        let flip_matrix = Matrix::new(1.0, 0.0, 0.0, -1.0, 0.0, 1.0);
        cr.transform(flip_matrix);

        for plot in self.plots.iter() {
            plot.draw(&cr);
        }
    }
}
