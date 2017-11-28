//! ## Figure
//!
//! Definition of the Figure struct
//!

use std::fs::File;

use gio;
use gio::prelude::*;
use gtk;
use gtk::prelude::*;
use gtk::DrawingArea;
use cairo::{Context, Format, ImageSurface};

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
    size: [usize; 2],
    bg_color: [f64; 4],
    application: gtk::Application,
}

impl Figure {
    pub fn new() -> Figure {
        let app = gtk::Application::new("com.astrup.application", gio::ApplicationFlags::empty())
                                   .expect("Failed to initialize application");
        Figure {
            plots: Vec::<Plot>::new(),
            title: String::from("Figure"),
            size: [800, 800],
            bg_color: [1.0, 1.0, 1.0, 1.0],
            application: app,
        }
    }

    pub fn set_title(&mut self, title: &str) {
        self.title = String::from(title);
    }

    pub fn set_size(&mut self, size: [usize; 2]) {
        self.size = size;
    }

    pub fn add(&mut self, plot: Plot) {
        self.plots.push(plot);
    }

    fn fit(&mut self) {
        // TODO: Adjust plot fig_frame automatically here, into a grid or something.
        for plot in self.plots.iter_mut() {
            plot.fit();
        }
    }

    // TODO: Return Result<(), Error>
    pub fn save(&mut self, filename: &str) {
        let surface = ImageSurface::create(Format::ARgb32, self.size[1] as i32, self.size[0] as i32)
                                   .expect("Can't create surface");
        let cr = Context::new(&surface);

        self.fit();
        cr.scale(self.size[1] as f64, self.size[0] as f64);

        cr.set_source_rgba(self.bg_color[0], self.bg_color[1], self.bg_color[2], self.bg_color[3]);
        cr.paint();

        // TODO: Place them in grid
        for plot in self.plots.iter() {
            plot.draw(&cr);
        }

        let mut file = File::create(filename).expect("Couldn't create 'file.png'");
        match surface.write_to_png(&mut file) {
            Ok(_) => println!("file.png created"),
            Err(_) => println!("Error create file.png"),
        }

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
        cr.scale(fig.size[1] as f64, fig.size[0] as f64);

        cr.set_source_rgba(fig.bg_color[0], fig.bg_color[1], fig.bg_color[2], fig.bg_color[3]);
        cr.paint();

        // TODO: Place them in grid
        for plot in fig.plots.iter() {
            plot.draw(cr);
        }

        Inhibit(false)
    }));

    window.set_default_size(fig.size[1] as i32, fig.size[0] as i32);

    window.connect_delete_event(clone!(window => move |_, _| {
        window.destroy();
        Inhibit(false)
    }));

    window.add(&drawing_area);
    window.show_all();

}
