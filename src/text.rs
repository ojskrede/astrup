//! Definition of the Text struct
//!

use cairo::{Context, Matrix, MatrixTrait};
use cairo::enums::{FontSlant, FontWeight};

/// A structure for text elements like labels and titles
#[derive(Clone, Debug)]
pub struct Text {
    content: String,
    font_size: f64,
    angle: f64,
    hor_offset: f64,
    ver_offset: f64,
    font_matrix: Matrix,
}

impl Text {
    /// Create and return a new Text struct
    pub fn new(content: &str) -> Text {
        Text {
            content: String::from(content),
            font_size: 0.03,
            angle: 0.0,
            hor_offset: 0.0,
            ver_offset: 0.0,
            font_matrix: Matrix::new(1.0, 0.0, 0.0, 1.0, 0.0, 0.0),
        }
    }

    /// Return the content of the text
    pub fn content(&self) -> String {
        self.content.clone()
    }

    /// Return the text font size
    pub fn font_size(&self) -> f64 {
        self.font_size
    }

    /// Return the angle of the text
    pub fn angle(&self) -> f64 {
        self.angle
    }

    /// Return the horisontal offset of the text
    pub fn hor_offset(&self) -> f64 {
        self.hor_offset
    }

    /// Return the vertical offset of the text
    pub fn ver_offset(&self) -> f64 {
        self.ver_offset
    }

    /// Overwrite the text content
    pub fn set_content(&mut self, content: &str) {
        self.content = String::from(content);
    }

    /// Overwrite the text font size
    pub fn set_font_size(&mut self, size: f64) {
        self.font_size = size;
    }

    /// Overwrite the text angle
    pub fn set_angle(&mut self, val: f64) {
        self.angle = val;
    }

    /// Owerwrite the text offset
    pub fn set_offset(&mut self, hor: f64, ver: f64) {
        self.hor_offset = hor;
        self.ver_offset = ver;
    }

    /// Scale the vertical and horisontal text offset
    // TODO: Separate vertical and horisontal scaling?
    pub fn scale_offset(&mut self, factor: f64) {
        self.hor_offset *= factor;
        self.ver_offset *= factor;
    }

    /// Scale the font size and offsets of the text
    pub fn scale_size(&mut self, factor: f64) {
        self.font_size *= factor;
        self.scale_offset(factor);
    }

    /// Draw text
    pub fn draw(&self, cr: &Context, fig_rel_height: f64, fig_rel_width: f64) {

        cr.select_font_face("Serif", FontSlant::Normal, FontWeight::Normal);
        cr.set_font_size(self.font_size);
        let curr_font_matrix = cr.get_font_matrix();
        cr.set_font_matrix(Matrix::new(fig_rel_height * curr_font_matrix.xx,
                                       1.0 * curr_font_matrix.yx,
                                       1.0 * curr_font_matrix.xy,
                                       fig_rel_width * curr_font_matrix.yy,
                                       1.0 * curr_font_matrix.x0,
                                       1.0 * curr_font_matrix.y0));

        cr.transform(Matrix::new(1.0, 0.0, 0.0, -1.0, 0.0, 0.0));
        cr.rotate(self.angle);
        cr.show_text(&self.content);
        cr.rotate(-self.angle);
        cr.transform(Matrix::new(1.0, 0.0, 0.0, -1.0, 0.0, 0.0));
    }
}
