//! Mark module
//!
//!

use cairo::Context;

use utils::{Coord, Frame, Text};

#[derive(Clone, Debug)]
pub struct Mark {
    local: Coord,
    global: Coord,
    label: Text,
}

impl Mark {
    pub fn new(coord: Coord) -> Mark {
        Mark {
            local: coord,
            global: Coord::new(0.0, 0.0),
            label: Text::new(""),
        }
    }

    pub fn global_x(&self) -> f64 {
        self.global.x()
    }

    pub fn global_y(&self) -> f64 {
        self.global.y()
    }

    pub fn global_coord(&self) -> Coord {
        self.global.clone()
    }

    pub fn label(&self) -> Text {
        self.label.clone()
    }

    pub fn label_hor_offset(&self) -> f64 {
        self.label.hor_offset()
    }

    pub fn label_ver_offset(&self) -> f64 {
        self.label.ver_offset()
    }

    pub fn set_label_content(&mut self, content: &str) {
        self.label.set_content(content);
    }

    pub fn set_label_offset(&mut self, hor: f64, ver: f64) {
        self.label.set_offset(hor, ver);
    }

    pub fn scale_label_offset(&mut self, factor: f64) {
        self.label.scale_offset(factor);
    }

    pub fn set_font_size(&mut self, val: f64) {
        self.label.set_font_size(val);
    }

    pub fn set_local(&mut self, coord: Coord) {
        self.local = coord;
    }

    pub fn set_global(&mut self, coord: Coord) {
        self.global = coord;
    }

    fn scale_size(&mut self, factor: f64) {
        self.label.scale_size(factor);
    }

    pub fn fit(&mut self, parent_frame: &Frame) {
        self.global = self.local.relative_to(parent_frame);
        self.scale_size(parent_frame.diag_len() / 2f64.sqrt());
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

/// ## GridLine
///
/// Indicator used by axis to serve as a reference for the displayed data
#[derive(Clone, Debug)]
pub struct GridLine {
    global_start: Coord,
    global_end: Coord,
    width: f64,
    color: [f64; 4],
}

impl GridLine {
    pub fn new(start: Coord, end: Coord) -> GridLine {
        GridLine {
            global_start: start,
            global_end: end,
            width: 0.005,
            color: [1.0, 1.0, 1.0, 1.0],
        }
    }

    pub fn set_color(&mut self, color: [f64; 4]) {
        self.color = color;
    }

    pub fn set_width(&mut self, width: f64) {
        self.width = width;
    }

    pub fn scale_size(&mut self, factor: f64) {
        self.width *= factor;
    }

    pub fn draw(&self, cr: &Context) {
        cr.set_source_rgba(self.color[0], self.color[1], self.color[2], self.color[3]);
        cr.set_line_width(self.width);
        cr.move_to(self.global_start.x(), self.global_start.y());
        cr.line_to(self.global_end.x(), self.global_end.y());
        cr.stroke();
    }
}

pub fn prettify(number: f64) -> String {
    let omagn = if number == 0.0 { 0.0 } else { number.abs().log10().floor() };
    if omagn > 2.0 || omagn < -2.0 {
        format!("{:>e}", number)
    } else if omagn >= 1.0 || omagn <= -1.0 {
        format!("{0:>.0}", number)
    } else {
        format!("{num:>.prec$}", num=number, prec= 2 - omagn as usize)
    }
}
