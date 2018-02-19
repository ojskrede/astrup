//! Definition of the Axis struct
//!

use std::f64;
use failure::{Error, err_msg};

use cairo::{Context};

use ::{utils, coord, shape, label, mark, color};

/// ## Axis
///
/// An axis is a reference source for the plot.
#[derive(Clone, Debug)]
pub struct Axis {
    local_start: coord::Coord,
    local_end: coord::Coord,
    global_start: coord::Coord,
    global_end: coord::Coord,
    direction: coord::Coord,
    color: color::Color,
    line_width: f64,
    data_range: [f64; 2],
    label: label::Label,
    ca_num_marks: usize,
    marks: Vec<mark::Mark>,
}

impl Axis {
    pub fn new() -> Axis {
        Axis {
            local_start: coord::Coord::new(),
            local_end: coord::Coord::new(),
            global_start: coord::Coord::new(),
            global_end: coord::Coord::new(),
            direction: coord::Coord::new(),
            color: color::Color::new(),
            line_width: 0.0025,
            data_range: [0.0, 1.0],
            label: label::Label::new(),
            ca_num_marks: 7,
            marks: Vec::<mark::Mark>::new(),
        }
    }

    pub fn new_from(start: coord::Coord, end: coord::Coord) -> Axis {
        Axis {
            local_start: start.clone(),
            local_end: end.clone(),
            global_start: coord::Coord::new(),
            global_end: coord::Coord::new(),
            direction: start.unit_direction_to(&end),
            color: color::Color::new(),
            line_width: 0.0025,
            data_range: [0.0, 1.0],
            label: label::Label::new(),
            ca_num_marks: 7,
            marks: Vec::<mark::Mark>::new(),
        }
    }

    pub fn set_color(&mut self, color_name: &str) {
        self.color.set_color_default(color_name);
    }

    pub fn set_color_rgb(&mut self, red: f32, green: f32, blue: f32) {
        self.color.set_color_rgb(red, green, blue);
    }

    pub fn set_color_rgba(&mut self, red: f32, green: f32, blue: f32, alpha: f32) {
        self.color.set_color_rgba(red, green, blue, alpha);
    }

    pub fn set_color_rgb_u8(&mut self, red: u8, green: u8, blue: u8) {
        self.color.set_color_rgb_u8(red, green, blue);
    }

    pub fn set_color_rgba_u8(&mut self, red: u8, green: u8, blue: u8, alpha: u8) {
        self.color.set_color_rgba_u8(red, green, blue, alpha);
    }

    pub fn set_color_str(&mut self, color_name: &str) -> Result<(), Error> {
        self.color.set_color_str(color_name)?;
        Ok(())
    }

    pub fn set_line_width(&mut self, val: f64) {
        self.line_width = val;
    }

    pub fn set_label(&mut self, label: &label::Label) {
        self.label = label.clone();
    }

    pub fn set_label_content(&mut self, content: &str) {
        self.label.set_content(content);
    }

    pub fn set_label_angle(&mut self, angle: f64) {
        self.label.set_angle(angle);
    }

    pub fn set_label_centroid(&mut self, x_coord: f64, y_coord: f64) {
        self.label.set_centroid(x_coord, y_coord)
    }

    pub fn set_label_font_size(&mut self, val: f64) {
        self.label.set_font_size(val);
    }

    pub fn set_num_ticks(&mut self, val: usize) {
        self.ca_num_marks = val;
    }

    pub fn set_positive_tick_length(&mut self, val: f64) {
        for mark in self.marks.iter_mut() {
            mark.set_positive_tick_length(val);
        }
    }

    pub fn set_negative_tick_length(&mut self, val: f64) {
        for mark in self.marks.iter_mut() {
            mark.set_negative_tick_length(val);
        }
    }

    pub fn set_tick_label_font_size(&mut self, val: f64) {
        for mark in self.marks.iter_mut() {
            mark.set_font_size(val);
        }
    }

    pub fn set_tick_label_offset(&mut self, val: f64) {
        for mark in self.marks.iter_mut() {
            mark.set_label_offset(val);
        }
    }

    /// Set the gaps around the tick label, for all tick labels on this axis. See the Label struct
    /// for reference.
    pub fn set_tick_label_frame_gaps(&mut self, left: f64, right: f64, bottom: f64, top: f64) {
        for mark in self.marks.iter_mut() {
            mark.set_label_frame_gaps(left, right, bottom, top);
        }
    }

    pub fn set_data_range(&mut self, data_min: f64, data_max: f64) {
        self.data_range = [data_min, data_max];
    }

    /// Set the gaps around the tick label, for all tick labels on this axis. See the Label struct
    /// for reference.
    pub fn set_label_frame_gaps(&mut self, left: f64, right: f64, bottom: f64, top: f64) {
        self.label.set_frame_gaps(left, right, bottom, top);
    }

    pub fn data_min(&self) -> f64 {
        self.data_range[0]
    }

    pub fn data_max(&self) -> f64 {
        self.data_range[1]
    }

    pub fn mark_coords(&self) -> Vec<coord::Coord> {
        let mut coords = Vec::<coord::Coord>::new();
        for mark in self.marks.iter() {
            coords.push(mark.global_coord());
        }

        coords
    }

    /// ## Compute marks
    ///
    /// Marks are used to determine the location of ticks and gridlines.
    ///
    /// This method will return a list of evenly spaced marks according to the following method.
    /// This assumes that the data range is known, and that know how many marks we want. The latter
    /// is determined by a variable, and will be used more of a guide than as the actual number of
    /// marks we get in the end.
    ///
    /// ### Method
    ///
    /// 1. Find the orider of magnitude of the difference in the data range. Call this *p*.
    /// 2a. Let min_point be min(data) rounded down to nearest *10^(p - 2)*.
    /// 2b. Let max_point be max(data) rounded up to nearest *10^(p - 2)*.
    /// 3. mark_distance = (max_point - min_point) / num_labels rounded to nearest *10^(p - 2)*
    /// 4. Then, let mark_k = min_point + k*mark_distance, for k = 0 until mark_k is greater or
    ///    equal to max(data).
    /// 5. Transform between labels in the data framework (the above) and positions in the drawing
    ///    framework using the data range and axis frame.
    ///
    ///
    ///  - The user can now set data range, but this function will override it. With this, the
    ///  output looks nicer, but I assume that when the user puts a data range, the user assumes
    ///  that this range should be used.
    pub fn compute_marks(&mut self) -> Result<(), Error> {
        let data_diff = self.data_range[1] - self.data_range[0];
        let ca_dist = data_diff / self.ca_num_marks as f64;
        let omagn = utils::order_of_magnitude(ca_dist);

        // Find for what k in (2, 5, 10) we shall round to the nearest ten power of
        let mut smallest_diff = f64::MAX;
        let mut round_number = 0f64;
        for &i in [2.0, 5.0, 10.0].iter() {
            let nearest = utils::round_nearest(ca_dist, omagn, i);
            let diff = (ca_dist - nearest).abs();
            if diff < smallest_diff {
                smallest_diff = diff;
                round_number = i;
            }
        }

        let actual_min_point = utils::round_down(self.data_range[0], omagn, round_number);
        let ca_max_point = *self.data_range.last().ok_or(err_msg("No final element"))?;
        let mark_distance = utils::round_nearest(ca_dist, omagn, round_number);

        let mut data_locations = vec![actual_min_point];
        let mut data_location_k = actual_min_point;
        let mut marks = Vec::<mark::Mark>::new();
        let mut add_next = true;
        while add_next {
            data_location_k += mark_distance;
            data_locations.push(data_location_k);
            if data_location_k > ca_max_point {
                add_next = false;
            }
        }
        let min_data = data_locations[0];
        let max_data = *data_locations.last().ok_or(err_msg("No final element"))?;
        for data_location in data_locations {
            let mark_x = utils::map_range(data_location, min_data, max_data,
                                          self.local_start.x(), self.local_end.x());
            let mark_y = utils::map_range(data_location, min_data, max_data,
                                          self.local_start.y(), self.local_end.y());
            let mark_location = coord::Coord::new_from(mark_x, mark_y);
            let mut mark_k = mark::Mark::new_from_coord(mark_location);
            mark_k.set_label_content(&utils::prettify(data_location));

            marks.push(mark_k);

        }
        self.data_range = [min_data, max_data];
        self.marks = marks;

        Ok(())
    }

    fn scale_size(&mut self, factor: f64) {
        self.line_width *= factor;
    }

    /// Fit
    ///
    /// This function is called just before draw(), and updates the default w.r.t. user input.
    /// and changes above in the hierarchy (canvas -> plot -> figure).
    pub fn fit(&mut self, canvas_frame: &shape::Rectangle) {
        // Local coordinates are determined from initialization or user input.
        self.global_start = self.local_start.relative_to(canvas_frame);
        self.global_end = self.local_end.relative_to(canvas_frame);
        let unit_perp_direction = self.global_start.perp_direction(&self.global_end);
        let scale_factor = canvas_frame.diag_len();
        self.scale_size(scale_factor);

        self.label.fit(canvas_frame);

        for mark in self.marks.iter_mut() {
            mark.set_tick_direction(&unit_perp_direction);
            let label_x = mark.local_x() + unit_perp_direction.x().abs() * mark.label_offset();
            let label_y = mark.local_y() + unit_perp_direction.y().abs() * mark.label_offset();
            mark.set_label_centroid(label_x, label_y);
            mark.fit(canvas_frame);
        }
    }

    /// Draw axis on canvas.
    pub fn draw(&self, cr: &Context, fig_rel_height: f64, fig_rel_width: f64) {
        // Draw ticks and tick labels
        for mark in self.marks.iter() {
            mark.draw(cr, fig_rel_height, fig_rel_width);
        }

        // Draw axis line
        let line_color = self.color.as_srgba();
        cr.set_source_rgba(line_color.red as f64, line_color.green as f64,
                           line_color.blue as f64, line_color.alpha as f64);
        cr.set_line_width(self.line_width * (self.direction.x().abs() * fig_rel_width +
                                             self.direction.y().abs() * fig_rel_height));
        cr.move_to(self.global_start.x(), self.global_start.y());
        cr.line_to(self.global_end.x(), self.global_end.y());
        cr.stroke();

        // Draw axis label
        self.label.draw(cr, fig_rel_height, fig_rel_width);
    }
}
