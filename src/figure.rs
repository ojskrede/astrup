//! Definition of the Figure struct
//!


use std::fs::File;
use failure::{Error, err_msg};

use cairo::{Context, Format, ImageSurface, Matrix, MatrixTrait, FontSlant, FontWeight};

use ::{plot, shape, color, label};

/// A Figure holds plots, and can be viewed on screen or saved as a png image.
#[derive(Clone)]
pub struct Figure {
    plots: Vec<plot::Plot>,
    title: label::Label,
    window_title: String,
    height: usize,
    width: usize,
    color: color::Color,
    local_frame: shape::Rectangle,
}

impl Figure {
    pub fn new() -> Figure {
        let mut local_frame = shape::Rectangle::new();
        local_frame.display_border(false);
        local_frame.set_color_internal(color::CustomColor::FigureBorder.as_srgba());
        let mut title = label::Label::new();
        title.set_color_internal(color::CustomColor::FigureTitle.as_srgba());
        title.set_centroid(0.5, 0.97);
        title.set_font_size(0.02);
        Figure {
            plots: Vec::<plot::Plot>::new(),
            title: title,
            window_title: String::from("Astrup"),
            height: 800,
            width: 1000,
            color: color::Color::new_custom(color::CustomColor::FigureBackground),
            local_frame: local_frame,
        }
    }

    // ----------------- FIGURE TITLE -------------------------------------- //

    /// Set figure title
    pub fn set_title(mut self, title: &str) -> Self {
        self.title.set_content(title);
        self
    }

    /// Set figure title font size
    pub fn set_title_font_size(mut self, val: f64) -> Self {
        self.title.set_font_size(val);
        self
    }

    /// Set figure title font slant
    pub fn set_title_font_slant(mut self, font_slant: FontSlant) -> Self {
        self.title.set_font_slant(font_slant);
        self
    }

    pub fn set_title_font_weight(mut self, font_weight: FontWeight) -> Self {
        self.title.set_font_weight(font_weight);
        self
    }

    pub fn set_title_font_family(mut self) -> Self {
        // TODO:
        self.title.set_font_family();
        self
    }

    /// Set the angle of the figure title
    pub fn set_title_angle(mut self, val: f64) -> Self {
        self.title.set_angle(val);
        self
    }

    /// Set the location of the figure title, relative to the figure frame
    pub fn set_title_centroid(mut self, x_coord: f64, y_coord: f64) -> Self {
        self.title.set_centroid(x_coord, y_coord);
        self
    }

    /// Set gaps around figure title.
    ///
    /// NOTE: This has currently no visible effect
    pub fn set_title_frame_gaps(mut self, left: f64, right: f64, bottom: f64, top: f64) -> Self {
        self.title.set_frame_gaps(left, right, bottom, top);
        self
    }

    /// Set the title color
    pub fn set_title_color(mut self, color: color::CustomColor) -> Self {
        self.title.set_color_internal(color.as_srgba());
        self
    }

    /// Set the title color
    pub fn set_title_color_rgb(mut self, red: f32, green: f32, blue: f32) -> Self {
        let color = color::Color::new_rgb(red, green, blue);
        self.title.set_color_internal(color.as_srgba());
        self
    }

    /// Set the title color
    pub fn set_title_color_rgba(mut self, red: f32, green: f32, blue: f32, alpha: f32) -> Self {
        let color = color::Color::new_rgba(red, green, blue, alpha);
        self.title.set_color_internal(color.as_srgba());
        self
    }

    /// Set the title color
    pub fn set_title_color_rgb_u8(mut self, red: u8, green: u8, blue: u8) -> Self {
        let color = color::Color::new_rgb_u8(red, green, blue);
        self.title.set_color_internal(color.as_srgba());
        self
    }

    /// Set the title color
    pub fn set_title_color_rgba_u8(mut self, red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        let color = color::Color::new_rgba_u8(red, green, blue, alpha);
        self.title.set_color_internal(color.as_srgba());
        self
    }

    /// Set the title color
    pub fn set_title_color_html(mut self, color: color::HtmlColor) -> Self {
        self.title.set_color_internal(color.as_srgba());
        self
    }

    /// Set window title. This is displayed in the window "header", and not in the figure itself.
    pub fn set_window_title(&mut self, title: &str) {
        self.window_title = String::from(title);
    }

    // ----------------- APPEARANCE ---------------------------------------- //

    pub fn set_height(mut self, val: usize) -> Self {
        self.height = val;
        self
    }

    pub fn set_width(mut self, val: usize) -> Self {
        self.width = val;
        self
    }

    /// Set the figure background color
    pub fn set_color(mut self, color: color::CustomColor) -> Self {
        self.color.set_color_custom(color);
        self
    }

    /// Set the figure background color
    pub fn set_color_rgb(mut self, red: f32, green: f32, blue: f32) -> Self {
        self.color.set_color_rgb(red, green, blue);
        self
    }

    /// Set the figure background color
    pub fn set_color_rgba(mut self, red: f32, green: f32, blue: f32, alpha: f32) -> Self {
        self.color.set_color_rgba(red, green, blue, alpha);
        self
    }

    /// Set the figure background color
    pub fn set_color_rgb_u8(mut self, red: u8, green: u8, blue: u8) -> Self {
        self.color.set_color_rgb_u8(red, green, blue);
        self
    }

    /// Set the figure background color
    pub fn set_color_rgba_u8(mut self, red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        self.color.set_color_rgba_u8(red, green, blue, alpha);
        self
    }

    /// Set the figure background color
    pub fn set_color_html(mut self, color: color::HtmlColor) -> Self {
        self.color.set_color_html(color);
        self
    }

    // ----------------- GETTERS ------------------------------------------- //

    pub fn window_title(&self) -> String {
        self.window_title.clone()
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn width(&self) -> usize {
        self.width
    }

    /// Whether or not to display a border around the figure
    pub fn display_border(mut self, val: bool) -> Self {
        self.local_frame.display_border(val);
        self
    }

    /// Set the figure border color
    pub fn set_border_color(mut self, color: color::CustomColor) -> Self {
        let color = color::Color::new_custom(color);
        self.local_frame.set_color_internal(color.as_srgba());
        self
    }

    /// Set the figure border color
    pub fn set_border_color_rgb(mut self, red: f32, green: f32, blue: f32) -> Self {
        let color = color::Color::new_rgb(red, green, blue);
        self.local_frame.set_color_internal(color.as_srgba());
        self
    }

    /// Set the figure border color
    pub fn set_border_color_rgba(mut self, red: f32, green: f32, blue: f32, alpha: f32) -> Self {
        let color = color::Color::new_rgba(red, green, blue, alpha);
        self.local_frame.set_color_internal(color.as_srgba());
        self
    }

    /// Set the figure border color
    pub fn set_border_color_rgb_u8(mut self, red: u8, green: u8, blue: u8) -> Self {
        let color = color::Color::new_rgb_u8(red, green, blue);
        self.local_frame.set_color_internal(color.as_srgba());
        self
    }

    /// Set the figure border color
    pub fn set_border_color_rgba_u8(mut self, red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        let color = color::Color::new_rgba_u8(red, green, blue, alpha);
        self.local_frame.set_color_internal(color.as_srgba());
        self
    }

    /// Set the figure border color
    pub fn set_border_color_html(mut self, color: color::HtmlColor) -> Self {
        self.local_frame.set_color_internal(color.as_srgba());
        self
    }

    /// Set the line width of the border around the figure
    pub fn set_border_thickness(mut self, val: f64) -> Self {
        self.local_frame.display_border(true);
        self.local_frame.set_border_thickness(val);
        self
    }

    pub fn add(mut self, plot: &plot::Plot) -> Self {
        self.plots.push(plot.clone());
        self
    }

    pub fn save(self, filename: &str) -> Result<(Self), Error> {
        // Since both save() and show() can be called, and since all drawing is happening in both,
        // multiple calls to fit() will be made, and this can mess up things if we call it on self.
        // The simplest solution is to clone self. But one should perhaps make fit() idempotent?.
        let mut fig = self.clone();
        fig.fit()?;
        let surface = match ImageSurface::create(Format::ARgb32, fig.width as i32, fig.height as i32) {
            Ok(val) => val,
            Err(msg) => return Err(err_msg(format!("{:?}", msg))),
        };

        let cr = Context::new(&surface);

        fig.draw(&cr);

        let mut file = File::create(filename)?;
        surface.write_to_png(&mut file)?;

        Ok((self))
    }

    pub(crate) fn fit(&mut self) -> Result<(), Error> {
        // TODO: Issue #13
        self.title.fit(&shape::Rectangle::new());
        for plot in self.plots.iter_mut() {
            let mut new_top = plot.top();
            if self.title.content() != "" {
                new_top = plot.top().min(0.93);
            }
            plot.set_top_mut_ref(new_top);
            plot.fit()?;
        }

        Ok(())
    }

    /// Draw the figure and the subsequent structures
    pub(crate) fn draw(&self, cr: &Context) {

        // # About non-square figures:
        //
        // All structures has been build with the assumption of a (0, 1) х (0, 1) square figure.
        // When we transform the figure to be non-square, all structures we have built will follow
        // the scaling. Below follows a rationale why things are done as they are.
        //
        // ## Desired outcomes
        // Plots, canvases, axes, and marks will be placed as they are expected to, taken the
        // figure scaling into accord. In general, all drawn structures are where they should be.
        // This is controlled by the fit() and scale_size() functions down in the hierarchy.
        //
        // ## Unfortunate outcomes
        // Even if the location is allright, the shape of the things we have drawn will be deformed
        // according to the scaling of the figure. If a figure is 200 х 800, all drawn structures,
        // like texts and lines, will be 4 times as fat as expected. This is ugly.
        //
        // ## One way to fix this
        // We can pass the figure height and figure width down to every object, and counter this
        // effect in the respective draw() functions. Since this only affects the shape, and not
        // the location of the object, it makes sense to do stuff in the draw() functions.
        cr.scale(self.width as f64, self.height as f64);
        let relative_height = self.height() as f64 / self.height().max(self.width()) as f64;
        let relative_width = self.width() as f64 / self.height().max(self.width()) as f64;

        let color_srgb = self.color.as_srgba();
        cr.set_source_rgba(color_srgb.red as f64, color_srgb.green as f64,
                           color_srgb.blue as f64, color_srgb.alpha as f64);
        cr.paint();

        // By default, the origin is in the top left corner, x is increasing to the right, and y is
        // increasing downwards. This transforms the origin to the bottom left, and increasing y
        // upwards.
        let flip_matrix = Matrix::new(1.0, 0.0, 0.0, -1.0, 0.0, 1.0);
        cr.transform(flip_matrix);

        self.title.draw(cr, relative_height, relative_width);

        // Frame border
        self.local_frame.draw(cr, relative_height, relative_width);

        for plot in self.plots.iter() {
            plot.draw(&cr, relative_height, relative_width);
        }
    }
}
