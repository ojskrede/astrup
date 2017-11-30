//! Axis module
//!

use cairo::Context;
use cairo::enums::{FontSlant, FontWeight};

use utils;
use utils::{Frame, Text};

#[derive(Clone, Debug)]
pub enum Orientation {
    Horizontal,
    Vertical,
}
#[derive(Clone, Debug)]
pub struct Mark {
    // Mark in figure coordinate system
    fig: f64,
    // Mark in data coordinate system
    data: f64,
}

impl Mark {
    fn new() -> Mark {
        Mark {
            fig: 0.0,
            data: 0.0,
        }
    }

    pub fn set(&mut self, fig: f64, data: f64) {
        self.fig = fig;
        self.data = data;
    }

    fn fig_mark(&self) -> f64 { self.fig }

    fn data_mark(&self) -> f64 { self.data }
}

/// ## Tick
///
/// Indicator used by axis to serve as a reference for the displayed data
#[derive(Clone, Debug)]
struct Tick {
    orientation: Orientation,
    x_center: f64,
    y_center: f64,
    color: [f64; 4],
    line_width: f64,
    label: Text,
    label_offset: f64,
}

impl Tick {
    fn new(orientation: Orientation, x_center: f64, y_center: f64) -> Tick {
        Tick {
            orientation: orientation,
            x_center: x_center,
            y_center: y_center,
            color: [0.0, 0.0, 0.0, 1.0],
            line_width: 0.005,
            label: Text::new(""),
            label_offset: 0.05,
        }
    }

    fn set_label(&mut self, content: &str) {
        self.label.set_content(content);
    }

    fn set_label_size(&mut self, size: f64) {
        self.label.set_font_size(size);
    }

    fn label_size(&self) -> f64 {
        self.label.font_size()
    }

    fn x_center(&self) -> f64 {
        self.x_center
    }

    fn y_center(&self) -> f64 {
        self.y_center
    }

    fn scale_size(&mut self, factor: f64) {
        self.label_offset *= factor;
        self.line_width *= factor;
        self.label.scale_size(factor);
    }

    fn draw(&self, cr: &Context) {
        cr.set_source_rgba(self.color[0], self.color[1], self.color[2], self.color[3]);
        cr.set_line_width(self.line_width);

        match self.orientation {
            Orientation::Horizontal => {
                cr.move_to(self.x_center, self.y_center - 0.01);
                cr.line_to(self.x_center, self.y_center);
                cr.stroke();
            },
            Orientation::Vertical => {
                cr.move_to(self.x_center, self.y_center);
                cr.line_to(self.x_center + 0.01, self.y_center);
                cr.stroke();
            },
        }

        // Label
        cr.select_font_face("Serif", FontSlant::Normal, FontWeight::Normal);
        cr.set_font_size(self.label.font_size());
        match self.orientation {
            Orientation::Horizontal => {
                cr.move_to(self.x_center - 0.5 * self.label_offset, self.y_center + self.label_offset);
                cr.show_text(&self.label.content());
            },
            Orientation::Vertical => {
                cr.move_to(self.x_center - 2.0 * self.label_offset, self.y_center + 0.6 * self.label_offset);
                cr.show_text(&self.label.content());
            },
        }
    }
}

/// ## GridLine
///
/// Indicator used by axis to serve as a reference for the displayed data
#[derive(Clone, Debug)]
struct GridLine {
    color: [f64; 4],
    fig_frame: Frame,
}

impl GridLine {
    fn new() -> GridLine {
        GridLine {
            color: [1.0, 1.0, 1.0, 1.0],
            fig_frame: Frame::new(),
        }
    }

    /// Fit
    ///
    /// This function is the last call before draw(). This means that all information about
    /// possible changes from default, made by users, is taken into account here. This is supposed
    /// to fit the current gridlines w.r.t. possible changes in the axis where this belong.
    fn fit(&mut self, x_factor: f64, y_factor: f64) {
        self.fig_frame.set(x_factor * self.fig_frame.x_min(), x_factor * self.fig_frame.x_max(),
                           y_factor * self.fig_frame.y_min(), y_factor * self.fig_frame.y_max());
    }

    fn draw(&self, cr: &Context) {
        cr.set_source_rgba(self.color[0], self.color[1], self.color[2], self.color[3]);
        cr.rectangle(self.fig_frame.x_min(), self.fig_frame.y_min(),
                     self.fig_frame.x_max() - self.fig_frame.x_min(),
                     self.fig_frame.y_max() - self.fig_frame.y_min());
        cr.fill();
    }
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
    coords: Frame,
    fig_frame: Frame,
    data_range: [f64; 2],
    label: Text,
    label_offset: f64,
    ref_num_ticks: usize,
    grid: bool,
    gridlines: Vec<GridLine>,
    ticks: Vec<Tick>,
}

impl Axis {
    pub fn new(orientation: Orientation) -> Axis {
        Axis {
            orientation: orientation,
            coords: Frame::new(),
            fig_frame: Frame::new(),
            color: [0.0, 0.0, 0.0, 1.0],
            line_width: 0.005,
            label: Text::new(""),
            label_offset: 0.12,
            data_range: [0.0, 1.0],
            ref_num_ticks: 5,
            grid: true,
            gridlines: Vec::<GridLine>::new(),
            ticks: Vec::<Tick>::new(),
        }
    }

    pub fn set(&mut self, x_start: f64, x_end: f64, y_start: f64, y_end: f64) {
        self.coords.set(x_start, x_end, y_start, y_end);
    }

    /// Fit
    ///
    /// This function is called just before draw(), and updates the default w.r.t. user input.
    /// and changes above in the hierarchy (canvas -> plot -> figure).
    pub fn fit(&mut self, fig_frame: Frame, data_frame: Frame) {
        // Compute marks here
        self.marks = utils::compute_marks();

        let factor = x_factor.max(y_factor);
        self.label_offset *= factor;
        self.line_width *= factor;
        self.label.scale_size(factor);
        self.fig_frame.set(x_factor * self.fig_frame.x_min(), x_factor * self.fig_frame.x_max(),
                           y_factor * self.fig_frame.y_min(), y_factor * self.fig_frame.y_max());

        for gridline in self.gridlines.iter_mut() {
            gridline.fit(x_factor, y_factor);
        }
        for tick in self.ticks.iter_mut() {
            tick.scale_size(factor);
        }
    }

    pub fn draw(&self, cr: &Context) {

        // Axis line
        cr.set_source_rgba(self.color[0], self.color[1], self.color[2], self.color[3]);
        cr.set_line_width(self.line_width);
        cr.move_to(self.coords.x_min(), self.coords.y_min());
        cr.line_to(self.coords.x_max(), self.coords.y_max());
        cr.stroke();

        // Label
        cr.select_font_face("Serif", FontSlant::Italic, FontWeight::Normal);
        cr.set_font_size(self.label.font_size());
        match self.orientation {
            Orientation::Horizontal => {
                cr.move_to((self.coords.x_min() + self.coords.x_max()) / 2.0,
                           self.coords.y_min() + self.label_offset);
                cr.show_text(&self.label.content());
            },
            Orientation::Vertical => {
                // TODO: Rotate label so that it is vertical
                cr.move_to(self.coords.x_min() - self.label_offset,
                           (self.coords.y_min() + self.coords.y_max()) / 2.0);
                cr.show_text(&self.label.content());
            },
        }

        // Gridlines
        if self.grid {
            for tick in ticks.iter_mut() {
                // FIXME: Provide information about the plot height the x-axis and plot width for
                // the y axis.
                match self.orientation {
                    Orientation::Horizontal => {
                        let mut gridline = GridLine::new(tick.x_center(), tick.y_center(),
                                                         tick.x_center(), self.plot_frame.y_min());
                        gridline.scale_size(scale_factor);
                        gridline.draw(cr);
                    }
                    Orientation::Vertical => {
                        let mut gridline = GridLine::new(tick.x_center(), tick.y_center(),
                                                         self.plot_frame.x_max(), tick.y_center());
                        gridline.scale_size(scale_factor);
                        gridline.draw(cr);
                    },
                };
            }
        }

        // Ticks and tick labels
        for tick in ticks.iter_mut() {
            tick.scale_size(scale_factor);
            tick.draw(cr);
        }
    }
}
