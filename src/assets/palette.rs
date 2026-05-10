use peniko::Color;

/// A collection of curated colors for consistent animation design.
///
/// `Palette` provides a set of pre-defined colors that look good together
/// and can be used as defaults for various element properties.
pub struct Palette;

impl Palette {
    /// A deep black for backgrounds or shadows.
    pub const BLACK: Color = Color::rgb8(0x14, 0x14, 0x14);
    /// A soft, modern blue.
    pub const BLUE: Color = Color::rgb8(0x68, 0xab, 0xdf);
    /// A dark gray for secondary backgrounds.
    pub const DARK_GRAY: Color = Color::rgb8(0x24, 0x24, 0x24);
    /// Standard gray.
    pub const GRAY: Color = Color::rgb8(0x80, 0x80, 0x80);
    /// A vibrant green.
    pub const GREEN: Color = Color::rgb8(0x87, 0xc0, 0x95);
    /// A light gray for borders or text.
    pub const LIGHT_GRAY: Color = Color::rgb8(0xe5, 0xe5, 0xe5);
    /// A warm orange.
    pub const ORANGE: Color = Color::rgb8(0xf2, 0xa6, 0x5a);
    /// A soft purple.
    pub const PURPLE: Color = Color::rgb8(0xd1, 0x8e, 0xe2);
    /// A bright red.
    pub const RED: Color = Color::rgb8(0xe1, 0x32, 0x38);
    /// Pure white.
    pub const WHITE: Color = Color::rgb8(0xff, 0xff, 0xff);
    /// A vibrant yellow.
    pub const YELLOW: Color = Color::rgb8(0xe6, 0xa7, 0x00);
}
