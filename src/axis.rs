//! Axis module
//!

use cairo::Context;
use cairo::enums::{FontSlant, FontWeight};

use utils::{Frame, Text};
use mark::{Mark, Tick, GridLine, trim_marks};

#[derive(Clone, Debug)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

#[derive(Clone, Debug)]
pub enum Side {
    Left,
    Right,
    Bottom,
    Top,
}

/// ## Axis
///
/// An axis is a reference source for the plot. It is often displayed as a line with evenly spaced
/// ticks. The ticks are often labeled, and so is also the whole axis.
#[derive(Clone, Debug)]
pub struct Axis {
    orientation: Orientation,
    color: [f64; 4],
    line_width: f64,
    side: Side,
    local_frame: Frame,
    global_frame: Frame,
    data_range: [f64; 2],
    label: Text,
    label_offset: f64,
    ca_num_marks: usize,
    tick_color: [f64; 4],
    grid: bool,
    grid_width: f64,
    grid_color: [f64; 4],
    marks: Vec<Mark>,
    ticks: Vec<Tick>,
    gridlines: Vec<GridLine>,
}

impl Axis {
    pub fn new(orientation: Orientation, side: Side) -> Axis {
        Axis {
            orientation: orientation,
            color: [0.0, 0.0, 0.0, 1.0],
            line_width: 0.005,
            side: side,
            local_frame: Frame::new(),
            global_frame: Frame::new(),
            data_range: [0.0, 1.0],
            label: Text::new(""),
            label_offset: 0.12,
            ca_num_marks: 5,
            tick_color: [0.0, 0.0, 0.0, 0.0],
            grid: true,
            grid_width: 0.005,
            grid_color: [1.0, 1.0, 1.0, 1.0],
            marks: Vec::<Mark>::new(),
            ticks: Vec::<Tick>::new(),
            gridlines: Vec::<GridLine>::new(),
        }
    }

    pub fn orientation(&self) -> Orientation {
        self.orientation.clone()
    }

    pub fn data_min(&self) -> f64 {
        self.data_range[0]
    }

    pub fn data_max(&self) -> f64 {
        self.data_range[1]
    }

    /// Update gridlines
    fn set_gridlines(&mut self) {
        let mut gridlines = Vec::<GridLine>::new();
        for mark in self.marks.iter() {
            let grid_frame = match self.orientation {
                Orientation::Horizontal => Frame::from_sides(mark.fig_mark(),
                                                             mark.fig_mark() + self.grid_width,
                                                             self.global_frame.bottom(),
                                                             self.global_frame.top()),
                Orientation::Vertical => Frame::from_sides(self.global_frame.left(),
                                                           self.global_frame.right(),
                                                           mark.fig_mark(),
                                                           mark.fig_mark() + self.grid_width),
            };
            gridlines.push(GridLine::from_params(self.grid_color, grid_frame));
        }
    }

    /// Update ticks
    fn set_ticks(&mut self) {
        let mut ticks = Vec::<Tick>::new();
        for mark in self.marks.iter() {
            let coord = match self.side {
                Side::Left => self.global_frame.left(),
                Side::Right => self.global_frame.right(),
                Side::Bottom => self.global_frame.bottom(),
                Side::Top => self.global_frame.top(),
            };
            let mut tick = Tick::new(self.orientation.clone(), mark.fig_mark(), coord);
            // TODO: Format ticks in compute_marks
            tick.set_label(&format!("{}", mark.data_mark()));
            tick.set_color(self.tick_color);
            ticks.push(tick);
        }
    }

    fn scale_size(&mut self, factor: f64) {
        self.label_offset *= factor;
        self.line_width *= factor;
        self.label.scale_size(factor);
    }

    /// Fit
    ///
    /// This function is called just before draw(), and updates the default w.r.t. user input.
    /// and changes above in the hierarchy (canvas -> plot -> figure).
    pub fn fit(&mut self, canvas_frame: Frame, canvas_marks: Vec<Mark>) {
        // Local frame is determined from initialization or user input.
        self.global_frame = self.local_frame.relative_to(&canvas_frame);
        let scale_factor = self.global_frame.diag_len();
        self.scale_size(scale_factor);

        // Compute marks and set ticks and gridlines
        self.marks = match self.orientation {
            Orientation::Horizontal => trim_marks(canvas_marks, self.global_frame.left(), self.global_frame.right()),
            Orientation::Vertical => trim_marks(canvas_marks, self.global_frame.bottom(), self.global_frame.top()),
        };
        self.set_ticks();
        self.set_gridlines();

        for tick in self.ticks.iter_mut() {
            tick.fit(self.global_frame.clone());
        }
    }

    pub fn draw(&self, cr: &Context) {

        // Axis line
        cr.set_source_rgba(self.color[0], self.color[1], self.color[2], self.color[3]);
        cr.set_line_width(self.line_width);
        match self.side {
            Side::Left => {
                cr.move_to(self.global_frame.left(), self.global_frame.bottom());
                cr.line_to(self.global_frame.left(), self.global_frame.top());
            },
            Side::Right => {
                cr.move_to(self.global_frame.right(), self.global_frame.bottom());
                cr.line_to(self.global_frame.right(), self.global_frame.top());
            },
            Side::Bottom => {
                cr.move_to(self.global_frame.left(), self.global_frame.bottom());
                cr.line_to(self.global_frame.right(), self.global_frame.bottom());
            },
            Side::Top => {
                cr.move_to(self.global_frame.left(), self.global_frame.top());
                cr.line_to(self.global_frame.right(), self.global_frame.top());
            },
        }
        cr.stroke();

        // Label
        cr.select_font_face("Serif", FontSlant::Italic, FontWeight::Normal);
        cr.set_font_size(self.label.font_size());
        match self.side {
            Side::Left => {
                cr.move_to(self.global_frame.left() + self.label_offset,
                           self.global_frame.bottom() + self.global_frame.height() / 2.0);
            },
            Side::Right => {
                cr.move_to(self.global_frame.right() + self.label_offset,
                           self.global_frame.bottom() + self.global_frame.height() / 2.0);
            },
            Side::Bottom => {
                cr.move_to(self.global_frame.left() + self.global_frame.width() / 2.0,
                           self.global_frame.bottom() + self.label_offset);
            },
            Side::Top => {
                cr.move_to(self.global_frame.left() + self.global_frame.width() / 2.0,
                           self.global_frame.top() + self.label_offset);
            },
        }
        cr.show_text(&self.label.content());

        // Gridlines
        if self.grid {
            for gridline in self.gridlines.iter() {
                gridline.draw(cr);
            }
        }

        // Ticks and tick labels
        for tick in self.ticks.iter() {
            tick.draw(cr);
        }
    }
}
