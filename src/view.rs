
use gio;
use gio::prelude::*;
use gtk;
use gtk::prelude::*;
use gtk::{ApplicationWindow, Window, WindowPosition, DrawingArea};

use figure::Figure;

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

/*
struct Header {
    container: gtk::HeaderBar
}

impl Header {
    fn new(name: &str) -> Header {
        let container = gtk::HeaderBar::new();

        container.set_title(name);

        container.set_show_close_button(true);

        Header { container }
    }
}
*/

pub struct View {
    figures: Vec<Figure>,
    application: gtk::Application,
}

impl View {
    pub fn new(mut figure: Figure) -> View {
        figure.fit();
        let app = gtk::Application::new("com.astrup.application", gio::ApplicationFlags::empty())
                                           .expect("Failed to initialize application");
        View {
            figures: vec![figure],
            application: app,
        }
    }

    pub fn add(&mut self, mut figure: Figure) {
        figure.fit();
        self.figures.push(figure);
    }

    pub fn show(self) {
        let figures = self.figures;
        self.application.connect_startup(move |app| {
            for fig in figures.iter() {
                let window = ApplicationWindow::new(app);
                let drawing_area = Box::new(DrawingArea::new)();
                //drawing_area.set_size_request(fig.width() as i32, fig.height() as i32);
                drawing_area.connect_draw(clone!(fig => move |_, cr| {
                    fig.draw(cr);

                    Inhibit(false)
                }));

                window.set_title("Figure name");
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
