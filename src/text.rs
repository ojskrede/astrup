//! Definition of the Text struct
//!

use palette::Srgba;
use cairo::{Context, Matrix, MatrixTrait};
use cairo::enums::{FontSlant, FontWeight};

use ::color;

/// A structure for text to be used in labels
#[derive(Clone, Debug)]
pub struct Text {
    content: String,
    font_size: f64,
    font_matrix: Matrix,
    font_slant: FontSlant,
    font_weight: FontWeight,
    font_family: String, // TODO: Enum
    color: color::Color,
}

impl Text {
    /// Create and return a new Text struct
    pub fn new() -> Text {
        Text {
            content: String::from(""),
            font_size: 0.03,
            font_matrix: Matrix::new(1.0, 0.0, 0.0, 1.0, 0.0, 0.0),
            font_slant: FontSlant::Normal,
            font_weight: FontWeight::Normal,
            font_family: String::from("Serif"),
            color: color::Color::new(),
        }
    }

    /// Create and return a new Text struct
    #[allow(dead_code)] // TODO: Issue #13
    pub fn new_from(content: &str) -> Text {
        Text {
            content: String::from(content),
            font_size: 0.03,
            font_matrix: Matrix::new(1.0, 0.0, 0.0, 1.0, 0.0, 0.0),
            font_slant: FontSlant::Normal,
            font_weight: FontWeight::Normal,
            font_family: String::from("Serif"),
            color: color::Color::new(),
        }
    }

    /// Return the content of the text
    #[allow(dead_code)] // TODO: Issue #13
    pub fn content(&self) -> String {
        self.content.clone()
    }

    /// Return the text font size
    #[allow(dead_code)] // TODO: Issue #13
    pub fn font_size(&self) -> f64 {
        self.font_size
    }

    /// Overwrite the text content
    pub fn set_content(&mut self, content: &str) {
        self.content = String::from(content);
    }

    /// Overwrite the text font size
    pub fn set_font_size(&mut self, size: f64) {
        self.font_size = size;
    }

    pub fn set_font_slant(&mut self, font_slant: FontSlant) {
        self.font_slant = font_slant;
    }

    pub fn set_font_weight(&mut self, font_weight: FontWeight) {
        self.font_weight = font_weight;
    }

    pub fn set_font_family(&mut self) {
        // TODO:
        self.font_family = String::from("Serif");
    }

    /// Set text color
    pub fn set_color_internal(&mut self, color: Srgba) {
        self.color.set_color(color);
    }

    /// Scale the font size and of the text
    pub fn scale_size(&mut self, factor: f64) {
        self.font_size *= factor;
    }

    /// Draw text
    pub fn draw(&self, cr: &Context, fig_rel_height: f64, fig_rel_width: f64, angle: f64,
                left_gap: f64, right_gap: f64, bottom_gap: f64, top_gap: f64, line_width: f64) {
        // NOTE 1: This function assumes that we are using the default cairo Context coordinate
        // convention. That is: with y increasing downwards. In the rest of the program, we assume
        // that y is increasing upwards. Because of this, the shift in cr.rel_move_to() below has a
        // positive sign in the y-argument.
        // NOTE 2: If the text angle is not in {0, pi/2, pi, 3pi/2}, it looks ugly when the figure
        // is not square.

        cr.select_font_face(&self.font_family, self.font_slant, self.font_weight);
        let text_color = self.color.as_srgba();
        cr.set_source_rgba(text_color.red as f64, text_color.green as f64,
                           text_color.blue as f64, text_color.alpha as f64);

        // Adjust font size
        cr.set_font_size(self.font_size);
        let font_matrix = cr.get_font_matrix();

        // In case the text is rotated...
        let norm_factor = angle.cos() + angle.sin();
        let new_xx = font_matrix.xx * (angle.cos() * fig_rel_height + angle.sin() * fig_rel_width);
        let new_yy = font_matrix.yy * (angle.sin() * fig_rel_height + angle.cos() * fig_rel_width);

        cr.set_font_matrix(Matrix::new(new_xx / norm_factor,
                                       1.0 * font_matrix.yx,
                                       1.0 * font_matrix.xy,
                                       new_yy / norm_factor,
                                       1.0 * font_matrix.x0,
                                       1.0 * font_matrix.y0));

        let text_width = cr.text_extents(&self.content).width;
        let text_height = cr.text_extents(&self.content).height;

        // Potentially draw a frame around the label
        let curr_pos = cr.get_current_point();
        cr.set_line_width(line_width);
        cr.rel_move_to(-text_width / 2.0 - left_gap, text_height / 2.0 + bottom_gap);
        cr.rel_line_to(left_gap + text_width + right_gap, 0.0);
        cr.rel_line_to(0.0, -bottom_gap - text_height - top_gap);
        cr.rel_line_to(-right_gap - text_width - left_gap, 0.0);
        cr.close_path();
        cr.stroke();
        //cr.rel_line_to(0.0, 2.0 * gap_size + text_height); // cr.close_path()
        cr.move_to(curr_pos.0, curr_pos.1);

        // Draw the text, and reset the font matrix
        cr.rel_move_to(-text_width / 2.0, text_height / 2.0);
        cr.show_text(&self.content);

        cr.set_font_matrix(font_matrix);
    }
}

