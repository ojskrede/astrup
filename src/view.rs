//! Definition of the View struct
//!

use failure::Error;

use gio;
use gio::prelude::*;
use gtk;
use gtk::prelude::*;
use gtk::{ApplicationWindow, Window, WindowPosition, DrawingArea};

use figure::Figure;

// Make moving clones into closures more convenient. From cairo-rs tutorial.
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

/// A struct used to display the figure(s) on screen.
///
/// Currently, this is just a window with a title bar and an exit cross. One window is made for
/// each figure. If you resize the window after it has been launched, the figure stays intact, e.i.
/// no resizing of the figure. This can change in the future, but has low priority.
pub struct View {
    figures: Vec<Figure>,
    application: gtk::Application,
}

impl View {
    /// Create and return a new View
    pub fn new() -> Result<View, Error> {
        let app = gtk::Application::new("com.astrup.application", gio::ApplicationFlags::empty())
                                        .expect("Failed to initialize application");
        Ok(View {
            figures: vec![],
            application: app,
        })
    }

    /// Create and return a new View from an existing Figure
    pub fn with_figure(mut figure: Figure) -> Result<View, Error> {
        figure.fit()?;
        let app = gtk::Application::new("com.astrup.application", gio::ApplicationFlags::empty())
                                        .expect("Failed to initialize application");
        Ok(View {
            figures: vec![figure],
            application: app,
        })
    }

    /// Add figures to be displayed
    pub fn add_figure(mut self, mut figure: Figure) -> Result<Self, Error> {
        figure.fit()?;
        self.figures.push(figure);

        Ok(self)
    }

    /// Display the figures on the screen. This must be called in order to display anything.
    pub fn show(self) {
        let figures = self.figures;
        self.application.connect_startup(move |app| {
            for fig in &figures {
                let window = ApplicationWindow::new(app);
                let drawing_area = Box::new(DrawingArea::new)();
                drawing_area.connect_draw(clone!(fig => move |_, cr| {
                    fig.draw(cr);

                    Inhibit(false)
                }));

                window.set_title(&fig.window_title());
                window.set_border_width(10);
                //window.fullscreen();
                window.set_position(WindowPosition::Center);
                window.set_default_size(fig.width() as i32, fig.height() as i32);

                //let header = Header::new(&fig.get_title());
                //window.set_titlebar(&header.container);
                window.set_wmclass("app-name", "App name");
                Window::set_default_icon_name("iconname");

                window.connect_delete_event(clone!(window => move |_, _| {
                    window.destroy();
                    Inhibit(false)
                }));

                window.add(&drawing_area);
                window.show_all();
            }
        });

        self.application.connect_activate(|_| {});
        self.application.run(&Vec::<_>::new());
    }
}
