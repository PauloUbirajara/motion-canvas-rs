use vello::peniko::Color;

pub struct Palette;

impl Palette {
    pub const BLACK: Color = Color::rgb8(0x14, 0x14, 0x14);
    pub const BLUE: Color = Color::rgb8(0x68, 0xab, 0xdf);
    pub const DARK_GRAY: Color = Color::rgb8(0x24, 0x24, 0x24);
    pub const GRAY: Color = Color::rgb8(0x80, 0x80, 0x80);
    pub const GREEN: Color = Color::rgb8(0x87, 0xc0, 0x95);
    pub const LIGHT_GRAY: Color = Color::rgb8(0xe5, 0xe5, 0xe5);
    pub const ORANGE: Color = Color::rgb8(0xf2, 0xa6, 0x5a);
    pub const PURPLE: Color = Color::rgb8(0xd1, 0x8e, 0xe2);
    pub const RED: Color = Color::rgb8(0xe1, 0x32, 0x38);
    pub const WHITE: Color = Color::rgb8(0xff, 0xff, 0xff);
    pub const YELLOW: Color = Color::rgb8(0xe6, 0xa7, 0x00);
}
