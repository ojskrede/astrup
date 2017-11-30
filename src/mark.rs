//! Mark module
//!
//!

use cairo::Context;
use cairo::enums::{FontSlant, FontWeight};

use utils::{Frame, Text, map_range, round_out, round_nearest};

use axis::Orientation;

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

    pub fn fig_mark(&self) -> f64 { self.fig }

    pub fn data_mark(&self) -> f64 { self.data }
}


/// ## Tick
///
/// Indicator used by axis to serve as a reference for the displayed data
#[derive(Clone, Debug)]
pub struct Tick {
    orientation: Orientation,
    x_center: f64,
    y_center: f64,
    color: [f64; 4],
    line_width: f64,
    label: Text,
    label_offset: f64,
}

impl Tick {
    pub fn new(orientation: Orientation, x_center: f64, y_center: f64) -> Tick {
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

    pub fn set_label(&mut self, content: &str) {
        self.label.set_content(content);
    }

    pub fn set_color(&mut self, color: [f64; 4]) {
        self.color = color;
    }

    fn set_label_size(&mut self, size: f64) {
        self.label.set_font_size(size);
    }

    fn orientation(&self) -> Orientation {
        self.orientation.clone()
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

    pub fn fit(&mut self, axis_frame: Frame) {
        self.scale_size(axis_frame.diag_len());
    }

    pub fn draw(&self, cr: &Context) {
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

/// ## Compute marks
///
/// Marks are used by axis ticks, and axis gridlines, to determine their location.
///
/// This method will return a list of evenly spaced marks according to the following method.
/// This assumes that the data range is known, and that know how many marks we want. The latter
/// is determined by a variable, and will be used more of a guide than as the actual number of
/// marks we get in the end.
///
/// ### Method
///
/// 1. Find the orider of magnitude of the difference in the data range. Call this omagn.
/// 2a. Let min_point be min(data) rounded down to nearest 10^(omagn - 2).
/// 2b. Let max_point be max(data) rounded up to nearest 10^(omagn - 2).
/// 3. mark_distance = (max_point - min_point) / num_labels rounded to nearest 10^(omagn - 2)
/// 4. Then, let mark_k = min_point + k*mark_distance, for k = 0 until mark_k is greater or
///    equal to max(data).
/// 5. Transform between labels in the data framework (the above) and positions in the drawing
///    framework using the data range and axis frame.
///
///
/// TODO:
///  - Add a feature that only accepts marks at locations 10^k * {1, 2, 5} for integer k.
///  - Compute the martk data location based on largest data frame. Then update the axis' data
///  range to be cover (be the same as) its mark data range. Then adjust the plot location of
///  its marks, data, gridlines, etc. Currently the axis range is determined by the range of
///  the data, and not the range of its marks. Also, the user should be able to set the data
///  range, this should then determine the mark range, which in turn should determine the axis
///  range.
pub fn compute_mark_locations(ca_num_marks: usize, ref_min: f64, ref_max: f64,
                          data_min: f64, data_max: f64) -> Vec<Mark> {
    let data_diff = data_max - data_min;
    let omagn = data_diff.log10().ceil();
    let actual_min_point = round_out(data_min, omagn);
    let ca_max_point = round_out(data_max, omagn);
    let mark_distance = round_nearest((ca_max_point - actual_min_point) / ca_num_marks as f64, omagn);

    let mut data_location_k = actual_min_point;
    let mut marks = Vec::<Mark>::new();
    let mut add_next = true;
    while add_next {
        if data_location_k > ca_max_point {
            add_next = false;
        }

        let ref_location_k = map_range(data_location_k, data_min, data_max, ref_min, ref_max);
        let mut mark_k = Mark::new();
        mark_k.set(ref_location_k, data_location_k);

        marks.push(mark_k);
        data_location_k += mark_distance;
    }
    marks
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
