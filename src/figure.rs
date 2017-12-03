//! ## Figure
//!
//! Definition of the Figure struct
//!


use std::fs::File;
use failure::{Error, err_msg};
use palette::Rgba;

use gio;
use gtk;
use cairo::{Context, Format, ImageSurface, Matrix, MatrixTrait};

use plot::Plot;

#[derive(Clone)]
pub struct Figure {
    plots: Vec<Plot>,
    title: String,
    height: usize,
    width: usize,
    color: Rgba,
    application: gtk::Application,
}

impl Figure {
    pub fn new() -> Figure {
        let app = gtk::Application::new("com.astrup.application", gio::ApplicationFlags::empty())
                                   .expect("Failed to initialize application");
        Figure {
            plots: Vec::<Plot>::new(),
            title: String::from("Figure"),
            height: 800,
            width: 800,
            color: Rgba::new(1.0, 1.0, 1.0, 1.0),
            application: app,
        }
    }

    pub fn set_title(&mut self, title: &str) {
        self.title = String::from(title);
    }

    pub fn set_height(&mut self, val: usize) {
        self.height = val;
    }

    pub fn set_width(&mut self, val: usize) {
        self.width = val;
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

    pub fn add(&mut self, plot: Plot) {
        self.plots.push(plot);
    }

    pub fn fit(&mut self) -> Result<(), Error> {
        for plot in self.plots.iter_mut() {
            plot.fit()?;
        }

        Ok(())
    }

    pub fn save(&self, filename: &str) -> Result<(), Error> {
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

        Ok(())
    }

    pub fn draw(&self, cr: &Context) {
        cr.scale(self.width as f64, self.height as f64);

        cr.set_source_rgba(self.color.red as f64, self.color.green as f64, self.color.blue as f64,
                           self.color.alpha as f64);
        cr.paint();

        cr.set_source_rgba(0.0, 0.0, 0.0, 1.0);
        cr.set_line_width(0.01);
        cr.rectangle(0.0, 0.0, 1.0, 1.0);
        cr.stroke();

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
