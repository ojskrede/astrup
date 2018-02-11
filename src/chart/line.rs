//! ## Line
//!
//! Module that defines the Line struct
//!

use cairo::{Context, LineCap};
use palette::Rgba;
use ndarray::AsArray;

use chart::point::Point;
use utils::{self, Frame, Drawable, Plottable, NonNan};

#[derive(Clone, Debug)]
enum LineStyle {
    Plain,
    LeftStair,
    RightStair,
}

#[derive(Clone, Debug)]
enum StrokeStyle {
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
    fn new(stroke_style: &StrokeStyle) -> DashPattern {
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

/// Line struct
///
/// With this chart, one is able to display data using lines. Straight lines are drawn between
/// coordinates, determined by the input data points. You can e.g. alter the line color, the dash
/// pattern, and the stroke style.
///
/// **Note** The input can be any data container that implements the AsArray trait (e.g. a Vec, or
/// ndarray Array), but the contained data must be f64. Ideally, this should be any integer or
/// float, but I have not been able to implement a generic over them also. This is perhaps
/// connected to things like higher kinded types and the like, which I think will come soon.
#[derive(Clone, Debug)]
pub struct Line {
    data_points: Vec<Point>,
    data_frame: Frame,
    global_frame: Frame,
    color: Rgba,
    line_width: f64,
    line_style: LineStyle,
    stroke_style: StrokeStyle,
    dash_pattern: DashPattern,
}

impl Line {
    /// Create and return a new Line chart
    pub fn new<'a, I: AsArray<'a, f64>>(x_data_coords: I, y_data_coords: I) -> Line {
        let x_view: Vec<_> = x_data_coords.into().iter().map(|v| NonNan::new(*v).unwrap()).collect();
        let y_view: Vec<_> = y_data_coords.into().iter().map(|v| NonNan::new(*v).unwrap()).collect();
        let ref x_data_min = x_view.iter().min().expect("Could not find x min");
        let ref x_data_max = x_view.iter().max().expect("Could not find x max");
        let ref y_data_min = y_view.iter().min().expect("Could not find y min");
        let ref y_data_max = y_view.iter().max().expect("Could not find y max");


        let mut data_points = Vec::<Point>::new();
        for (ref x, ref y) in x_view.iter().zip(y_view.iter()) {
            let mut point = Point::new(x.val(), y.val());
            point.set_size(0.0);
            data_points.push(point);
        }
        let stroke_style = StrokeStyle::Continuous;
        let dash_pattern = DashPattern::new(&stroke_style);

        Line {
            data_points: data_points,
            data_frame: Frame::from_sides(x_data_min.val(), x_data_max.val(),
                                          y_data_min.val(), y_data_max.val()),
            global_frame: Frame::new(),
            color: Rgba::new(0.1, 0.2, 0.5, 0.9),
            line_width: 0.005,
            line_style: LineStyle::Plain,
            stroke_style: stroke_style,
            dash_pattern: dash_pattern,
        }
    }

    /// Set the line color
    pub fn set_color(&mut self, color: Rgba) {
        self.color = color;
    }

    /// Set the line width
    pub fn set_line_width(&mut self, val: f64) {
        self.line_width = val;
    }

    /// Set the style of the line. Plain, left stair, or right stair.
    pub fn set_line_style(&mut self, style: &str) {
        match style {
            "plain" => self.line_style = LineStyle::Plain,
            "left_stair" => self.line_style = LineStyle::LeftStair,
            "right_stair" => self.line_style = LineStyle::RightStair,
            _ => self.line_style = LineStyle::Plain,
        }
    }

    /// Set the stroke style of the line
    pub fn set_stroke_style(&mut self, style: &str) {
        match style {
            "dashed" => self.stroke_style = StrokeStyle::Dashed,
            "dotted" => self.stroke_style = StrokeStyle::Dotted,
            _ => self.stroke_style = StrokeStyle::Continuous,
        }
        self.dash_pattern = DashPattern::new(&self.stroke_style);
    }

    /// Set the length of the ``on duration'' of a dash in a dash line
    pub fn set_dash_on_length(&mut self, val: f64) {
        self.dash_pattern.set_on_length(val);
    }

    /// Set the length of the ``off duration'' of a dash in a dash line
    pub fn set_dash_off_length(&mut self, val: f64) {
        self.dash_pattern.set_off_length(val);
    }

    /// Set the offset of the line dash pattern
    pub fn set_dash_offset(&mut self, val: f64) {
        self.dash_pattern.set_offset(val);
    }

    /// Set the line cap of the line dash pattern
    pub fn set_line_cap(&mut self, cap: LineCap) {
        self.dash_pattern.set_line_cap(cap);
    }
}

impl Drawable for Line {
    fn scale_size(&mut self, factor: f64) {
        self.line_width *= factor;
        self.dash_pattern.scale_size(factor);
    }

    fn fit(&mut self, canvas_global_frame: &Frame, canvas_data_frame: &Frame) {
        self.global_frame = canvas_global_frame.clone();
        self.data_frame = canvas_data_frame.clone();
        let scale_factor = self.global_frame.diag_len() / 2f64.sqrt();
        self.scale_size(scale_factor);

        for data_point in self.data_points.iter_mut() {
            data_point.fit(canvas_global_frame, canvas_data_frame);
        }
    }

    fn draw(&self, cr: &Context) {
        let mut first_point = true;
        cr.set_dash(&[self.dash_pattern.on_length(), self.dash_pattern.off_length()],
                      self.dash_pattern.offset());
        cr.set_line_cap(self.dash_pattern.line_cap());
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
                        cr.set_source_rgba(self.color.red as f64, self.color.green as f64,
                                           self.color.blue as f64, self.color.alpha as f64);
                        cr.set_line_width(self.line_width);
                        cr.line_to(canvas_point.x_coord(), canvas_point.y_coord());
                        cr.stroke();
                    }
                    canvas_point.draw(cr);
                    cr.move_to(canvas_point.x_coord(), canvas_point.y_coord());
                    first_point = false;
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
                        canvas_point.draw(cr);
                    } else {
                        cr.set_source_rgba(self.color.red as f64, self.color.green as f64,
                                           self.color.blue as f64, self.color.alpha as f64);
                        cr.set_line_width(self.line_width);
                        cr.line_to(canvas_point.x_coord(), canvas_point.y_coord());
                        cr.stroke();
                    }
                    cr.move_to(canvas_point.x_coord(), canvas_point.y_coord());

                    if !first_point {
                        canvas_point.set_x_coord(canvas_x);

                        cr.set_source_rgba(self.color.red as f64, self.color.green as f64,
                                           self.color.blue as f64, self.color.alpha as f64);
                        cr.set_line_width(self.line_width);
                        cr.line_to(canvas_point.x_coord(), canvas_point.y_coord());
                        cr.stroke();

                        canvas_point.draw(cr);
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
                        canvas_point.draw(cr);
                    } else {
                        cr.set_source_rgba(self.color.red as f64, self.color.green as f64,
                                           self.color.blue as f64, self.color.alpha as f64);
                        cr.set_line_width(self.line_width);
                        cr.line_to(canvas_point.x_coord(), canvas_point.y_coord());
                        cr.stroke();
                    }
                    cr.move_to(canvas_point.x_coord(), canvas_point.y_coord());

                    if !first_point {
                        canvas_point.set_y_coord(canvas_y);

                        cr.set_source_rgba(self.color.red as f64, self.color.green as f64,
                                           self.color.blue as f64, self.color.alpha as f64);
                        cr.set_line_width(self.line_width);
                        cr.line_to(canvas_point.x_coord(), canvas_point.y_coord());
                        cr.stroke();

                        canvas_point.draw(cr);
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

impl Plottable for Line {
    fn data_frame(&self) -> Frame {
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

    fn set_data_frame(&mut self, new_data_frame: Frame) {
        self.data_frame = new_data_frame;
    }
}
