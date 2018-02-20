//! Definitions of things related to colors
//!

use palette::{Srgba, named};

/// Simple struct used to hold and generate default chart colors
pub struct ChartColors {
    pub(self) blue: Srgba,
    pub(self) red: Srgba,
    pub(self) green: Srgba,
    pub(self) yellow: Srgba,
    pub(self) violet: Srgba,
    pub(self) cyan: Srgba,
    pub(self) orange: Srgba,
    pub(self) magenta: Srgba,
    pub(self) internal_index: usize,
}

impl ChartColors {
    pub fn new() -> ChartColors {
        ChartColors {
            blue: Srgba::new_u8(23, 108, 190, 255),
            red: Srgba::new_u8(224, 52, 11, 255),
            green: Srgba::new_u8(34, 174, 51, 255),
            yellow: Srgba::new_u8(255, 200, 14, 255),
            violet: Srgba::new_u8(136, 60, 177, 255),
            cyan: Srgba::new_u8(0, 198, 198, 255),
            orange: Srgba::new_u8(255, 102, 7, 255),
            magenta: Srgba::new_u8(194, 58, 160, 255),
            internal_index: 0,
        }
    }
}

impl Iterator for ChartColors {
    type Item = Srgba;
    fn next(&mut self) -> Option<Srgba> {
        let colors = vec![self.blue,
                          self.red,
                          self.green,
                          self.yellow,
                          self.violet,
                          self.cyan,
                          self.orange,
                          self.magenta];
        let index = self.internal_index % colors.len();
        self.internal_index += 1;
        Some(colors[index])
    }
}

/// Simple struct used to hold default colors used in this library.
pub struct DefaultColors {
    chart_colors: ChartColors,
    black: Srgba,
    gray: Srgba,
    white: Srgba,
    figure_background: Srgba,
    plot_background: Srgba,
    canvas_background: Srgba,
    figure_border: Srgba,
    plot_border: Srgba,
    canvas_border: Srgba,
    grid_line: Srgba,
    axis_line: Srgba,
    tick: Srgba,
    figure_title: Srgba,
    plot_title: Srgba,
    axis_label: Srgba,
    tick_label: Srgba,
}

impl DefaultColors {
    pub fn new() -> DefaultColors {
        DefaultColors {
            chart_colors: ChartColors::new(),
            black: Srgba::new_u8(0, 0, 0, 255),
            gray: Srgba::new_u8(127, 127, 127, 255),
            white: Srgba::new_u8(255, 255, 255, 255),
            figure_background: Srgba::new_u8(1, 1, 1, 0),
            plot_background: Srgba::new_u8(250, 250, 250, 255),
            canvas_background: Srgba::new_u8(240, 240, 240, 255),
            figure_border: Srgba::new_u8(50, 50, 50, 255),
            plot_border: Srgba::new_u8(50, 50, 50, 255),
            canvas_border: Srgba::new_u8(50, 50, 50, 255),
            grid_line: Srgba::new_u8(255, 255, 255, 127),
            axis_line: Srgba::new_u8(30, 30, 30, 255),
            tick: Srgba::new_u8(30, 30, 30, 255),
            figure_title: Srgba::new_u8(30, 30, 30, 255),
            plot_title: Srgba::new_u8(30, 30, 30, 255),
            axis_label: Srgba::new_u8(30, 30, 30, 255),
            tick_label: Srgba::new_u8(30, 30, 30, 255),
        }
    }

    pub fn color(&self, name: &str) -> Srgba {
        match name {
            "blue" | "b" => self.chart_colors.blue,
            "red" | "r" => self.chart_colors.red,
            "green" | "g" => self.chart_colors.green,
            "yellow" | "y" => self.chart_colors.yellow,
            "violet" | "purple" | "v" | "p" => self.chart_colors.violet,
            "cyan" | "c" => self.chart_colors.cyan,
            "orange" | "o" => self.chart_colors.orange,
            "magenta" | "m" => self.chart_colors.magenta,
         // -------------------------------------------------------------------
            "black" | "k" => self.black,
            "gray" | "grey" => self.gray,
            "white" | "w" => self.white,
         // -------------------------------------------------------------------
            "figure_background" => self.figure_background,
            "plot_background" => self.plot_background,
            "canvas_background" => self.canvas_background,
            "figure_border" => self.figure_border,
            "plot_border" => self.plot_border,
            "canvas_border" => self.canvas_border,
            "grid_line" => self.grid_line,
            "axis_line" => self.axis_line,
            "tick" => self.tick,
            "figure_title" => self.figure_title,
            "plot_title" => self.plot_title,
            "axis_label" => self.axis_label,
            "tick_label" => self.tick_label,
         // -------------------------------------------------------------------
            _ => {
                println!("Warning: unknown color selection: {}", name);
                println!("Default black is selected");
                self.black
            }
        }
    }
}

/// ## Color
///
/// The type used for colors in this library. It is really just a small wrapper over the
/// palette crate.
#[derive(Clone, Debug)]
pub struct Color {
    color: Srgba,
}

impl Color {
    pub fn new() -> Color {
        Color {
            color: Srgba::new(0.0, 0.0, 0.0, 1.0),
        }
    }

    pub fn new_default(color_name: &str) -> Color {
        let default_color = DefaultColors::new();
        Color {
            color: default_color.color(color_name),
        }
    }

    pub fn new_rgb(red: f32, green: f32, blue: f32) -> Color {
        Color {
            color: Srgba::new(red, green, blue, 1.0),
        }
    }

    pub fn new_rgb_u8(red: u8, green: u8, blue: u8) -> Color {
        Color {
            color: Srgba::new_u8(red, green, blue, 255),
        }
    }

    pub fn new_rgba(red: f32, green: f32, blue: f32, alpha: f32) -> Color {
        Color {
            color: Srgba::new(red, green, blue, alpha),
        }
    }

    pub fn new_rgba_u8(red: u8, green: u8, blue: u8, alpha: u8) -> Color {
        Color {
            color: Srgba::new_u8(red, green, blue, alpha),
        }
    }

    pub fn new_str(color_name: &str) -> Color {
        let color_srgb = match named::from_str(color_name) {
            Some(val) => val,
            None => {
                println!("The following color name is not supported: {}", color_name);
                println!("Black is selected");
                (0, 0, 0)
            }
        };
        Color {
            color: Srgba::from_pixel(&color_srgb).into(),
        }
    }

    pub fn set_color(&mut self, color: Srgba) {
        self.color = color
    }

    pub fn set_color_default(&mut self, color_name: &str) {
        self.set_color(DefaultColors::new().color(color_name));
    }

    pub fn set_color_rgb(&mut self, red: f32, green: f32, blue: f32) {
        self.color = Srgba::new(red, green, blue, 1.0);
    }

    pub fn set_color_rgb_u8(&mut self, red: u8, green: u8, blue: u8) {
        self.color = Srgba::new_u8(red, green, blue, 255);
    }

    pub fn set_color_rgba(&mut self, red: f32, green: f32, blue: f32, alpha: f32) {
        /* TODO: is this needed?
        let red = red.max(0.0);
        let red = red.min(1.0);
        let green = green.max(0.0);
        let green = green.min(1.0);
        let blue = blue.max(0.0);
        let blue = blue.min(1.0);
        */
        self.color = Srgba::new(red, green, blue, alpha);
    }

    pub fn set_color_rgba_u8(&mut self, red: u8, green: u8, blue: u8, alpha: u8) {
        self.color = Srgba::new_u8(red, green, blue, alpha);
    }

    pub fn set_color_str(&mut self, color_name: &str) {
        let color_srgb = match named::from_str(color_name) {
            Some(val) => val,
            None => {
                println!("The following color name is not supported: {}", color_name);
                println!("Black is selected");
                (0, 0, 0)
            }
        };
        self.color = Srgba::from_pixel(&color_srgb).into();
    }

    pub fn as_srgba(&self) -> Srgba {
        self.color
    }
}
