//! ## Figure
//!
//! Definition of the Figure struct
//!


use std::f64::consts::PI;

use std::fs::File;
use failure::{Error, err_msg};

use gio;
use gio::prelude::*;
use gtk;
use gtk::prelude::*;
use gtk::DrawingArea;
use cairo::{Context, Format, ImageSurface, Matrix, MatrixTrait};

use plot::Plot;

// make moving clones into closures more convenient
macro_rules! clone {
    (@param _) => ( _ );
    (@param $x:ident) => ( $x );
    ($($n:ident),+ => move || $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move || $body
        }
    );
    ($($n:ident),+ => move |$($p:tt),+| $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move |$(clone!(@param $p),)+| $body
        }
    );
}

#[derive(Clone)]
pub struct Figure {
    plots: Vec<Plot>,
    title: String,
    height: usize,
    width: usize,
    color: [f64; 4],
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
            color: [1.0, 1.0, 1.0, 1.0],
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

    pub fn add(&mut self, plot: Plot) {
        self.plots.push(plot);
    }

    fn fit(&mut self) {
        for plot in self.plots.iter_mut() {
            plot.fit();
        }
    }

    pub fn save(&self, filename: &str) -> Result<(), Error> {
        // Since both save() and show() can be called, and since all drawing is happening in both,
        // multiple calls to fit() will be made, and this can mess up things if we call it on self.
        // The simplest solution is to clone self. But one should perhaps make fit() idempotent?.
        let mut fig = self.clone();
        fig.fit();
        let surface = match ImageSurface::create(Format::ARgb32, fig.width as i32, fig.height as i32) {
            Ok(val) => val,
            Err(msg) => return Err(err_msg(format!("{:?}", msg))),
        };
        let cr = Context::new(&surface);

        cr.scale(fig.width as f64, fig.height as f64);

        cr.set_source_rgba(fig.color[0], fig.color[1], fig.color[2], fig.color[3]);
        cr.paint();

        // By default, the origin is in the top left corner, x is increasing to the right, and y is
        // increasing downwards. This transforms the origin to the bottom left, and increasing y
        // upwards.
        let flip_matrix = Matrix::new(1.0, 0.0, 0.0, -1.0, 0.0, 1.0);
        cr.transform(flip_matrix);

        for plot in fig.plots.iter() {
            plot.draw(&cr);
        }

        let mut file = File::create(filename)?;
        surface.write_to_png(&mut file)?;

        Ok(())
    }

    pub fn show(self) {
        // In order to move self into the innermost nested closure (the argument of
        // drawing_area.connect_draw() ), we clone self here, and move use it.
        let mut fig = self.clone();
        fig.fit();
        self.application.connect_startup(move |app| {
            build_ui(&fig, app);
        });

        self.application.connect_activate(|_| {});

        self.application.run(&Vec::<_>::new());
    }

    //fn save(path: PathBuf) {}
}

fn build_ui(fig: &Figure, app: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(app);
    let drawing_area = Box::new(DrawingArea::new)();
    drawing_area.connect_draw(clone!(fig => move |_, cr| {
        cr.scale(fig.width as f64, fig.height as f64);

        cr.set_source_rgba(fig.color[0], fig.color[1], fig.color[2], fig.color[3]);
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

        for plot in fig.plots.iter() {
            plot.draw(cr);
        }

        Inhibit(false)
    }));

    window.set_default_size(fig.width as i32, fig.height as i32);

    window.connect_delete_event(clone!(window => move |_, _| {
        window.destroy();
        Inhibit(false)
    }));

    window.add(&drawing_area);
    window.show_all();

}
