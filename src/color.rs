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
    #[allow(unknown_lints)]
    #[allow(match_same_arms)]
    pub fn as_srgba(&self) -> Srgba {
        match *self {
            CustomColor::Blue             => Srgba::<f32>::from_pixel(&CUSTOM_BLUE),
            CustomColor::Red              => Srgba::<f32>::from_pixel(&CUSTOM_RED),
            CustomColor::Green            => Srgba::<f32>::from_pixel(&CUSTOM_GREEN),
            CustomColor::Yellow           => Srgba::<f32>::from_pixel(&CUSTOM_YELLOW),
            CustomColor::Violet           => Srgba::<f32>::from_pixel(&CUSTOM_VIOLET),
            CustomColor::Cyan             => Srgba::<f32>::from_pixel(&CUSTOM_CYAN),
            CustomColor::Orange           => Srgba::<f32>::from_pixel(&CUSTOM_ORANGE),
            CustomColor::Magenta          => Srgba::<f32>::from_pixel(&CUSTOM_MAGENTA),
            CustomColor::Black            => Srgba::<f32>::from_pixel(&CUSTOM_BLACK),
            CustomColor::Gray             => Srgba::<f32>::from_pixel(&CUSTOM_GRAY),
            CustomColor::White            => Srgba::<f32>::from_pixel(&CUSTOM_WHITE),
            CustomColor::FigureBackground => Srgba::<f32>::from_pixel(&FIGURE_BACKGROUND),
            CustomColor::PlotBackground   => Srgba::<f32>::from_pixel(&PLOT_BACKGROUND),
            CustomColor::CanvasBackground => Srgba::<f32>::from_pixel(&CANVAS_BACKGROUND),
            CustomColor::FigureBorder     => Srgba::<f32>::from_pixel(&FIGURE_BORDER),
            CustomColor::PlotBorder       => Srgba::<f32>::from_pixel(&PLOT_BORDER),
            CustomColor::CanvasBorder     => Srgba::<f32>::from_pixel(&CANVAS_BORDER),
            CustomColor::GridLine         => Srgba::<f32>::from_pixel(&GRID_LINE),
            CustomColor::AxisLine         => Srgba::<f32>::from_pixel(&AXIS_LINE),
            CustomColor::Tick             => Srgba::<f32>::from_pixel(&TICK),
            CustomColor::FigureTitle      => Srgba::<f32>::from_pixel(&FIGURE_TITLE),
            CustomColor::PlotTitle        => Srgba::<f32>::from_pixel(&PLOT_TITLE),
            CustomColor::AxisLabel        => Srgba::<f32>::from_pixel(&AXIS_LABEL),
            CustomColor::TickLabel        => Srgba::<f32>::from_pixel(&TICK_LABEL),
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
    #[allow(unknown_lints)]
    #[allow(match_same_arms)]
    pub fn as_srgba(&self) -> Srgba {
        match *self {
            HtmlColor::Aliceblue            => Srgba::<f32>::from_pixel(&named::ALICEBLUE),
            HtmlColor::Antiquewhite         => Srgba::<f32>::from_pixel(&named::ANTIQUEWHITE),
            HtmlColor::Aqua                 => Srgba::<f32>::from_pixel(&named::AQUA),
            HtmlColor::Aquamarine           => Srgba::<f32>::from_pixel(&named::AQUAMARINE),
            HtmlColor::Azure                => Srgba::<f32>::from_pixel(&named::AZURE),
            HtmlColor::Beige                => Srgba::<f32>::from_pixel(&named::BEIGE),
            HtmlColor::Bisque               => Srgba::<f32>::from_pixel(&named::BISQUE),
            HtmlColor::Black                => Srgba::<f32>::from_pixel(&named::BLACK),
            HtmlColor::Blanchedalmond       => Srgba::<f32>::from_pixel(&named::BLANCHEDALMOND),
            HtmlColor::Blue                 => Srgba::<f32>::from_pixel(&named::BLUE),
            HtmlColor::Blueviolet           => Srgba::<f32>::from_pixel(&named::BLUEVIOLET),
            HtmlColor::Brown                => Srgba::<f32>::from_pixel(&named::BROWN),
            HtmlColor::Burlywood            => Srgba::<f32>::from_pixel(&named::BURLYWOOD),
            HtmlColor::Cadetblue            => Srgba::<f32>::from_pixel(&named::CADETBLUE),
            HtmlColor::Chartreuse           => Srgba::<f32>::from_pixel(&named::CHARTREUSE),
            HtmlColor::Chocolate            => Srgba::<f32>::from_pixel(&named::CHOCOLATE),
            HtmlColor::Coral                => Srgba::<f32>::from_pixel(&named::CORAL),
            HtmlColor::Cornflowerblue       => Srgba::<f32>::from_pixel(&named::CORNFLOWERBLUE),
            HtmlColor::Cornsilk             => Srgba::<f32>::from_pixel(&named::CORNSILK),
            HtmlColor::Crimson              => Srgba::<f32>::from_pixel(&named::CRIMSON),
            HtmlColor::Cyan                 => Srgba::<f32>::from_pixel(&named::CYAN),
            HtmlColor::Darkblue             => Srgba::<f32>::from_pixel(&named::DARKBLUE),
            HtmlColor::Darkcyan             => Srgba::<f32>::from_pixel(&named::DARKCYAN),
            HtmlColor::Darkgoldenrod        => Srgba::<f32>::from_pixel(&named::DARKGOLDENROD),
            HtmlColor::Darkgray             => Srgba::<f32>::from_pixel(&named::DARKGRAY),
            HtmlColor::Darkgreen            => Srgba::<f32>::from_pixel(&named::DARKGREEN),
            HtmlColor::Darkgrey             => Srgba::<f32>::from_pixel(&named::DARKGREY),
            HtmlColor::Darkkhaki            => Srgba::<f32>::from_pixel(&named::DARKKHAKI),
            HtmlColor::Darkmagenta          => Srgba::<f32>::from_pixel(&named::DARKMAGENTA),
            HtmlColor::Darkolivegreen       => Srgba::<f32>::from_pixel(&named::DARKOLIVEGREEN),
            HtmlColor::Darkorange           => Srgba::<f32>::from_pixel(&named::DARKORANGE),
            HtmlColor::Darkorchid           => Srgba::<f32>::from_pixel(&named::DARKORCHID),
            HtmlColor::Darkred              => Srgba::<f32>::from_pixel(&named::DARKRED),
            HtmlColor::Darksalmon           => Srgba::<f32>::from_pixel(&named::DARKSALMON),
            HtmlColor::Darkseagreen         => Srgba::<f32>::from_pixel(&named::DARKSEAGREEN),
            HtmlColor::Darkslateblue        => Srgba::<f32>::from_pixel(&named::DARKSLATEBLUE),
            HtmlColor::Darkslategray        => Srgba::<f32>::from_pixel(&named::DARKSLATEGRAY),
            HtmlColor::Darkslategrey        => Srgba::<f32>::from_pixel(&named::DARKSLATEGREY),
            HtmlColor::Darkturquoise        => Srgba::<f32>::from_pixel(&named::DARKTURQUOISE),
            HtmlColor::Darkviolet           => Srgba::<f32>::from_pixel(&named::DARKVIOLET),
            HtmlColor::Deeppink             => Srgba::<f32>::from_pixel(&named::DEEPPINK),
            HtmlColor::Deepskyblue          => Srgba::<f32>::from_pixel(&named::DEEPSKYBLUE),
            HtmlColor::Dimgray              => Srgba::<f32>::from_pixel(&named::DIMGRAY),
            HtmlColor::Dimgrey              => Srgba::<f32>::from_pixel(&named::DIMGREY),
            HtmlColor::Dodgerblue           => Srgba::<f32>::from_pixel(&named::DODGERBLUE),
            HtmlColor::Firebrick            => Srgba::<f32>::from_pixel(&named::FIREBRICK),
            HtmlColor::Floralwhite          => Srgba::<f32>::from_pixel(&named::FLORALWHITE),
            HtmlColor::Forestgreen          => Srgba::<f32>::from_pixel(&named::FORESTGREEN),
            HtmlColor::Fuchsia              => Srgba::<f32>::from_pixel(&named::FUCHSIA),
            HtmlColor::Gainsboro            => Srgba::<f32>::from_pixel(&named::GAINSBORO),
            HtmlColor::Ghostwhite           => Srgba::<f32>::from_pixel(&named::GHOSTWHITE),
            HtmlColor::Gold                 => Srgba::<f32>::from_pixel(&named::GOLD),
            HtmlColor::Goldenrod            => Srgba::<f32>::from_pixel(&named::GOLDENROD),
            HtmlColor::Gray                 => Srgba::<f32>::from_pixel(&named::GRAY),
            HtmlColor::Grey                 => Srgba::<f32>::from_pixel(&named::GREY),
            HtmlColor::Green                => Srgba::<f32>::from_pixel(&named::GREEN),
            HtmlColor::Greenyellow          => Srgba::<f32>::from_pixel(&named::GREENYELLOW),
            HtmlColor::Honeydew             => Srgba::<f32>::from_pixel(&named::HONEYDEW),
            HtmlColor::Hotpink              => Srgba::<f32>::from_pixel(&named::HOTPINK),
            HtmlColor::Indianred            => Srgba::<f32>::from_pixel(&named::INDIANRED),
            HtmlColor::Indigo               => Srgba::<f32>::from_pixel(&named::INDIGO),
            HtmlColor::Ivory                => Srgba::<f32>::from_pixel(&named::IVORY),
            HtmlColor::Khaki                => Srgba::<f32>::from_pixel(&named::KHAKI),
            HtmlColor::Lavender             => Srgba::<f32>::from_pixel(&named::LAVENDER),
            HtmlColor::Lavenderblush        => Srgba::<f32>::from_pixel(&named::LAVENDERBLUSH),
            HtmlColor::Lawngreen            => Srgba::<f32>::from_pixel(&named::LAWNGREEN),
            HtmlColor::Lemonchiffon         => Srgba::<f32>::from_pixel(&named::LEMONCHIFFON),
            HtmlColor::Lightblue            => Srgba::<f32>::from_pixel(&named::LIGHTBLUE),
            HtmlColor::Lightcoral           => Srgba::<f32>::from_pixel(&named::LIGHTCORAL),
            HtmlColor::Lightcyan            => Srgba::<f32>::from_pixel(&named::LIGHTCYAN),
            HtmlColor::Lightgoldenrodyellow => Srgba::<f32>::from_pixel(&named::LIGHTGOLDENRODYELLOW),
            HtmlColor::Lightgray            => Srgba::<f32>::from_pixel(&named::LIGHTGRAY),
            HtmlColor::Lightgreen           => Srgba::<f32>::from_pixel(&named::LIGHTGREEN),
            HtmlColor::Lightgrey            => Srgba::<f32>::from_pixel(&named::LIGHTGREY),
            HtmlColor::Lightpink            => Srgba::<f32>::from_pixel(&named::LIGHTPINK),
            HtmlColor::Lightsalmon          => Srgba::<f32>::from_pixel(&named::LIGHTSALMON),
            HtmlColor::Lightseagreen        => Srgba::<f32>::from_pixel(&named::LIGHTSEAGREEN),
            HtmlColor::Lightskyblue         => Srgba::<f32>::from_pixel(&named::LIGHTSKYBLUE),
            HtmlColor::Lightslategray       => Srgba::<f32>::from_pixel(&named::LIGHTSLATEGRAY),
            HtmlColor::Lightslategrey       => Srgba::<f32>::from_pixel(&named::LIGHTSLATEGREY),
            HtmlColor::Lightsteelblue       => Srgba::<f32>::from_pixel(&named::LIGHTSTEELBLUE),
            HtmlColor::Lightyellow          => Srgba::<f32>::from_pixel(&named::LIGHTYELLOW),
            HtmlColor::Lime                 => Srgba::<f32>::from_pixel(&named::LIME),
            HtmlColor::Limegreen            => Srgba::<f32>::from_pixel(&named::LIMEGREEN),
            HtmlColor::Linen                => Srgba::<f32>::from_pixel(&named::LINEN),
            HtmlColor::Magenta              => Srgba::<f32>::from_pixel(&named::MAGENTA),
            HtmlColor::Maroon               => Srgba::<f32>::from_pixel(&named::MAROON),
            HtmlColor::Mediumaquamarine     => Srgba::<f32>::from_pixel(&named::MEDIUMAQUAMARINE),
            HtmlColor::Mediumblue           => Srgba::<f32>::from_pixel(&named::MEDIUMBLUE),
            HtmlColor::Mediumorchid         => Srgba::<f32>::from_pixel(&named::MEDIUMORCHID),
            HtmlColor::Mediumpurple         => Srgba::<f32>::from_pixel(&named::MEDIUMPURPLE),
            HtmlColor::Mediumseagreen       => Srgba::<f32>::from_pixel(&named::MEDIUMSEAGREEN),
            HtmlColor::Mediumslateblue      => Srgba::<f32>::from_pixel(&named::MEDIUMSLATEBLUE),
            HtmlColor::Mediumspringgreen    => Srgba::<f32>::from_pixel(&named::MEDIUMSPRINGGREEN),
            HtmlColor::Mediumturquoise      => Srgba::<f32>::from_pixel(&named::MEDIUMTURQUOISE),
            HtmlColor::Mediumvioletred      => Srgba::<f32>::from_pixel(&named::MEDIUMVIOLETRED),
            HtmlColor::Midnightblue         => Srgba::<f32>::from_pixel(&named::MIDNIGHTBLUE),
            HtmlColor::Mintcream            => Srgba::<f32>::from_pixel(&named::MINTCREAM),
            HtmlColor::Mistyrose            => Srgba::<f32>::from_pixel(&named::MISTYROSE),
            HtmlColor::Moccasin             => Srgba::<f32>::from_pixel(&named::MOCCASIN),
            HtmlColor::Navajowhite          => Srgba::<f32>::from_pixel(&named::NAVAJOWHITE),
            HtmlColor::Navy                 => Srgba::<f32>::from_pixel(&named::NAVY),
            HtmlColor::Oldlace              => Srgba::<f32>::from_pixel(&named::OLDLACE),
            HtmlColor::Olive                => Srgba::<f32>::from_pixel(&named::OLIVE),
            HtmlColor::Olivedrab            => Srgba::<f32>::from_pixel(&named::OLIVEDRAB),
            HtmlColor::Orange               => Srgba::<f32>::from_pixel(&named::ORANGE),
            HtmlColor::Orangered            => Srgba::<f32>::from_pixel(&named::ORANGERED),
            HtmlColor::Orchid               => Srgba::<f32>::from_pixel(&named::ORCHID),
            HtmlColor::Palegoldenrod        => Srgba::<f32>::from_pixel(&named::PALEGOLDENROD),
            HtmlColor::Palegreen            => Srgba::<f32>::from_pixel(&named::PALEGREEN),
            HtmlColor::Paleturquoise        => Srgba::<f32>::from_pixel(&named::PALETURQUOISE),
            HtmlColor::Palevioletred        => Srgba::<f32>::from_pixel(&named::PALEVIOLETRED),
            HtmlColor::Papayawhip           => Srgba::<f32>::from_pixel(&named::PAPAYAWHIP),
            HtmlColor::Peachpuff            => Srgba::<f32>::from_pixel(&named::PEACHPUFF),
            HtmlColor::Peru                 => Srgba::<f32>::from_pixel(&named::PERU),
            HtmlColor::Pink                 => Srgba::<f32>::from_pixel(&named::PINK),
            HtmlColor::Plum                 => Srgba::<f32>::from_pixel(&named::PLUM),
            HtmlColor::Powderblue           => Srgba::<f32>::from_pixel(&named::POWDERBLUE),
            HtmlColor::Purple               => Srgba::<f32>::from_pixel(&named::PURPLE),
            HtmlColor::Red                  => Srgba::<f32>::from_pixel(&named::RED),
            HtmlColor::Rosybrown            => Srgba::<f32>::from_pixel(&named::ROSYBROWN),
            HtmlColor::Royalblue            => Srgba::<f32>::from_pixel(&named::ROYALBLUE),
            HtmlColor::Saddlebrown          => Srgba::<f32>::from_pixel(&named::SADDLEBROWN),
            HtmlColor::Salmon               => Srgba::<f32>::from_pixel(&named::SALMON),
            HtmlColor::Sandybrown           => Srgba::<f32>::from_pixel(&named::SANDYBROWN),
            HtmlColor::Seagreen             => Srgba::<f32>::from_pixel(&named::SEAGREEN),
            HtmlColor::Seashell             => Srgba::<f32>::from_pixel(&named::SEASHELL),
            HtmlColor::Sienna               => Srgba::<f32>::from_pixel(&named::SIENNA),
            HtmlColor::Silver               => Srgba::<f32>::from_pixel(&named::SILVER),
            HtmlColor::Skyblue              => Srgba::<f32>::from_pixel(&named::SKYBLUE),
            HtmlColor::Slateblue            => Srgba::<f32>::from_pixel(&named::SLATEBLUE),
            HtmlColor::Slategray            => Srgba::<f32>::from_pixel(&named::SLATEGRAY),
            HtmlColor::Slategrey            => Srgba::<f32>::from_pixel(&named::SLATEGREY),
            HtmlColor::Snow                 => Srgba::<f32>::from_pixel(&named::SNOW),
            HtmlColor::Springgreen          => Srgba::<f32>::from_pixel(&named::SPRINGGREEN),
            HtmlColor::Steelblue            => Srgba::<f32>::from_pixel(&named::STEELBLUE),
            HtmlColor::Tan                  => Srgba::<f32>::from_pixel(&named::TAN),
            HtmlColor::Teal                 => Srgba::<f32>::from_pixel(&named::TEAL),
            HtmlColor::Thistle              => Srgba::<f32>::from_pixel(&named::THISTLE),
            HtmlColor::Tomato               => Srgba::<f32>::from_pixel(&named::TOMATO),
            HtmlColor::Turquoise            => Srgba::<f32>::from_pixel(&named::TURQUOISE),
            HtmlColor::Violet               => Srgba::<f32>::from_pixel(&named::VIOLET),
            HtmlColor::Wheat                => Srgba::<f32>::from_pixel(&named::WHEAT),
            HtmlColor::White                => Srgba::<f32>::from_pixel(&named::WHITE),
            HtmlColor::Whitesmoke           => Srgba::<f32>::from_pixel(&named::WHITESMOKE),
            HtmlColor::Yellow               => Srgba::<f32>::from_pixel(&named::YELLOW),
            HtmlColor::Yellowgreen          => Srgba::<f32>::from_pixel(&named::YELLOWGREEN),
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

    pub fn with_custom(color: &CustomColor) -> Color {
        Color {
            color: color.as_srgba()
        }
    }

    pub fn with_rgb(red: f32, green: f32, blue: f32) -> Color {
        Color {
            color: Srgba::new(red, green, blue, 1.0),
        }
    }

    pub fn with_rgb_u8(red: u8, green: u8, blue: u8) -> Color {
        Color {
            color: Srgba::new_u8(red, green, blue, 255),
        }
    }

    pub fn with_rgba(red: f32, green: f32, blue: f32, alpha: f32) -> Color {
        Color {
            color: Srgba::new(red, green, blue, alpha),
        }
    }

    pub fn with_rgba_u8(red: u8, green: u8, blue: u8, alpha: u8) -> Color {
        Color {
            color: Srgba::new_u8(red, green, blue, alpha),
        }
    }

    #[allow(dead_code)]
    pub fn with_html(color: &HtmlColor) -> Color {
        Color {
            color: color.as_srgba()
        }
    }

    pub fn set_color(&mut self, color: Srgba) {
        self.color = color
    }

    pub fn set_color_custom(&mut self, color: &CustomColor) {
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

    pub fn set_color_html(&mut self, color: &HtmlColor) {
        self.color = color.as_srgba();
    }

    pub fn as_srgba(&self) -> Srgba {
        self.color
    }
}
