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
    plot_x_start: f64,
    plot_x_end: f64,
    plot_y_start: f64,
    plot_y_end: f64,
    color: [f64; 4],
    line_width: f64,
}

impl GridLine {
    fn new(x_start: f64, y_start: f64, x_end: f64, y_end: f64) -> GridLine {
        GridLine {
            plot_x_start: x_start,
            plot_x_end: x_end,
            plot_y_start: y_start,
            plot_y_end: y_end,
            color: [1.0, 1.0, 1.0, 1.0],
            line_width: 0.005,
        }
    }

    fn scale_size(&mut self, factor: f64) {
        self.line_width *= factor;
    }

    fn draw(&self, cr: &Context) {
        cr.set_source_rgba(self.color[0], self.color[1], self.color[2], self.color[3]);
        cr.set_line_width(self.line_width);
        cr.move_to(self.plot_x_start, self.plot_y_start);
        cr.line_to(self.plot_x_end, self.plot_y_end);
        cr.stroke();
    }
}

/// ## Axis
///
/// An axis is a reference source for the plot. It is often displayed as a line with evenly spaced
/// ticks. The ticks are often labeled, and so is also the whole axis.
#[derive(Clone, Debug)]
pub struct Axis {
    orientation: Orientation,
    plot_coords: Frame,
    plot_frame: Frame,
    color: [f64; 4],
    line_width: f64,
    label: Text,
    label_offset: f64,
    data_range: [f64; 2],
    ref_num_ticks: usize,
    grid: bool,
}

impl Axis {
    pub fn new(orientation: Orientation, x_start: f64, x_end: f64, y_start: f64, y_end: f64) -> Axis {
        Axis {
            orientation: orientation,
            plot_coords: Frame::new(x_start, x_end, y_start, y_end),
            plot_frame: Frame::new(0.0, 1.0, 0.0, 1.0),
            color: [0.0, 0.0, 0.0, 1.0],
            line_width: 0.005,
            label: Text::new(""),
            label_offset: 0.12,
            data_range: [0.0, 1.0],
            ref_num_ticks: 5,
            grid: true,
        }
    }

    pub fn set_label(&mut self, label: &str) {
        self.label.set_content(label)
    }

    pub fn set_data_range(&mut self, min: f64, max: f64) {
        self.data_range = [min, max];
    }

    pub fn data_min(&self) -> f64 {
        self.data_range[0]
    }

    pub fn data_max(&self) -> f64 {
        self.data_range[1]
    }

    pub fn plot_x_start(&self) -> f64 {
        self.plot_coords.x_min()
    }

    pub fn plot_x_end(&self) -> f64 {
        self.plot_coords.x_max()
    }

    pub fn plot_y_start(&self) -> f64 {
        self.plot_coords.y_min()
    }

    pub fn plot_y_end(&self) -> f64 {
        self.plot_coords.y_min()
    }

    pub fn line_width(&self) -> f64 {
        self.line_width
    }

    pub fn scale_size(&mut self, factor: f64) {
        self.label_offset *= factor;
        self.line_width *= factor;
        self.label.scale_size(factor);
    }

    pub fn set_plot_coords(&mut self, frame: Frame) {
        self.plot_coords = frame;
    }

    pub fn set_plot_frame(&mut self, frame: Frame) {
        self.plot_frame = frame;
    }

    /// ## Compute ticks
    ///
    /// This method will return a list of evenly spaced ticks according to the following method.
    /// This assumes that the data range is known, and that know how many ticks we want. The latter
    /// is determined by a variable, and will be used more of a guide than as the actual number of
    /// ticks we get in the end.
    ///
    /// ### Method
    ///
    /// 1. Find the orider of magnitude of the difference in the data range. Call this omagn.
    /// 2a. Let min_point be min(data) rounded down to nearest 10^(omagn - 2).
    /// 2b. Let max_point be max(data) rounded up to nearest 10^(omagn - 2).
    /// 3. tick_distance = (max_point - min_point) / num_labels rounded to nearest 10^(omagn - 2)
    /// 4. Then, let tick_k = min_point + k*tick_distance, for k = 0 until tick_k is greater or
    ///    equal to max(data).
    /// 5. Transform between labels in the data framework (the above) and positions in the drawing
    ///    framework using the data range and axis frame.
    fn compute_ticks(&self) -> Vec<Tick> {
        let data_diff = self.data_max() - self.data_min();
        let omagn = data_diff.log10().ceil();
        let actual_min_point = utils::round_out(self.data_min(), omagn);
        let ref_max_point = utils::round_out(self.data_max(), omagn);
        let tick_distance = utils::round_nearest((ref_max_point - actual_min_point) / self.ref_num_ticks as f64, omagn);

        let plot_min = match self.orientation {
            Orientation::Horizontal => self.plot_coords.x_min(),
            Orientation::Vertical => self.plot_coords.y_min(),
        };
        let plot_max = match self.orientation {
            Orientation::Horizontal => self.plot_coords.x_max(),
            Orientation::Vertical => self.plot_coords.y_max(),
        };
        let mut data_loc_k = actual_min_point;
        let mut plot_ticks = Vec::<Tick>::new();
        while data_loc_k < ref_max_point {
            let plot_loc_k = utils::change_domain(data_loc_k, self.data_min(), self.data_max(),
                                                  plot_min, plot_max);
            let mut plot_tick = match self.orientation {
                Orientation::Horizontal => Tick::new(self.orientation.clone(), plot_loc_k, self.plot_coords.y_min()),
                Orientation::Vertical => Tick::new(self.orientation.clone(), self.plot_coords.x_min(), plot_loc_k),
            };
            // FIXME: Tick label format
            if omagn > 5.0 {
                plot_tick.set_label(&format!("{0:.0}", data_loc_k));
            } else if omagn > 0.0 {
                // TODO: Scientific notation
                plot_tick.set_label(&format!("{0:.0}", data_loc_k));
            } else if omagn < -5.0 {
                // TODO: Scientific notation
                plot_tick.set_label(&format!("{0:.1$}", data_loc_k, omagn.abs() as usize));
            } else {
                plot_tick.set_label(&format!("{0:.1$}", data_loc_k, omagn.abs() as usize));
            };
            plot_ticks.push(plot_tick);
            data_loc_k += tick_distance;
        }
        plot_ticks
    }

    pub fn draw(&self, cr: &Context) {

        // Axis line
        cr.set_source_rgba(self.color[0], self.color[1], self.color[2], self.color[3]);
        cr.set_line_width(self.line_width);
        cr.move_to(self.plot_coords.x_min(), self.plot_coords.y_min());
        cr.line_to(self.plot_coords.x_max(), self.plot_coords.y_max());
        cr.stroke();

        // Label
        cr.select_font_face("Serif", FontSlant::Italic, FontWeight::Normal);
        cr.set_font_size(self.label.font_size());
        match self.orientation {
            Orientation::Horizontal => {
                cr.move_to((self.plot_coords.x_min() + self.plot_coords.x_max()) / 2.0,
                           self.plot_coords.y_min() + self.label_offset);
                cr.show_text(&self.label.content());
            },
            Orientation::Vertical => {
                // TODO: Rotate label so that it is vertical
                cr.move_to(self.plot_coords.x_min() - self.label_offset,
                           (self.plot_coords.y_min() + self.plot_coords.y_max()) / 2.0);
                cr.show_text(&self.label.content());
            },
        }

        // FIXME: Move size scaling to a different place

        let x_scale_factor = self.plot_frame.x_max() - self.plot_frame.x_min();
        let y_scale_factor = self.plot_frame.y_max() - self.plot_frame.y_min();
        let scale_factor = x_scale_factor.max(y_scale_factor);


        // Gridlines
        let mut ticks = self.compute_ticks();
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
