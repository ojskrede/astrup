//! ## Plot
//!
//! Module that defines the Plot structure.
//!

use std::f64::{MAX, MIN};

use cairo::Context;
use cairo::enums::{FontSlant, FontWeight};

use utils::{Drawable, Frame};
use scatter::Scatter;
//use style::Style;


#[derive(Clone, Debug)]
struct Axis {
    x_start: f64,
    x_end: f64,
    y_start: f64,
    y_end: f64,
    color: [f64; 4],
    lw: f64,
    label: String,
    range: [f64; 2],
}

impl Axis {
    fn new(x_start: f64, x_end: f64, y_start: f64, y_end: f64) -> Axis {
        Axis {
            x_start: x_start,
            x_end: x_end,
            y_start: y_start,
            y_end: y_end,
            color: [0.0, 0.0, 0.0, 1.0],
            lw: 0.01,
            label: String::from("Label"),
            range: [0.0, 1.0],
        }
    }

    fn set_label(&mut self, label: &str) {
        self.label = String::from(label);
    }

    fn set_range(&mut self, min: f64, max: f64) {
        self.range = [min, max];
    }

    pub fn min(&self) -> f64 {
        self.range[0]
    }

    pub fn max(&self) -> f64 {
        self.range[1]
    }
}

#[derive(Clone, Debug)]
pub struct Plot {
    size: [usize; 2],
    //style: Style,
    title: String,
    bg_color: [f64; 4],
    grid: bool,
    border: bool,
    origin: [f64; 2],
    x_axis: Axis,
    y_axis: Axis,
    drawables: Vec<Scatter>,
}

impl Plot {
    pub fn new() -> Plot {
        let x_origin = 0.2;
        let y_origin = 0.8;

        Plot {
            size: [200, 300],
            origin: [x_origin, y_origin],
            title: String::from("Plot"),
            bg_color: [0.9, 0.9, 0.9, 0.9],
            grid: false,
            border: true,
            x_axis: Axis::new(x_origin - 0.05, 0.9, y_origin, y_origin),
            y_axis: Axis::new(x_origin, x_origin, y_origin + 0.05, 0.1),
            drawables: Vec::<Scatter>::new(),
        }
    }

    pub fn grid(&mut self) {
        self.grid = true;
    }

    pub fn bg_color(&mut self, bg_color: &[f64; 4]) {
        self.bg_color = bg_color.to_owned();
    }

    pub fn x_label(&mut self, label: &str) {
        self.x_axis.set_label(label);
    }

    pub fn y_label(&mut self, label: &str) {
        self.y_axis.set_label(label);
    }

    pub fn draw(&mut self, drawable: Scatter) {
        self.drawables.push(drawable);
    }

    pub fn fit(&mut self) {

        let mut min_x = MAX;
        let mut max_x = MIN;
        let mut min_y = MAX;
        let mut max_y = MIN;
        for drawable in self.drawables.iter() {
            let frame = drawable.frame();
            if drawable.min_x() < min_x { min_x = frame.min_x(); }
            if drawable.max_x() > max_x { max_x = frame.max_x(); }
            if drawable.min_y() < min_y { min_y = frame.min_y(); }
            if drawable.max_y() > max_y { max_y = frame.max_y(); }
        }

        self.x_axis.set_range(min_x, max_x);
        self.y_axis.set_range(min_y, max_y);

        let outer_frame = Frame::new(self.x_axis.min(), self.x_axis.max(), self.y_axis.min(), self.y_axis.max());
        let frame = Frame::new(0.2, 0.9, 0.1, 0.8);
        for drawable in self.drawables.iter_mut() {
            drawable.fit(&frame);
        }
    }

    pub fn draw_fn(&self, cr: &Context) {

        //cr.scale(self.size[1] as f64, self.size[0] as f64); // TODO: Why does this ruin things

        // Background
        cr.set_source_rgba(self.bg_color[0], self.bg_color[1], self.bg_color[2], self.bg_color[3]);
        cr.paint();

        // if self.grid {}

        if self.border {
            cr.set_source_rgb(0.0, 0.0, 0.0);
            cr.set_line_width(0.01);
            cr.move_to(0.0, 0.0);
            cr.line_to(0.0, 1.0);
            cr.line_to(1.0, 1.0);
            cr.line_to(1.0, 0.0);
            cr.line_to(0.0, 0.0);
            cr.stroke();
        }

        // TODO: Create draw_fn for axis
        // Horizontal axis
        cr.set_source_rgba(self.x_axis.color[0], self.x_axis.color[1], self.x_axis.color[2],
                           self.x_axis.color[3]);
        cr.set_line_width(self.x_axis.lw);
        cr.move_to(self.x_axis.x_start, self.x_axis.y_start);
        cr.line_to(self.x_axis.x_end, self.x_axis.y_end);
        cr.stroke();

        cr.select_font_face("Sans", FontSlant::Normal, FontWeight::Normal);
        cr.set_font_size(0.04);

        cr.move_to(self.origin[0], self.origin[1] + 0.1);
        cr.show_text(&format!("{}", self.x_axis.min()));
        cr.move_to(self.x_axis.x_end, self.origin[1] + 0.1);
        cr.show_text(&format!("{}", self.x_axis.max()));

        // Vertical axis
        cr.set_source_rgba(self.y_axis.color[0], self.y_axis.color[1], self.y_axis.color[2],
                           self.y_axis.color[3]);
        cr.set_line_width(self.y_axis.lw);
        cr.move_to(self.y_axis.x_start, self.y_axis.y_start);
        cr.line_to(self.y_axis.x_end, self.y_axis.y_end);
        cr.stroke();

        cr.move_to(self.origin[0] - 0.1, self.origin[1]);
        cr.show_text(&format!("{}", self.y_axis.min()));
        cr.move_to(self.origin[0] - 0.1, self.y_axis.y_end);
        cr.show_text(&format!("{}", self.y_axis.max()));

        for drawable in self.drawables.iter() {
            drawable.draw_fn(cr);
        }
    }

}

/*
impl Drawable for Plot {
    fn draw_fn(&self, cr: &Context) {

        //cr.scale(self.size[1] as f64, self.size[0] as f64); // TODO: Why does this ruin things

        // Background
        cr.set_source_rgba(self.bg_color[0], self.bg_color[1], self.bg_color[2], self.bg_color[3]);
        cr.paint();

        // if self.grid {}

        if self.border {
            cr.set_source_rgb(0.0, 0.0, 0.0);
            cr.set_line_width(0.01);
            cr.move_to(0.0, 0.0);
            cr.line_to(0.0, 1.0);
            cr.line_to(1.0, 1.0);
            cr.line_to(1.0, 0.0);
            cr.line_to(0.0, 0.0);
            cr.stroke();
        }

        // Horizontal axis
        cr.set_source_rgba(self.x_axis.color[0], self.x_axis.color[1], self.x_axis.color[2],
                           self.x_axis.color[3]);
        cr.set_line_width(self.x_axis.lw);
        cr.move_to(self.x_axis.x_start, self.x_axis.y_start);
        cr.line_to(self.x_axis.x_end, self.x_axis.y_end);
        cr.stroke();

        // Vertical axis
        cr.set_source_rgba(self.y_axis.color[0], self.y_axis.color[1], self.y_axis.color[2],
                           self.y_axis.color[3]);
        cr.set_line_width(self.y_axis.lw);
        cr.move_to(self.y_axis.x_start, self.y_axis.y_start);
        cr.line_to(self.y_axis.x_end, self.y_axis.y_end);
        cr.stroke();
    }
}
*/
