//! Definitions of things related to coloring
//!

use failure::{Error, err_msg};
use palette::{Rgba, pixel, named};

/// ## DefaultColors
///
/// Simple struct used to hold and generate default colors used in this library.
pub struct DefaultColors {
    blue: Rgba,
    red: Rgba,
    green: Rgba,
    yellow: Rgba,
    violet: Rgba,
    cyan: Rgba,
    orange: Rgba,
    magenta: Rgba,
    black: Rgba,
    gray: Rgba,
    white: Rgba,
    internal_index: usize,
}

impl DefaultColors {
    pub fn new() -> DefaultColors {
        DefaultColors {
            blue: Rgba::new_u8(23, 108, 190, 255),
            red: Rgba::new_u8(224, 52, 11, 255),
            green: Rgba::new_u8(34, 174, 51, 255),
            yellow: Rgba::new_u8(255, 200, 14, 255),
            violet: Rgba::new_u8(136, 60, 177, 255),
            cyan: Rgba::new_u8(0, 198, 198, 255),
            orange: Rgba::new_u8(255, 102, 7, 255),
            magenta: Rgba::new_u8(194, 58, 160, 255),
            black: Rgba::new_u8(0, 0, 0, 255),
            gray: Rgba::new_u8(127, 127, 127, 255),
            white: Rgba::new_u8(255, 255, 255, 255),
            internal_index: 0,
        }
    }

    pub fn color(&self, name: &str) -> Rgba {
        match name {
            "blue" | "b" => self.blue,
            "red" | "r" => self.red,
            "green" | "g" => self.green,
            "yellow" | "y" => self.yellow,
            "violet" | "purple" | "v" | "p" => self.violet,
            "cyan" | "c" => self.cyan,
            "orange" | "o" => self.orange,
            "magenta" | "m" => self.magenta,
            "black" | "k" => self.black,
            "gray" | "grey" => self.gray,
            "white" | "w" => self.white,
            _ => {
                println!("Warning: unknown color selection: {}", name);
                println!("Default blue is selected");
                self.blue
            }
        }
    }
}

impl Iterator for DefaultColors {
    type Item = Rgba;
    fn next(&mut self) -> Option<Rgba> {
        let colors = vec![self.blue,
                          self.red,
                          self.green,
                          self.yellow,
                          self.violet,
                          self.cyan,
                          self.orange,
                          self.magenta,
                          self.black,
                          self.gray,
                          self.white];
        let index = self.internal_index % colors.len();
        self.internal_index += 1;
        Some(colors[index])
    }
}

/// ## Color
///
/// The type used for colors in this library. It is really just a small wrapper over the
/// palette crate.
#[derive(Clone, Debug)]
pub struct Color {
    color: Rgba,
}

impl Color {
    pub fn new() -> Color {
        Color {
            color: Rgba::new(0.0, 0.0, 0.0, 1.0),
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
            color: Rgba::new(red, green, blue, 1.0),
        }
    }

    pub fn new_rgb_u8(red: u8, green: u8, blue: u8) -> Color {
        Color {
            color: Rgba::new_u8(red, green, blue, 255),
        }
    }

    pub fn new_rgba(red: f32, green: f32, blue: f32, alpha: f32) -> Color {
        Color {
            color: Rgba::new(red, green, blue, alpha),
        }
    }

    pub fn new_rgba_u8(red: u8, green: u8, blue: u8, alpha: u8) -> Color {
        Color {
            color: Rgba::new_u8(red, green, blue, alpha),
        }
    }

    pub fn new_str(&mut self, color_name: &str) -> Result<Color, Error> {
        let color_srgb = named::from_str(color_name).ok_or(err_msg("Unknown color name"))?;
        Ok(Color {
            color: pixel::Srgb::from_pixel(&color_srgb).into(),
        })
    }

    pub fn set_color(&mut self, color: Rgba) {
        self.color = color
    }

    pub fn set_color_default(&mut self, color_name: &str) {
        self.set_color(DefaultColors::new().color(color_name));
    }

    pub fn set_color_rgb(&mut self, red: f32, green: f32, blue: f32) {
        self.color = Rgba::new(red, green, blue, 1.0);
    }

    pub fn set_color_rgb_u8(&mut self, red: u8, green: u8, blue: u8) {
        self.color = Rgba::new_u8(red, green, blue, 255);
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
        self.color = Rgba::new(red, green, blue, alpha);
    }

    pub fn set_color_rgba_u8(&mut self, red: u8, green: u8, blue: u8, alpha: u8) {
        self.color = Rgba::new_u8(red, green, blue, alpha);
    }

    pub fn set_color_str(&mut self, color_name: &str) -> Result<(), Error> {
        let color_srgb = named::from_str(color_name).ok_or(err_msg("Unknown color name"))?;
        self.color = pixel::Srgb::from_pixel(&color_srgb).into();
        Ok(())
    }

    pub fn as_rgba(&self) -> Rgba {
        self.color
    }

    pub fn as_srgb(&self) -> pixel::Srgb {
        pixel::Srgb::from(self.color)
    }
}
