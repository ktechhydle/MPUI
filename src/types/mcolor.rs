pub struct MColor {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl MColor {
    pub fn default() -> Self {
        MColor {
            r: 255,
            g: 255,
            b: 255,
            a: 255,
        }
    }

    pub fn new(&self) -> Self {
        MColor::default()
    }

    pub fn set_hex(hex_code: &str) -> Self {
        MColor::from_hex(hex_code)
    }

    pub fn set_rgb(r: u8, g: u8, b: u8) -> Self {
        MColor::from_rgb(r, g, b)
    }

    pub fn set_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        MColor::from_rgba(r, g, b, a)
    }

    pub fn from_hex(hex_code: &str) -> Self {
        let hex_code = hex_code.trim_start_matches('#');

        if hex_code.len() < 6 {
            return MColor::default();
        }

        let r = u8::from_str_radix(&hex_code[0..2], 16).unwrap_or(255);
        let g = u8::from_str_radix(&hex_code[2..4], 16).unwrap_or(255);
        let b = u8::from_str_radix(&hex_code[4..6], 16).unwrap_or(255);
        let a = if hex_code.len() == 8 {
            u8::from_str_radix(&hex_code[6..8], 16).unwrap_or(255)
        } else {
            255
        };

        MColor { r, g, b, a }
    }

    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        MColor {
            r: r,
            g: g,
            b: b,
            a: 255,
        }
    }

    pub fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        MColor {
            r: r,
            g: g,
            b: b,
            a: a,
        }
    }
}
