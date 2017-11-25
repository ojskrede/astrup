//! ## Plot
//!
//! Module that defines the Plot structure.
//!


use cairo::Context;

use draw::Drawable;
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
        }
    }

    fn set_label(&mut self, label: &str) {
        self.label = String::from(label);
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
        let x_origin = 0.1;
        let y_origin = 0.9;

        Plot {
            size: [200, 300],
            origin: [0.1, 0.9],
            title: String::from("Plot"),
            bg_color: [0.9, 0.9, 0.9, 0.9],
            grid: false,
            border: true,
            x_axis: Axis::new(x_origin - 0.05, 0.9, y_origin, y_origin),
            y_axis: Axis::new(x_origin, x_origin, 0.1, y_origin + 0.05),
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
