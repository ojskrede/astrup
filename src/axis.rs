//! Definition of the Axis struct
//!

use std::f64;
use failure::{err_msg, Error};
use palette::Srgba;

use cairo::{Context, FontSlant, FontWeight};

use {color, coord, label, mark, shape, utils};

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
    #[allow(dead_code)]
    pub fn new() -> Axis {
        let mut label = label::Label::new();
        label.set_color_internal(color::CustomColor::AxisLabel.as_srgba());
        Axis {
            local_start: coord::Coord::new(),
            local_end: coord::Coord::new(),
            global_start: coord::Coord::new(),
            global_end: coord::Coord::new(),
            direction: coord::Coord::new(),
            color: color::Color::with_custom(&color::CustomColor::AxisLine),
            line_width: 0.0025,
            data_range: [0.0, 1.0],
            label: label,
            ca_num_marks: 6,
            marks: Vec::<mark::Mark>::new(),
        }
    }

    pub fn with_boundaries(start: &coord::Coord, end: &coord::Coord) -> Axis {
        let mut label = label::Label::new();
        label.set_color_internal(color::CustomColor::AxisLabel.as_srgba());
        Axis {
            local_start: start.clone(),
            local_end: end.clone(),
            global_start: coord::Coord::new(),
            global_end: coord::Coord::new(),
            direction: start.unit_direction_to(end),
            color: color::Color::with_custom(&color::CustomColor::AxisLine),
            line_width: 0.0025,
            data_range: [0.0, 1.0],
            label: label,
            ca_num_marks: 6,
            marks: Vec::<mark::Mark>::new(),
        }
    }

    // ----------------- APPEARANCE ---------------------------------------- //

    /// Set axis color
    pub fn set_color_internal(&mut self, color: Srgba) {
        self.color.set_color(color);
    }

    pub fn set_line_width(&mut self, val: f64) {
        self.line_width = val;
    }

    // ----------------- LABELS -------------------------------------------- //

    pub fn set_label(&mut self, label: &label::Label) {
        self.label = label.clone();
    }

    #[allow(dead_code)] // TODO: When axis becomes public
    pub fn set_label_content(&mut self, content: &str) {
        self.label.set_content(content);
    }

    #[allow(dead_code)] // TODO: When axis becomes public
    pub fn set_label_angle(&mut self, angle: f64) {
        self.label.set_angle(angle);
    }

    #[allow(dead_code)] // TODO: When axis becomes public
    pub fn set_label_centroid(&mut self, x_coord: f64, y_coord: f64) {
        self.label.set_centroid(x_coord, y_coord)
    }

    pub fn set_label_font_size(&mut self, val: f64) {
        self.label.set_font_size(val);
    }

    pub fn set_label_font_slant(&mut self, font_slant: FontSlant) {
        self.label.set_font_slant(font_slant);
    }

    pub fn set_label_font_weight(&mut self, font_weight: FontWeight) {
        self.label.set_font_weight(font_weight);
    }

    pub fn set_label_font_family(&mut self) {
        // TODO:
        self.label.set_font_family();
    }

    /// Set the gaps around the axis label. See the Label struct for reference.
    #[allow(dead_code)] // TODO: When axis becomes public
    pub fn set_label_frame_gaps(&mut self, left: f64, right: f64, bottom: f64, top: f64) {
        self.label.set_frame_gaps(left, right, bottom, top);
    }

    // ----------------- TICKS --------------------------------------------- //

    #[allow(dead_code)] // TODO: When axis becomes public
    pub fn set_num_ticks(&mut self, val: usize) {
        self.ca_num_marks = val;
    }

    pub fn set_positive_tick_length(&mut self, val: f64) {
        for mark in &mut self.marks {
            mark.set_positive_tick_length(val);
        }
    }

    pub fn set_negative_tick_length(&mut self, val: f64) {
        for mark in &mut self.marks {
            mark.set_negative_tick_length(val);
        }
    }

    pub fn set_tick_color_internal(&mut self, color: Srgba) {
        for mark in &mut self.marks {
            mark.set_tick_color_internal(color);
        }
    }

    pub fn set_tick_label_font_size(&mut self, val: f64) {
        for mark in &mut self.marks {
            mark.set_font_size(val);
        }
    }

    pub fn set_tick_label_font_slant(&mut self, font_slant: FontSlant) {
        for mark in &mut self.marks {
            mark.set_font_slant(font_slant);
        }
    }

    pub fn set_tick_label_font_weight(&mut self, font_weight: FontWeight) {
        for mark in &mut self.marks {
            mark.set_font_weight(font_weight);
        }
    }

    pub fn set_tick_label_font_family(&mut self) {
        // TODO:
        for mark in &mut self.marks {
            mark.set_font_family();
        }
    }

    pub fn set_tick_label_color_internal(&mut self, color: Srgba) {
        for mark in &mut self.marks {
            mark.set_label_color_internal(color);
        }
    }

    pub fn set_tick_label_offset(&mut self, val: f64) {
        for mark in &mut self.marks {
            mark.set_label_offset(val);
        }
    }

    /// Set the gaps around the tick label, for all tick labels on this axis. See the Label struct
    /// for reference.
    pub fn set_tick_label_frame_gaps(&mut self, left: f64, right: f64, bottom: f64, top: f64) {
        for mark in &mut self.marks {
            mark.set_label_frame_gaps(left, right, bottom, top);
        }
    }

    // ----------------- DATA RANGE ---------------------------------------- //

    pub fn set_data_range(&mut self, data_min: f64, data_max: f64) {
        self.data_range = [data_min, data_max];
    }

    pub fn data_min(&self) -> f64 {
        self.data_range[0]
    }

    pub fn data_max(&self) -> f64 {
        self.data_range[1]
    }

    // ----------------- GENERAL INTERNAL ---------------------------------- //

    /// Return the coordinates of the marks of this axis. Coordinates are relative to the global
    /// figure frame.
    pub fn mark_coords(&self) -> Vec<coord::Coord> {
        let mut coords = Vec::<coord::Coord>::new();
        for mark in &self.marks {
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
        let ca_dist = data_diff / (self.ca_num_marks as f64 - 1.0);
        let omagn = utils::order_of_magnitude(ca_dist);

        // Find for what k in (1, 2, 5) we shall round to the nearest ten power of
        let mut smallest_diff = f64::MAX;
        let mut round_number = 0f64;
        for &i in &[1.0, 2.0, 5.0] {
            let nearest = utils::round_nearest(ca_dist, omagn, i);
            let diff = (ca_dist - nearest).abs();
            if diff < smallest_diff {
                smallest_diff = diff;
                round_number = i;
            }
        }

        let actual_min_point = utils::round_down(self.data_range[0], omagn, round_number);
        let ca_max_point = *self.data_range
            .last()
            .ok_or_else(|| err_msg("No final element"))?;
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
        let max_data = *data_locations
            .last()
            .ok_or_else(|| err_msg("No final element"))?;
        for data_location in data_locations {
            let mark_x = utils::map_range(
                data_location,
                min_data,
                max_data,
                self.local_start.x(),
                self.local_end.x(),
            );
            let mark_y = utils::map_range(
                data_location,
                min_data,
                max_data,
                self.local_start.y(),
                self.local_end.y(),
            );
            let mark_location = coord::Coord::with_coordinates(mark_x, mark_y);
            let mut mark_k = mark::Mark::with_location(mark_location);
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
    pub(crate) fn fit(&mut self, canvas_frame: &shape::Rectangle) {
        // Local coordinates are determined from initialization or user input.
        self.global_start = self.local_start.relative_to(canvas_frame);
        self.global_end = self.local_end.relative_to(canvas_frame);
        let unit_perp_direction = self.global_start.perp_direction(&self.global_end);
        let scale_factor = canvas_frame.diag_len();
        self.scale_size(scale_factor);

        self.label.fit(canvas_frame);

        for mark in &mut self.marks {
            mark.set_tick_direction(&unit_perp_direction);
            let label_x = mark.local_x() + unit_perp_direction.x().abs() * mark.label_offset();
            let label_y = mark.local_y() + unit_perp_direction.y().abs() * mark.label_offset();
            mark.set_label_centroid(label_x, label_y);
            mark.fit(canvas_frame);
        }
    }

    /// Draw axis on canvas.
    pub(crate) fn draw(&self, cr: &Context, fig_rel_height: f64, fig_rel_width: f64) {
        // Draw ticks and tick labels
        for mark in &self.marks {
            mark.draw(cr, fig_rel_height, fig_rel_width);
        }

        // Draw axis line
        let line_color = self.color.as_srgba();
        cr.set_source_rgba(
            f64::from(line_color.red),
            f64::from(line_color.green),
            f64::from(line_color.blue),
            f64::from(line_color.alpha),
        );
        cr.set_line_width(
            self.line_width
                * (self.direction.x().abs() * fig_rel_width
                    + self.direction.y().abs() * fig_rel_height),
        );
        cr.move_to(self.global_start.x(), self.global_start.y());
        cr.line_to(self.global_end.x(), self.global_end.y());
        cr.stroke();

        // Draw axis label
        self.label.draw(cr, fig_rel_height, fig_rel_width);
    }
}
