//! Axis module
//!

use std::f64::MAX;

use cairo::{Context, Matrix, MatrixTrait};
use cairo::enums::{FontSlant, FontWeight};

use utils::{Coord, Frame, Text, round_down, round_nearest, map_range};
use mark::{Mark, Tick, prettify};

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
    local_start: Coord,
    local_end: Coord,
    global_start: Coord,
    global_end: Coord,
    color: [f64; 4],
    line_width: f64,
    data_range: [f64; 2],
    label: Text,
    ca_num_marks: usize,
    tick_color: [f64; 4],
    tick_length: f64,
    tick_width: f64,
    marks: Vec<Mark>,
    ticks: Vec<Tick>,
}

impl Axis {
    pub fn new() -> Axis {
        Axis {
            local_start: Coord::new(0.0, 0.0),
            local_end: Coord::new(0.0, 0.0),
            global_start: Coord::new(0.0, 0.0),
            global_end: Coord::new(0.0, 0.0),
            color: [0.0, 0.0, 0.0, 1.0],
            line_width: 0.005,
            data_range: [0.0, 1.0],
            label: Text::new(""),
            ca_num_marks: 6,
            tick_color: [0.0, 0.0, 0.0, 1.0],
            tick_length: 0.05,
            tick_width: 0.01,
            marks: Vec::<Mark>::new(),
            ticks: Vec::<Tick>::new(),
        }
    }

    // TODO: Impl default?
    pub fn from_coord(start: Coord, end: Coord) -> Axis {
        Axis {
            local_start: start,
            local_end: end,
            global_start: Coord::new(0.0, 0.0),
            global_end: Coord::new(0.0, 0.0),
            color: [0.0, 0.0, 0.0, 1.0],
            line_width: 0.005,
            data_range: [0.0, 1.0],
            label: Text::new(""),
            ca_num_marks: 6,
            tick_color: [0.0, 0.0, 0.0, 1.0],
            tick_length: 0.04,
            tick_width: 0.005,
            marks: Vec::<Mark>::new(),
            ticks: Vec::<Tick>::new(),
        }
    }

    pub fn set_label(&mut self, content: &str) {
        self.label.set_content(content);
    }

    pub fn set_label_angle(&mut self, angle: f64) {
        self.label.set_angle(angle);
    }

    pub fn scale_label_offset(&mut self, factor: f64) {
        self.label.scale_offset(factor);
    }

    pub fn scale_tick_length(&mut self, factor: f64) {
        self.tick_length *= factor;
    }

    pub fn set_tick_width(&mut self, val: f64) {
        self.tick_width = val;
    }

    pub fn set_tick_font_size(&mut self, val: f64) {
        for mark in self.marks.iter_mut() {
            mark.set_font_size(val);
        }
    }

    pub fn set_tick_label_offset(&mut self, hor: f64, ver: f64) {
        for mark in self.marks.iter_mut() {
            mark.set_label_offset(hor, ver);
        }
    }

    pub fn scale_tick_label_offset(&mut self, factor: f64) {
        for mark in self.marks.iter_mut() {
            mark.scale_label_offset(factor);
        }
    }

    pub fn set_data_range(&mut self, data_min: f64, data_max: f64) {
        self.data_range = [data_min, data_max];
    }

    pub fn set_label_offset(&mut self, hor: f64, ver: f64) {
        self.label.set_offset(hor, ver);
    }

    pub fn data_min(&self) -> f64 {
        self.data_range[0]
    }

    pub fn data_max(&self) -> f64 {
        self.data_range[1]
    }

    pub fn mark_coords(&self) -> Vec<Coord> {
        let mut coords = Vec::<Coord>::new();
        for mark in self.marks.iter() {
            coords.push(mark.global_coord());
        }

        coords
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
    pub fn compute_marks(&mut self) {
        let data_diff = self.data_range[1] - self.data_range[0];
        let ca_dist = data_diff / self.ca_num_marks as f64;
        let omagn = ca_dist.abs().log10().floor();

        // Find for what k in (2, 5, 10) we shall round to the nearest ten power of
        let mut smallest_diff = MAX;
        let mut round_number = 0f64;
        for &i in [2.0, 5.0, 10.0].iter() {
            let nearest = round_nearest(ca_dist, omagn, i);
            let diff = (ca_dist - nearest).abs();
            if diff < smallest_diff {
                smallest_diff = diff;
                round_number = i;
            }
        }

        let actual_min_point = round_down(self.data_range[0], omagn, round_number);
        let ca_max_point = *self.data_range.last().unwrap();
        let mark_distance = round_nearest(ca_dist, omagn, round_number);

        let mut data_locations = vec![actual_min_point];
        let mut data_location_k = actual_min_point;
        let mut marks = Vec::<Mark>::new();
        let mut add_next = true;
        while add_next {
            data_location_k += mark_distance;
            data_locations.push(data_location_k);
            if data_location_k > ca_max_point {
                add_next = false;
            }
        }
        let min_data = data_locations[0];
        let max_data = *data_locations.last().unwrap();
        for data_location in data_locations {
            let mark_x = map_range(data_location, min_data, max_data,
                                   self.local_start.x(), self.local_end.x());
            let mark_y = map_range(data_location, min_data, max_data,
                                   self.local_start.y(), self.local_end.y());
            let mark_location = Coord::new(mark_x, mark_y);
            let mut mark_k = Mark::new(mark_location);
            mark_k.set_label_content(&prettify(data_location));

            marks.push(mark_k);

        }
        self.data_range = [min_data, max_data];
        self.marks = marks;
    }

    fn scale_size(&mut self, factor: f64) {
        self.tick_length *= factor;
        self.tick_width *= factor;
        self.line_width *= factor;
        self.label.scale_size(factor);
    }

    /// Fit
    ///
    /// This function is called just before draw(), and updates the default w.r.t. user input.
    /// and changes above in the hierarchy (canvas -> plot -> figure).
    pub fn fit(&mut self, canvas_frame: &Frame) {
        // Local coordinates are determined from initialization or user input.
        self.global_start = self.local_start.relative_to(&canvas_frame);
        self.global_end = self.local_end.relative_to(&canvas_frame);
        let scale_factor = canvas_frame.diag_len() / 2f64.sqrt();
        self.scale_size(scale_factor);

        for mark in self.marks.iter_mut() {
            mark.fit(canvas_frame);
        }
    }

    /// Draw axis on canvas.
    pub fn draw(&self, cr: &Context) {
        // Ticks
        let unit_perp = self.global_start.perp_direction(&self.global_end);
        for mark in self.marks.iter() {
            cr.set_source_rgba(self.tick_color[0], self.tick_color[1], self.tick_color[2],
                               self.tick_color[3]);
            cr.set_line_width(self.tick_width);
            cr.move_to(mark.global_x(), mark.global_y());
            cr.line_to(mark.global_x() + unit_perp.x() * self.tick_length,
                       mark.global_y() + unit_perp.y() * self.tick_length);
            cr.stroke();

            cr.select_font_face("Serif", FontSlant::Normal, FontWeight::Normal);
            cr.set_font_size(mark.label().font_size());
            cr.move_to(mark.global_x() + mark.label_hor_offset(),
                       mark.global_y() + mark.label_ver_offset());

            cr.transform(Matrix::new(1.0, 0.0, 0.0, -1.0, 0.0, 0.0));
            cr.rotate(self.label.angle());
            cr.show_text(&mark.label().content());
            cr.rotate(-self.label.angle());
            cr.transform(Matrix::new(1.0, 0.0, 0.0, -1.0, 0.0, 0.0));
        }

        // Axis line
        cr.set_source_rgba(self.color[0], self.color[1], self.color[2], self.color[3]);
        cr.set_line_width(self.line_width);
        cr.move_to(self.global_start.x(), self.global_start.y());
        cr.line_to(self.global_end.x(), self.global_end.y());
        cr.stroke();

        // Label
        cr.select_font_face("Serif", FontSlant::Italic, FontWeight::Normal);
        cr.set_font_size(self.label.font_size());
        // TODO: Shift label "backwards" based on its length
        //let mid_norm = self.global_start.perp_bisector(&self.global_end, self.label_offset);
        //cr.move_to(mid_norm.x(), mid_norm.y());
        let mid_point_x = (self.global_start.x() + self.global_end.x()) / 2.0;
        let mid_point_y = (self.global_start.y() + self.global_end.y()) / 2.0;
        cr.move_to(mid_point_x + self.label.hor_offset(), mid_point_y + self.label.ver_offset());
        cr.transform(Matrix::new(1.0, 0.0, 0.0, -1.0, 0.0, 0.0));
        cr.rotate(self.label.angle());
        cr.show_text(&self.label.content());
        cr.rotate(-self.label.angle());
        cr.transform(Matrix::new(1.0, 0.0, 0.0, -1.0, 0.0, 0.0));

        // Ticks and tick labels
        //for tick in self.ticks.iter() {
            //tick.draw(cr);
        //}
    }
}
