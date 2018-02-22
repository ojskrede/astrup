//! Module that defines the Line struct
//!

use cairo::{Context, LineCap};
use palette::Srgba;
use ndarray::AsArray;

use ::{chart, utils, shape, coord, color};

#[derive(Clone, Debug)]
pub enum LineStyle {
    Plain,
    LeftStair,
    RightStair,
}

#[derive(Clone, Debug)]
pub enum StrokeStyle {
    Continuous,
    Dashed,
    Dotted,
}

#[derive(Clone, Debug)]
struct DashPattern {
    on_length: f64,
    off_length: f64,
    offset: f64,
    cap: LineCap,
}

impl DashPattern {
    fn with_style(stroke_style: &StrokeStyle) -> DashPattern {
        match *stroke_style {
            StrokeStyle::Dashed => {
                DashPattern {
                    on_length: 0.01,
                    off_length: 0.015,
                    offset: 0.0,
                    cap: LineCap::Square,
                }
            },
            StrokeStyle::Dotted => {
                DashPattern {
                    on_length: 0.0,
                    off_length: 0.015,
                    offset: 0.0,
                    cap: LineCap::Round,
                }
            },
            StrokeStyle::Continuous => {
                DashPattern {
                    on_length: 1.0,
                    off_length: 0.0,
                    offset: 0.0,
                    cap: LineCap::Round,
                }
            },
        }
    }

    fn set_on_length(&mut self, val: f64) {
        self.on_length = val;
    }

    fn set_off_length(&mut self, val: f64) {
        self.off_length = val;
    }

    fn set_offset(&mut self, val: f64) {
        self.offset = val;
    }

    fn set_line_cap(&mut self, cap: LineCap) {
        self.cap = cap;
    }

    fn on_length(&self) -> f64 {
        self.on_length
    }

    fn off_length(&self) -> f64 {
        self.off_length
    }

    fn offset(&self) -> f64 {
        self.offset
    }

    fn line_cap(&self) -> LineCap {
        self.cap
    }

    fn scale_size(&mut self, factor: f64) {
        self.on_length *= factor;
        self.off_length *= factor;
    }
}

/// Line chart
///
/// With this chart, one is able to display data using lines. Straight lines are drawn between
/// coordinates, determined by the input data points. You can e.g. alter the line color, the dash
/// pattern, and the stroke style.
#[derive(Clone, Debug)]
pub struct Line {
    data_points: Vec<chart::point::Point>,
    data_frame: shape::Rectangle,
    global_frame: shape::Rectangle,
    color: color::Color,
    is_color_updated: bool,
    line_width: f64,
    line_style: LineStyle,
    stroke_style: StrokeStyle,
    dash_pattern: DashPattern,
}

impl Line {
    /// Create and return a new Line chart
    pub fn new<'a, I: AsArray<'a, f64>>(x_data_coords: I, y_data_coords: I) -> Line {
        let x_view: Vec<_> = x_data_coords.into().iter().map(|v| utils::NonNan::new(*v).unwrap()).collect();
        let y_view: Vec<_> = y_data_coords.into().iter().map(|v| utils::NonNan::new(*v).unwrap()).collect();
        let ref x_data_min = x_view.iter().min().expect("Could not find x min");
        let ref x_data_max = x_view.iter().max().expect("Could not find x max");
        let ref y_data_min = y_view.iter().min().expect("Could not find y min");
        let ref y_data_max = y_view.iter().max().expect("Could not find y max");


        let mut data_points = Vec::<chart::point::Point>::new();
        for (ref x, ref y) in x_view.iter().zip(y_view.iter()) {
            let mut point = chart::point::Point::new(x.val(), y.val());
            point.set_size(0.0);
            data_points.push(point);
        }
        let stroke_style = StrokeStyle::Continuous;
        let dash_pattern = DashPattern::with_style(&stroke_style);

        Line {
            data_points: data_points,
            data_frame: shape::Rectangle::with_boundaries(x_data_min.val(), x_data_max.val(),
                                                          y_data_min.val(), y_data_max.val()),
            global_frame: shape::Rectangle::new(),
            color: color::Color::with_custom(color::CustomColor::Blue),
            is_color_updated: false,
            line_width: 0.0035,
            line_style: LineStyle::Plain,
            stroke_style: stroke_style,
            dash_pattern: dash_pattern,
        }
    }

    /// Set the line color using the default, built in colors
    pub fn set_color(mut self, color: color::CustomColor) -> Self {
        self.color.set_color_custom(color);
        self.is_color_updated = true;
        self
    }

    /// Set the line color
    pub fn set_color_rgb(mut self, red: f32, green: f32, blue: f32) -> Self {
        self.color.set_color_rgb(red, green, blue);
        self.is_color_updated = true;
        self
    }

    /// Set the line color
    pub fn set_color_rgba(mut self, red: f32, green: f32, blue: f32, alpha: f32) -> Self {
        self.color.set_color_rgba(red, green, blue, alpha);
        self.is_color_updated = true;
        self
    }

    /// Set the line color
    pub fn set_color_rgb_u8(mut self, red: u8, green: u8, blue: u8) -> Self {
        self.color.set_color_rgb_u8(red, green, blue);
        self.is_color_updated = true;
        self
    }

    /// Set the line color
    pub fn set_color_rgba_u8(mut self, red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        self.color.set_color_rgba_u8(red, green, blue, alpha);
        self.is_color_updated = true;
        self
    }

    /// Set the line color from name. See the [palette
    /// documentation](https://docs.rs/palette/0.3.0/palette/named/index.html) for more info.
    pub fn set_color_html(mut self, color: color::HtmlColor) -> Self {
        self.color.set_color_html(color);
        self.is_color_updated = true;
        self
    }

    /// Set the line width
    pub fn set_line_width(mut self, val: f64) -> Self {
        self.line_width = val;
        self
    }

    /// Set the style of the line. Plain, left stair, or right stair.
    pub fn set_line_style(mut self, style: LineStyle) -> Self {
        self.line_style = style;
        self
    }

    /// Set the stroke style of the line
    pub fn set_stroke_style(mut self, style: StrokeStyle) -> Self {
        self.stroke_style = style;
        self.dash_pattern = DashPattern::with_style(&self.stroke_style);
        self
    }

    /// Set the length of the ``on duration'' of a dash in a dash line
    pub fn set_dash_on_length(mut self, val: f64) -> Self {
        self.dash_pattern.set_on_length(val);
        self
    }

    /// Set the length of the ``off duration'' of a dash in a dash line
    pub fn set_dash_off_length(mut self, val: f64) -> Self {
        self.dash_pattern.set_off_length(val);
        self
    }

    /// Set the offset of the line dash pattern
    pub fn set_dash_offset(mut self, val: f64) -> Self {
        self.dash_pattern.set_offset(val);
        self
    }

    /// Set the line cap of the line dash pattern
    pub fn set_line_cap(mut self, cap: LineCap) -> Self {
        self.dash_pattern.set_line_cap(cap);
        self
    }
}

impl utils::Drawable for Line {
    fn set_color_internal(&mut self, color: Srgba) {
        self.color.set_color(color);
        self.is_color_updated = true;
    }

    fn is_color_updated(&self) -> bool {
        self.is_color_updated
    }

    fn scale_size(&mut self, factor: f64) {
        self.line_width *= factor;
        self.dash_pattern.scale_size(factor);
    }

    fn fit(&mut self, canvas_global_frame: &shape::Rectangle, canvas_data_frame: &shape::Rectangle) {
        self.global_frame = canvas_global_frame.clone();
        self.data_frame = canvas_data_frame.clone();
        let scale_factor = self.global_frame.diag_len();
        self.scale_size(scale_factor);

        for data_point in self.data_points.iter_mut() {
            data_point.fit(canvas_global_frame, canvas_data_frame);
        }
    }

    fn draw(&self, cr: &Context, fig_rel_height: f64, fig_rel_width: f64) {
        let mut first_point = true;
        cr.set_dash(&[self.dash_pattern.on_length(), self.dash_pattern.off_length()],
                      self.dash_pattern.offset());
        cr.set_line_cap(self.dash_pattern.line_cap());
        let mut prev_coord = coord::Coord::new();
        let line_color = self.color.as_srgba();
        match self.line_style {
            LineStyle::Plain => {
                for data_point in self.data_points.iter() {
                    let canvas_x = utils::map_range(data_point.x_coord(),
                                                    self.data_frame.left(), self.data_frame.right(),
                                                    self.global_frame.left(), self.global_frame.right());
                    let canvas_y = utils::map_range(data_point.y_coord(),
                                                    self.data_frame.bottom(), self.data_frame.top(),
                                                    self.global_frame.bottom(), self.global_frame.top());
                    let mut canvas_point = data_point.clone();
                    canvas_point.set_x_coord(canvas_x);
                    canvas_point.set_y_coord(canvas_y);

                    if !first_point {
                        let curr_coord = canvas_point.coord();
                        let direction = prev_coord.unit_direction_to(&curr_coord);
                        cr.set_source_rgba(line_color.red as f64, line_color.green as f64,
                                           line_color.blue as f64, line_color.alpha as f64);
                        let line_width = self.line_width * (direction.x().abs() * fig_rel_width + direction.y().abs() * fig_rel_height);
                        cr.set_line_width(line_width);
                        cr.line_to(canvas_point.x_coord(), canvas_point.y_coord());
                        cr.stroke();
                    }
                    canvas_point.draw(cr, fig_rel_height, fig_rel_width);
                    cr.move_to(canvas_point.x_coord(), canvas_point.y_coord());
                    first_point = false;
                    prev_coord = canvas_point.coord();
                }
            },
            LineStyle::LeftStair => {
                let mut prev_canvas_x = 0.0;
                for data_point in self.data_points.iter() {
                    let canvas_x = utils::map_range(data_point.x_coord(),
                                                    self.data_frame.left(), self.data_frame.right(),
                                                    self.global_frame.left(), self.global_frame.right());
                    let canvas_y = utils::map_range(data_point.y_coord(),
                                                    self.data_frame.bottom(), self.data_frame.top(),
                                                    self.global_frame.bottom(), self.global_frame.top());
                    let mut canvas_point = data_point.clone();

                    if first_point {
                        canvas_point.set_x_coord(canvas_x);
                        canvas_point.set_y_coord(canvas_y);
                    } else {
                        canvas_point.set_x_coord(prev_canvas_x);
                        canvas_point.set_y_coord(canvas_y);
                    }

                    if first_point {
                        // After the first point, this is the "angle point" in the stair, and has
                        // no original point attached to it, therefore we do not draw it.
                        canvas_point.draw(cr, fig_rel_height, fig_rel_width);
                    } else {
                        cr.set_source_rgba(line_color.red as f64, line_color.green as f64,
                                           line_color.blue as f64, line_color.alpha as f64);
                        cr.set_line_width(self.line_width);
                        cr.line_to(canvas_point.x_coord(), canvas_point.y_coord());
                        cr.stroke();
                    }
                    cr.move_to(canvas_point.x_coord(), canvas_point.y_coord());

                    if !first_point {
                        canvas_point.set_x_coord(canvas_x);

                        cr.set_source_rgba(line_color.red as f64, line_color.green as f64,
                                           line_color.blue as f64, line_color.alpha as f64);
                        cr.set_line_width(self.line_width);
                        cr.line_to(canvas_point.x_coord(), canvas_point.y_coord());
                        cr.stroke();

                        canvas_point.draw(cr, fig_rel_height, fig_rel_width);
                        cr.move_to(canvas_point.x_coord(), canvas_point.y_coord());
                    }

                    prev_canvas_x = canvas_x;

                    first_point = false;
                }
            },
            LineStyle::RightStair => {
                let mut prev_canvas_y = 0.0;
                for data_point in self.data_points.iter() {
                    let canvas_x = utils::map_range(data_point.x_coord(),
                                                    self.data_frame.left(), self.data_frame.right(),
                                                    self.global_frame.left(), self.global_frame.right());
                    let canvas_y = utils::map_range(data_point.y_coord(),
                                                    self.data_frame.bottom(), self.data_frame.top(),
                                                    self.global_frame.bottom(), self.global_frame.top());
                    let mut canvas_point = data_point.clone();

                    if first_point {
                        canvas_point.set_x_coord(canvas_x);
                        canvas_point.set_y_coord(canvas_y);
                    } else {
                        canvas_point.set_x_coord(canvas_x);
                        canvas_point.set_y_coord(prev_canvas_y);
                    }

                    if first_point {
                        // After the first point, this is the "angle point" in the stair, and has
                        // no original point attached to it, therefore we do not draw it.
                        canvas_point.draw(cr, fig_rel_height, fig_rel_width);
                    } else {
                        cr.set_source_rgba(line_color.red as f64, line_color.green as f64,
                                           line_color.blue as f64, line_color.alpha as f64);
                        cr.set_line_width(self.line_width);
                        cr.line_to(canvas_point.x_coord(), canvas_point.y_coord());
                        cr.stroke();
                    }
                    cr.move_to(canvas_point.x_coord(), canvas_point.y_coord());

                    if !first_point {
                        canvas_point.set_y_coord(canvas_y);

                        cr.set_source_rgba(line_color.red as f64, line_color.green as f64,
                                           line_color.blue as f64, line_color.alpha as f64);
                        cr.set_line_width(self.line_width);
                        cr.line_to(canvas_point.x_coord(), canvas_point.y_coord());
                        cr.stroke();

                        canvas_point.draw(cr, fig_rel_height, fig_rel_width);
                        cr.move_to(canvas_point.x_coord(), canvas_point.y_coord());
                    }

                    prev_canvas_y = canvas_y;

                    first_point = false;
                }
            },
        }
        // Reset back to continuous line
        cr.set_dash(&[], 0.0);
    }
}

impl utils::Plottable for Line {
    fn data_frame(&self) -> shape::Rectangle {
        self.data_frame.clone()
    }

    fn data_x_min(&self) -> f64 {
        self.data_frame.left()
    }

    fn data_x_max(&self) -> f64 {
        self.data_frame.right()
    }

    fn data_y_min(&self) -> f64 {
        self.data_frame.bottom()
    }

    fn data_y_max(&self) -> f64 {
        self.data_frame.top()
    }

    fn set_data_frame(&mut self, new_data_frame: shape::Rectangle) {
        self.data_frame = new_data_frame;
    }
}
