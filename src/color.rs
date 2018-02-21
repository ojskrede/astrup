//! Definitions of things related to colors
//!
//! Astrup uses the crate [palette]() to manage colors behind the scenes. In the future, there
//! should possibly be an even tighter coupling. For now, Astrup prowides the following methods for
//! defining the color of an `object`:
//!
//! #### `fn set_object_color(mut self, color: CustomColor) -> Self`
//! This lets the user select one of a set of predefined default colors.
//!
//! #### `fn set_object_color_rgb(mut self, red: f32, green: f32, blue: f32) -> Self`
//! Gives the color specified by the amount of red, green, and blue, all which take values in
//! `[0.0, 1.0]`. This color is completely opaque (alpha is set to 1.0).
//!
//! #### `fn set_object_color_rgba(mut self, red: f32, green: f32, blue: f32, alpha: f32) -> Self`
//! As `set_object_color_rgb(...)` but where you can adjust the transparency `alpha` which takes
//! values in `[0.0, 1.0]`.
//!
//! #### `fn set_object_color_rgb_u8(mut self, red: u8, green: u8, blue: u8) -> Self`
//! As `set_object_color_rgb(...)` but where color channel intensities are specified with values in
//! `[0, 255]`.
//!
//! #### `fn set_object_color_rgba_u8(mut self, red: u8, green: u8, blue: u8, alpha: u8) -> Self`
//! As `set_object_color_rgb_u8(...)` but where you can adjust the transparency `alpha` which takes
//! values in `[0, 255]`.
//!
//! #### `fn set_object_color_html(mut self, color: HtmlColor) -> Result<Self, Error>`
//! The argument is one of the 140 named HTML colors.
//!

use palette::{Srgba, named};

/// `#176CBE` <div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: #176CBE;"></div>
pub const CUSTOM_BLUE:       (u8, u8, u8, u8) = ( 23, 108, 190, 255);

/// `#E0340B` <div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: #E0340B;"></div>
pub const CUSTOM_RED:        (u8, u8, u8, u8) = (224,  52,  11, 255);

/// `#22AE33` <div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: #22AE33;"></div>
pub const CUSTOM_GREEN:      (u8, u8, u8, u8) = ( 34, 174,  51, 255);

/// `#FFC80E` <div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: #FFC80E;"></div>
pub const CUSTOM_YELLOW:     (u8, u8, u8, u8) = (255, 200,  14, 255);

/// `#883CB1` <div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: #883CB1;"></div>
pub const CUSTOM_VIOLET:     (u8, u8, u8, u8) = (136,  60, 177, 255);

/// `#00C6C6` <div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: #00C6C6;"></div>
pub const CUSTOM_CYAN:       (u8, u8, u8, u8) = (  0, 198, 198, 255);

/// `#FF6607` <div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: #FF6607;"></div>
pub const CUSTOM_ORANGE:     (u8, u8, u8, u8) = (255, 102,   7, 255);

/// `#C23AA0` <div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: #C23AA0;"></div>
pub const CUSTOM_MAGENTA:    (u8, u8, u8, u8) = (194,  58, 160, 255);

/// `#000000` <div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: #000000;"></div>
pub const CUSTOM_BLACK:      (u8, u8, u8, u8) = (  0,   0,   0, 255);

/// `#7F7F7F` <div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: #7F7F7F;"></div>
pub const CUSTOM_GRAY:       (u8, u8, u8, u8) = (127, 127, 127, 255);

/// `#FFFFFF` <div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: #FFFFFF;"></div>
pub const CUSTOM_WHITE:      (u8, u8, u8, u8) = (255, 255, 255, 255);

/// `#FFFFFF` <div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: #FFFFFF;"></div>
pub const FIGURE_BACKGROUND: (u8, u8, u8, u8) = (255, 255, 255, 0);

/// `#FAFAFA` <div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: #FAFAFA;"></div>
pub const PLOT_BACKGROUND:   (u8, u8, u8, u8) = (250, 250, 250, 255);

/// `#F0F0F0` <div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: #F0F0F0;"></div>
pub const CANVAS_BACKGROUND: (u8, u8, u8, u8) = (240, 240, 240, 255);

/// `#323232` <div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: #323232;"></div>
pub const FIGURE_BORDER:     (u8, u8, u8, u8) = ( 50,  50,  50, 255);

/// `#323232` <div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: #323232;"></div>
pub const PLOT_BORDER:       (u8, u8, u8, u8) = ( 50,  50,  50, 255);

/// `#323232` <div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: #323232;"></div>
pub const CANVAS_BORDER:     (u8, u8, u8, u8) = ( 50,  50,  50, 255);

/// `#FFFFFF` <div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: #FFFFFF;"></div>
pub const GRID_LINE:         (u8, u8, u8, u8) = (255, 255, 255, 255);

/// `#1E1E1E` <div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: #1E1E1E;"></div>
pub const AXIS_LINE:         (u8, u8, u8, u8) = ( 30,  30,  30, 255);

/// `#1E1E1E` <div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: #1E1E1E;"></div>
pub const TICK:              (u8, u8, u8, u8) = ( 30,  30,  30, 255);

/// `#1E1E1E` <div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: #1E1E1E;"></div>
pub const FIGURE_TITLE:      (u8, u8, u8, u8) = ( 30,  30,  30, 255);

/// `#1E1E1E` <div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: #1E1E1E;"></div>
pub const PLOT_TITLE:        (u8, u8, u8, u8) = ( 30,  30,  30, 255);

/// `#1E1E1E` <div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: #1E1E1E;"></div>
pub const AXIS_LABEL:        (u8, u8, u8, u8) = ( 30,  30,  30, 255);

/// `#1E1E1E` <div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: #1E1E1E;"></div>
pub const TICK_LABEL:        (u8, u8, u8, u8) = ( 30,  30,  30, 255);


/// Custom colors used in this library
pub enum CustomColor {
    Blue,
    Red,
    Green,
    Yellow,
    Violet,
    Cyan,
    Orange,
    Magenta,
    Black,
    Gray,
    White,
    FigureBackground,
    PlotBackground,
    CanvasBackground,
    FigureBorder,
    PlotBorder,
    CanvasBorder,
    GridLine,
    AxisLine,
    Tick,
    FigureTitle,
    PlotTitle,
    AxisLabel,
    TickLabel,
}

impl CustomColor {
    pub fn as_srgba(&self) -> Srgba {
        match *self {
            CustomColor::Blue             => Srgba::<f32>::from_pixel(&CUSTOM_BLUE).into(),
            CustomColor::Red              => Srgba::<f32>::from_pixel(&CUSTOM_RED).into(),
            CustomColor::Green            => Srgba::<f32>::from_pixel(&CUSTOM_GREEN).into(),
            CustomColor::Yellow           => Srgba::<f32>::from_pixel(&CUSTOM_YELLOW).into(),
            CustomColor::Violet           => Srgba::<f32>::from_pixel(&CUSTOM_VIOLET).into(),
            CustomColor::Cyan             => Srgba::<f32>::from_pixel(&CUSTOM_CYAN).into(),
            CustomColor::Orange           => Srgba::<f32>::from_pixel(&CUSTOM_ORANGE).into(),
            CustomColor::Magenta          => Srgba::<f32>::from_pixel(&CUSTOM_MAGENTA).into(),
            CustomColor::Black            => Srgba::<f32>::from_pixel(&CUSTOM_BLACK).into(),
            CustomColor::Gray             => Srgba::<f32>::from_pixel(&CUSTOM_GRAY).into(),
            CustomColor::White            => Srgba::<f32>::from_pixel(&CUSTOM_WHITE).into(),
            CustomColor::FigureBackground => Srgba::<f32>::from_pixel(&FIGURE_BACKGROUND).into(),
            CustomColor::PlotBackground   => Srgba::<f32>::from_pixel(&PLOT_BACKGROUND).into(),
            CustomColor::CanvasBackground => Srgba::<f32>::from_pixel(&CANVAS_BACKGROUND).into(),
            CustomColor::FigureBorder     => Srgba::<f32>::from_pixel(&FIGURE_BORDER).into(),
            CustomColor::PlotBorder       => Srgba::<f32>::from_pixel(&PLOT_BORDER).into(),
            CustomColor::CanvasBorder     => Srgba::<f32>::from_pixel(&CANVAS_BORDER).into(),
            CustomColor::GridLine         => Srgba::<f32>::from_pixel(&GRID_LINE).into(),
            CustomColor::AxisLine         => Srgba::<f32>::from_pixel(&AXIS_LINE).into(),
            CustomColor::Tick             => Srgba::<f32>::from_pixel(&TICK).into(),
            CustomColor::FigureTitle      => Srgba::<f32>::from_pixel(&FIGURE_TITLE).into(),
            CustomColor::PlotTitle        => Srgba::<f32>::from_pixel(&PLOT_TITLE).into(),
            CustomColor::AxisLabel        => Srgba::<f32>::from_pixel(&AXIS_LABEL).into(),
            CustomColor::TickLabel        => Srgba::<f32>::from_pixel(&TICK_LABEL).into(),
        }
    }
}

/// Named HTML colors supported by most modern browsers. For more info, see e.g.
/// [here](https://htmlcolorcodes.com/color-names/). See also the
/// [palette documentation(https://docs.rs/palette/0.3.0/palette/named/index.html) for the
/// definition.
pub enum HtmlColor {
    Aliceblue,
    Antiquewhite,
    Aqua,
    Aquamarine,
    Azure,
    Beige,
    Bisque,
    Black,
    Blanchedalmond,
    Blue,
    Blueviolet,
    Brown,
    Burlywood,
    Cadetblue,
    Chartreuse,
    Chocolate,
    Coral,
    Cornflowerblue,
    Cornsilk,
    Crimson,
    Cyan,
    Darkblue,
    Darkcyan,
    Darkgoldenrod,
    Darkgray,
    Darkgreen,
    Darkgrey,
    Darkkhaki,
    Darkmagenta,
    Darkolivegreen,
    Darkorange,
    Darkorchid,
    Darkred,
    Darksalmon,
    Darkseagreen,
    Darkslateblue,
    Darkslategray,
    Darkslategrey,
    Darkturquoise,
    Darkviolet,
    Deeppink,
    Deepskyblue,
    Dimgray,
    Dimgrey,
    Dodgerblue,
    Firebrick,
    Floralwhite,
    Forestgreen,
    Fuchsia,
    Gainsboro,
    Ghostwhite,
    Gold,
    Goldenrod,
    Gray,
    Grey,
    Green,
    Greenyellow,
    Honeydew,
    Hotpink,
    Indianred,
    Indigo,
    Ivory,
    Khaki,
    Lavender,
    Lavenderblush,
    Lawngreen,
    Lemonchiffon,
    Lightblue,
    Lightcoral,
    Lightcyan,
    Lightgoldenrodyellow,
    Lightgray,
    Lightgreen,
    Lightgrey,
    Lightpink,
    Lightsalmon,
    Lightseagreen,
    Lightskyblue,
    Lightslategray,
    Lightslategrey,
    Lightsteelblue,
    Lightyellow,
    Lime,
    Limegreen,
    Linen,
    Magenta,
    Maroon,
    Mediumaquamarine,
    Mediumblue,
    Mediumorchid,
    Mediumpurple,
    Mediumseagreen,
    Mediumslateblue,
    Mediumspringgreen,
    Mediumturquoise,
    Mediumvioletred,
    Midnightblue,
    Mintcream,
    Mistyrose,
    Moccasin,
    Navajowhite,
    Navy,
    Oldlace,
    Olive,
    Olivedrab,
    Orange,
    Orangered,
    Orchid,
    Palegoldenrod,
    Palegreen,
    Paleturquoise,
    Palevioletred,
    Papayawhip,
    Peachpuff,
    Peru,
    Pink,
    Plum,
    Powderblue,
    Purple,
    Red,
    Rosybrown,
    Royalblue,
    Saddlebrown,
    Salmon,
    Sandybrown,
    Seagreen,
    Seashell,
    Sienna,
    Silver,
    Skyblue,
    Slateblue,
    Slategray,
    Slategrey,
    Snow,
    Springgreen,
    Steelblue,
    Tan,
    Teal,
    Thistle,
    Tomato,
    Turquoise,
    Violet,
    Wheat,
    White,
    Whitesmoke,
    Yellow,
    Yellowgreen,
}

impl HtmlColor {
    pub fn as_srgba(&self) -> Srgba {
        match *self {
            HtmlColor::Aliceblue            => Srgba::<f32>::from_pixel(&named::ALICEBLUE).into(),
            HtmlColor::Antiquewhite         => Srgba::<f32>::from_pixel(&named::ANTIQUEWHITE).into(),
            HtmlColor::Aqua                 => Srgba::<f32>::from_pixel(&named::AQUA).into(),
            HtmlColor::Aquamarine           => Srgba::<f32>::from_pixel(&named::AQUAMARINE).into(),
            HtmlColor::Azure                => Srgba::<f32>::from_pixel(&named::AZURE).into(),
            HtmlColor::Beige                => Srgba::<f32>::from_pixel(&named::BEIGE).into(),
            HtmlColor::Bisque               => Srgba::<f32>::from_pixel(&named::BISQUE).into(),
            HtmlColor::Black                => Srgba::<f32>::from_pixel(&named::BLACK).into(),
            HtmlColor::Blanchedalmond       => Srgba::<f32>::from_pixel(&named::BLANCHEDALMOND).into(),
            HtmlColor::Blue                 => Srgba::<f32>::from_pixel(&named::BLUE).into(),
            HtmlColor::Blueviolet           => Srgba::<f32>::from_pixel(&named::BLUEVIOLET).into(),
            HtmlColor::Brown                => Srgba::<f32>::from_pixel(&named::BROWN).into(),
            HtmlColor::Burlywood            => Srgba::<f32>::from_pixel(&named::BURLYWOOD).into(),
            HtmlColor::Cadetblue            => Srgba::<f32>::from_pixel(&named::CADETBLUE).into(),
            HtmlColor::Chartreuse           => Srgba::<f32>::from_pixel(&named::CHARTREUSE).into(),
            HtmlColor::Chocolate            => Srgba::<f32>::from_pixel(&named::CHOCOLATE).into(),
            HtmlColor::Coral                => Srgba::<f32>::from_pixel(&named::CORAL).into(),
            HtmlColor::Cornflowerblue       => Srgba::<f32>::from_pixel(&named::CORNFLOWERBLUE).into(),
            HtmlColor::Cornsilk             => Srgba::<f32>::from_pixel(&named::CORNSILK).into(),
            HtmlColor::Crimson              => Srgba::<f32>::from_pixel(&named::CRIMSON).into(),
            HtmlColor::Cyan                 => Srgba::<f32>::from_pixel(&named::CYAN).into(),
            HtmlColor::Darkblue             => Srgba::<f32>::from_pixel(&named::DARKBLUE).into(),
            HtmlColor::Darkcyan             => Srgba::<f32>::from_pixel(&named::DARKCYAN).into(),
            HtmlColor::Darkgoldenrod        => Srgba::<f32>::from_pixel(&named::DARKGOLDENROD).into(),
            HtmlColor::Darkgray             => Srgba::<f32>::from_pixel(&named::DARKGRAY).into(),
            HtmlColor::Darkgreen            => Srgba::<f32>::from_pixel(&named::DARKGREEN).into(),
            HtmlColor::Darkgrey             => Srgba::<f32>::from_pixel(&named::DARKGREY).into(),
            HtmlColor::Darkkhaki            => Srgba::<f32>::from_pixel(&named::DARKKHAKI).into(),
            HtmlColor::Darkmagenta          => Srgba::<f32>::from_pixel(&named::DARKMAGENTA).into(),
            HtmlColor::Darkolivegreen       => Srgba::<f32>::from_pixel(&named::DARKOLIVEGREEN).into(),
            HtmlColor::Darkorange           => Srgba::<f32>::from_pixel(&named::DARKORANGE).into(),
            HtmlColor::Darkorchid           => Srgba::<f32>::from_pixel(&named::DARKORCHID).into(),
            HtmlColor::Darkred              => Srgba::<f32>::from_pixel(&named::DARKRED).into(),
            HtmlColor::Darksalmon           => Srgba::<f32>::from_pixel(&named::DARKSALMON).into(),
            HtmlColor::Darkseagreen         => Srgba::<f32>::from_pixel(&named::DARKSEAGREEN).into(),
            HtmlColor::Darkslateblue        => Srgba::<f32>::from_pixel(&named::DARKSLATEBLUE).into(),
            HtmlColor::Darkslategray        => Srgba::<f32>::from_pixel(&named::DARKSLATEGRAY).into(),
            HtmlColor::Darkslategrey        => Srgba::<f32>::from_pixel(&named::DARKSLATEGREY).into(),
            HtmlColor::Darkturquoise        => Srgba::<f32>::from_pixel(&named::DARKTURQUOISE).into(),
            HtmlColor::Darkviolet           => Srgba::<f32>::from_pixel(&named::DARKVIOLET).into(),
            HtmlColor::Deeppink             => Srgba::<f32>::from_pixel(&named::DEEPPINK).into(),
            HtmlColor::Deepskyblue          => Srgba::<f32>::from_pixel(&named::DEEPSKYBLUE).into(),
            HtmlColor::Dimgray              => Srgba::<f32>::from_pixel(&named::DIMGRAY).into(),
            HtmlColor::Dimgrey              => Srgba::<f32>::from_pixel(&named::DIMGREY).into(),
            HtmlColor::Dodgerblue           => Srgba::<f32>::from_pixel(&named::DODGERBLUE).into(),
            HtmlColor::Firebrick            => Srgba::<f32>::from_pixel(&named::FIREBRICK).into(),
            HtmlColor::Floralwhite          => Srgba::<f32>::from_pixel(&named::FLORALWHITE).into(),
            HtmlColor::Forestgreen          => Srgba::<f32>::from_pixel(&named::FORESTGREEN).into(),
            HtmlColor::Fuchsia              => Srgba::<f32>::from_pixel(&named::FUCHSIA).into(),
            HtmlColor::Gainsboro            => Srgba::<f32>::from_pixel(&named::GAINSBORO).into(),
            HtmlColor::Ghostwhite           => Srgba::<f32>::from_pixel(&named::GHOSTWHITE).into(),
            HtmlColor::Gold                 => Srgba::<f32>::from_pixel(&named::GOLD).into(),
            HtmlColor::Goldenrod            => Srgba::<f32>::from_pixel(&named::GOLDENROD).into(),
            HtmlColor::Gray                 => Srgba::<f32>::from_pixel(&named::GRAY).into(),
            HtmlColor::Grey                 => Srgba::<f32>::from_pixel(&named::GREY).into(),
            HtmlColor::Green                => Srgba::<f32>::from_pixel(&named::GREEN).into(),
            HtmlColor::Greenyellow          => Srgba::<f32>::from_pixel(&named::GREENYELLOW).into(),
            HtmlColor::Honeydew             => Srgba::<f32>::from_pixel(&named::HONEYDEW).into(),
            HtmlColor::Hotpink              => Srgba::<f32>::from_pixel(&named::HOTPINK).into(),
            HtmlColor::Indianred            => Srgba::<f32>::from_pixel(&named::INDIANRED).into(),
            HtmlColor::Indigo               => Srgba::<f32>::from_pixel(&named::INDIGO).into(),
            HtmlColor::Ivory                => Srgba::<f32>::from_pixel(&named::IVORY).into(),
            HtmlColor::Khaki                => Srgba::<f32>::from_pixel(&named::KHAKI).into(),
            HtmlColor::Lavender             => Srgba::<f32>::from_pixel(&named::LAVENDER).into(),
            HtmlColor::Lavenderblush        => Srgba::<f32>::from_pixel(&named::LAVENDERBLUSH).into(),
            HtmlColor::Lawngreen            => Srgba::<f32>::from_pixel(&named::LAWNGREEN).into(),
            HtmlColor::Lemonchiffon         => Srgba::<f32>::from_pixel(&named::LEMONCHIFFON).into(),
            HtmlColor::Lightblue            => Srgba::<f32>::from_pixel(&named::LIGHTBLUE).into(),
            HtmlColor::Lightcoral           => Srgba::<f32>::from_pixel(&named::LIGHTCORAL).into(),
            HtmlColor::Lightcyan            => Srgba::<f32>::from_pixel(&named::LIGHTCYAN).into(),
            HtmlColor::Lightgoldenrodyellow => Srgba::<f32>::from_pixel(&named::LIGHTGOLDENRODYELLOW).into(),
            HtmlColor::Lightgray            => Srgba::<f32>::from_pixel(&named::LIGHTGRAY).into(),
            HtmlColor::Lightgreen           => Srgba::<f32>::from_pixel(&named::LIGHTGREEN).into(),
            HtmlColor::Lightgrey            => Srgba::<f32>::from_pixel(&named::LIGHTGREY).into(),
            HtmlColor::Lightpink            => Srgba::<f32>::from_pixel(&named::LIGHTPINK).into(),
            HtmlColor::Lightsalmon          => Srgba::<f32>::from_pixel(&named::LIGHTSALMON).into(),
            HtmlColor::Lightseagreen        => Srgba::<f32>::from_pixel(&named::LIGHTSEAGREEN).into(),
            HtmlColor::Lightskyblue         => Srgba::<f32>::from_pixel(&named::LIGHTSKYBLUE).into(),
            HtmlColor::Lightslategray       => Srgba::<f32>::from_pixel(&named::LIGHTSLATEGRAY).into(),
            HtmlColor::Lightslategrey       => Srgba::<f32>::from_pixel(&named::LIGHTSLATEGREY).into(),
            HtmlColor::Lightsteelblue       => Srgba::<f32>::from_pixel(&named::LIGHTSTEELBLUE).into(),
            HtmlColor::Lightyellow          => Srgba::<f32>::from_pixel(&named::LIGHTYELLOW).into(),
            HtmlColor::Lime                 => Srgba::<f32>::from_pixel(&named::LIME).into(),
            HtmlColor::Limegreen            => Srgba::<f32>::from_pixel(&named::LIMEGREEN).into(),
            HtmlColor::Linen                => Srgba::<f32>::from_pixel(&named::LINEN).into(),
            HtmlColor::Magenta              => Srgba::<f32>::from_pixel(&named::MAGENTA).into(),
            HtmlColor::Maroon               => Srgba::<f32>::from_pixel(&named::MAROON).into(),
            HtmlColor::Mediumaquamarine     => Srgba::<f32>::from_pixel(&named::MEDIUMAQUAMARINE).into(),
            HtmlColor::Mediumblue           => Srgba::<f32>::from_pixel(&named::MEDIUMBLUE).into(),
            HtmlColor::Mediumorchid         => Srgba::<f32>::from_pixel(&named::MEDIUMORCHID).into(),
            HtmlColor::Mediumpurple         => Srgba::<f32>::from_pixel(&named::MEDIUMPURPLE).into(),
            HtmlColor::Mediumseagreen       => Srgba::<f32>::from_pixel(&named::MEDIUMSEAGREEN).into(),
            HtmlColor::Mediumslateblue      => Srgba::<f32>::from_pixel(&named::MEDIUMSLATEBLUE).into(),
            HtmlColor::Mediumspringgreen    => Srgba::<f32>::from_pixel(&named::MEDIUMSPRINGGREEN).into(),
            HtmlColor::Mediumturquoise      => Srgba::<f32>::from_pixel(&named::MEDIUMTURQUOISE).into(),
            HtmlColor::Mediumvioletred      => Srgba::<f32>::from_pixel(&named::MEDIUMVIOLETRED).into(),
            HtmlColor::Midnightblue         => Srgba::<f32>::from_pixel(&named::MIDNIGHTBLUE).into(),
            HtmlColor::Mintcream            => Srgba::<f32>::from_pixel(&named::MINTCREAM).into(),
            HtmlColor::Mistyrose            => Srgba::<f32>::from_pixel(&named::MISTYROSE).into(),
            HtmlColor::Moccasin             => Srgba::<f32>::from_pixel(&named::MOCCASIN).into(),
            HtmlColor::Navajowhite          => Srgba::<f32>::from_pixel(&named::NAVAJOWHITE).into(),
            HtmlColor::Navy                 => Srgba::<f32>::from_pixel(&named::NAVY).into(),
            HtmlColor::Oldlace              => Srgba::<f32>::from_pixel(&named::OLDLACE).into(),
            HtmlColor::Olive                => Srgba::<f32>::from_pixel(&named::OLIVE).into(),
            HtmlColor::Olivedrab            => Srgba::<f32>::from_pixel(&named::OLIVEDRAB).into(),
            HtmlColor::Orange               => Srgba::<f32>::from_pixel(&named::ORANGE).into(),
            HtmlColor::Orangered            => Srgba::<f32>::from_pixel(&named::ORANGERED).into(),
            HtmlColor::Orchid               => Srgba::<f32>::from_pixel(&named::ORCHID).into(),
            HtmlColor::Palegoldenrod        => Srgba::<f32>::from_pixel(&named::PALEGOLDENROD).into(),
            HtmlColor::Palegreen            => Srgba::<f32>::from_pixel(&named::PALEGREEN).into(),
            HtmlColor::Paleturquoise        => Srgba::<f32>::from_pixel(&named::PALETURQUOISE).into(),
            HtmlColor::Palevioletred        => Srgba::<f32>::from_pixel(&named::PALEVIOLETRED).into(),
            HtmlColor::Papayawhip           => Srgba::<f32>::from_pixel(&named::PAPAYAWHIP).into(),
            HtmlColor::Peachpuff            => Srgba::<f32>::from_pixel(&named::PEACHPUFF).into(),
            HtmlColor::Peru                 => Srgba::<f32>::from_pixel(&named::PERU).into(),
            HtmlColor::Pink                 => Srgba::<f32>::from_pixel(&named::PINK).into(),
            HtmlColor::Plum                 => Srgba::<f32>::from_pixel(&named::PLUM).into(),
            HtmlColor::Powderblue           => Srgba::<f32>::from_pixel(&named::POWDERBLUE).into(),
            HtmlColor::Purple               => Srgba::<f32>::from_pixel(&named::PURPLE).into(),
            HtmlColor::Red                  => Srgba::<f32>::from_pixel(&named::RED).into(),
            HtmlColor::Rosybrown            => Srgba::<f32>::from_pixel(&named::ROSYBROWN).into(),
            HtmlColor::Royalblue            => Srgba::<f32>::from_pixel(&named::ROYALBLUE).into(),
            HtmlColor::Saddlebrown          => Srgba::<f32>::from_pixel(&named::SADDLEBROWN).into(),
            HtmlColor::Salmon               => Srgba::<f32>::from_pixel(&named::SALMON).into(),
            HtmlColor::Sandybrown           => Srgba::<f32>::from_pixel(&named::SANDYBROWN).into(),
            HtmlColor::Seagreen             => Srgba::<f32>::from_pixel(&named::SEAGREEN).into(),
            HtmlColor::Seashell             => Srgba::<f32>::from_pixel(&named::SEASHELL).into(),
            HtmlColor::Sienna               => Srgba::<f32>::from_pixel(&named::SIENNA).into(),
            HtmlColor::Silver               => Srgba::<f32>::from_pixel(&named::SILVER).into(),
            HtmlColor::Skyblue              => Srgba::<f32>::from_pixel(&named::SKYBLUE).into(),
            HtmlColor::Slateblue            => Srgba::<f32>::from_pixel(&named::SLATEBLUE).into(),
            HtmlColor::Slategray            => Srgba::<f32>::from_pixel(&named::SLATEGRAY).into(),
            HtmlColor::Slategrey            => Srgba::<f32>::from_pixel(&named::SLATEGREY).into(),
            HtmlColor::Snow                 => Srgba::<f32>::from_pixel(&named::SNOW).into(),
            HtmlColor::Springgreen          => Srgba::<f32>::from_pixel(&named::SPRINGGREEN).into(),
            HtmlColor::Steelblue            => Srgba::<f32>::from_pixel(&named::STEELBLUE).into(),
            HtmlColor::Tan                  => Srgba::<f32>::from_pixel(&named::TAN).into(),
            HtmlColor::Teal                 => Srgba::<f32>::from_pixel(&named::TEAL).into(),
            HtmlColor::Thistle              => Srgba::<f32>::from_pixel(&named::THISTLE).into(),
            HtmlColor::Tomato               => Srgba::<f32>::from_pixel(&named::TOMATO).into(),
            HtmlColor::Turquoise            => Srgba::<f32>::from_pixel(&named::TURQUOISE).into(),
            HtmlColor::Violet               => Srgba::<f32>::from_pixel(&named::VIOLET).into(),
            HtmlColor::Wheat                => Srgba::<f32>::from_pixel(&named::WHEAT).into(),
            HtmlColor::White                => Srgba::<f32>::from_pixel(&named::WHITE).into(),
            HtmlColor::Whitesmoke           => Srgba::<f32>::from_pixel(&named::WHITESMOKE).into(),
            HtmlColor::Yellow               => Srgba::<f32>::from_pixel(&named::YELLOW).into(),
            HtmlColor::Yellowgreen          => Srgba::<f32>::from_pixel(&named::YELLOWGREEN).into(),
        }
    }
}

/// Simple struct used to generate chart custom chart colors
pub(crate) struct ChartColorGenerator {
    pub(self) internal_index: usize,
}

impl ChartColorGenerator {
    pub fn new() -> ChartColorGenerator {
        ChartColorGenerator {
            internal_index: 0,
        }
    }
}

impl Iterator for ChartColorGenerator {
    type Item = Srgba;
    fn next(&mut self) -> Option<Srgba> {
        let colors = vec![CustomColor::Blue.as_srgba(),
                          CustomColor::Red.as_srgba(),
                          CustomColor::Green.as_srgba(),
                          CustomColor::Yellow.as_srgba(),
                          CustomColor::Violet.as_srgba(),
                          CustomColor::Cyan.as_srgba(),
                          CustomColor::Orange.as_srgba(),
                          CustomColor::Magenta.as_srgba()];
        let index = self.internal_index % colors.len();
        self.internal_index += 1;
        Some(colors[index])
    }
}

/// The type used for colors in this library. It is really just a small wrapper over the
/// palette crate.
#[derive(Clone, Debug)]
pub(crate) struct Color {
    color: Srgba,
}

impl Color {
    pub fn new() -> Color {
        Color {
            color: Srgba::new(0.0, 0.0, 0.0, 1.0),
        }
    }

    pub fn new_custom(color: CustomColor) -> Color {
        Color {
            color: color.as_srgba()
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

    #[allow(dead_code)]
    pub fn new_html(color: HtmlColor) -> Color {
        Color {
            color: color.as_srgba()
        }
    }

    pub fn set_color(&mut self, color: Srgba) {
        self.color = color
    }

    pub fn set_color_custom(&mut self, color: CustomColor) {
        self.set_color(color.as_srgba());
    }

    pub fn set_color_rgb(&mut self, red: f32, green: f32, blue: f32) {
        self.color = Srgba::new(red, green, blue, 1.0);
    }

    pub fn set_color_rgb_u8(&mut self, red: u8, green: u8, blue: u8) {
        self.color = Srgba::new_u8(red, green, blue, 255);
    }

    pub fn set_color_rgba(&mut self, red: f32, green: f32, blue: f32, alpha: f32) {
        self.color = Srgba::new(red, green, blue, alpha);
    }

    pub fn set_color_rgba_u8(&mut self, red: u8, green: u8, blue: u8, alpha: u8) {
        self.color = Srgba::new_u8(red, green, blue, alpha);
    }

    pub fn set_color_html(&mut self, color: HtmlColor) {
        self.color = color.as_srgba();
    }

    pub fn as_srgba(&self) -> Srgba {
        self.color
    }
}
