//! Mark module
//!
//!

use cairo::{Context, Matrix, MatrixTrait};
use cairo::enums::{FontSlant, FontWeight};

use utils::{Coord, Frame, Text};

#[derive(Clone, Debug)]
pub struct Mark {
    local: Coord,
    global: Coord,
    label: Text,
    label_offset: f64,
    //tick: Tick,
    //gridline: GridLine,
}

impl Mark {
    pub fn new(coord: Coord) -> Mark {
        Mark {
            local: coord,
            global: Coord::new(0.0, 0.0),
            label: Text::new(""),
            label_offset: 0.05,
            //tick: Tick::new(),
            //gridline: GridLine::new(),
        }
    }

    pub fn set_label_content(&mut self, content: &str) {
        self.label.set_content(content);
    }

    pub fn set_local(&mut self, coord: Coord) {
        self.local = coord;
    }

    pub fn set_global(&mut self, coord: Coord) {
        self.global= coord;
    }

    pub fn label(&self) -> Text { self.label.clone() }

    fn scale_size(&mut self, factor: f64) {
        self.label_offset *= factor;
    }

    pub fn fit(&mut self, parent_frame: &Frame) {
        self.global = self.local.relative_to(parent_frame);
        self.scale_size(parent_frame.diag_len() / 2f64.sqrt());
        //self.label.fit()
    }

    pub fn draw(&self, cr: &Context) {
        cr.arc(self.global.x(), self.global.y(), 0.005, 0., 2.0*3.1415);
        cr.fill();

        cr.select_font_face("Serif", FontSlant::Italic, FontWeight::Normal);
        cr.set_font_size(self.label.font_size());
        cr.move_to(self.global.x(), self.global.y());

        cr.transform(Matrix::new(1.0, 0.0, 0.0, -1.0, 0.0, 0.0));
        cr.rotate(self.label.angle());
        cr.show_text(&self.label.content());
        cr.rotate(-self.label.angle());
        cr.transform(Matrix::new(1.0, 0.0, 0.0, -1.0, 0.0, 0.0));
        //self.tick.draw(cr);
        //self.gridline.draw(cr);
    }
}


/// ## Tick
///
/// Indicator used by axis to serve as a reference for the displayed data
#[derive(Clone, Debug)]
pub struct Tick {
    color: [f64; 4],
    line_width: f64,
    length: f64,
}

impl Tick {
    pub fn new() -> Tick {
        Tick {
            color: [0.0, 0.0, 0.0, 1.0],
            line_width: 0.005,
            length: 0.01,
        }
    }

    pub fn set_color(&mut self, color: [f64; 4]) {
        self.color = color;
    }

    fn scale_size(&mut self, factor: f64) {
        self.line_width *= factor;
        self.length *= factor;
    }

    pub fn fit(&mut self, mark_frame: Frame) {
        self.scale_size(mark_frame.diag_len() / 2f64.sqrt());
    }
}

/*
/// ## GridLine
///
/// Indicator used by axis to serve as a reference for the displayed data
#[derive(Clone, Debug)]
pub struct GridLine {
    color: [f64; 4],
    global_frame: Frame,
}

impl GridLine {
    pub fn new() -> GridLine {
        GridLine {
            color: [1.0, 1.0, 1.0, 1.0],
            global_frame: Frame::new(),
        }
    }

    pub fn from_params(color: [f64; 4], frame: Frame) -> GridLine {
        GridLine {
            color: color,
            global_frame: frame,
        }
    }

    pub fn draw(&self, cr: &Context) {
        cr.set_source_rgba(self.color[0], self.color[1], self.color[2], self.color[3]);
        cr.rectangle(self.global_frame.left(), self.global_frame.bottom(),
                     self.global_frame.width(), self.global_frame.height());
        cr.fill();
    }
}

/// Trim mark locations
///
/// Given a list of marks and boundaries assumed to be in the same reference system as the fig
/// locations of the marks in the list, return a list where only marks inside the boundaries are
/// kept.
pub fn trim_marks(marks: Vec<Mark>, min_val: f64, max_val: f64) -> Vec<Mark> {
    let mut trimmed_list = Vec::<Mark>::new();
    for mark in marks {
        if mark.fig_mark() < max_val && mark.fig_mark() > min_val {
            trimmed_list.push(mark);
        }
    }
    trimmed_list
}
*/

pub fn prettify(number: f64, omagn: f64) -> String {
    if omagn > 5.0 || omagn < -5.0 {
        format!("{:e}", number)
    } else if omagn >= 1.5 || omagn <= -1.5 {
        format!("{0:.0}", number)
    } else {
        format!("{num:.prec$}", num=number, prec= 2 - omagn as usize)
    }
}
