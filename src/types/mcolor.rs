pub struct MColor {
    r: f64,
    g: f64,
    b: f64,
    a: f64,
}

impl MColor {
    pub fn from_hex(hex_code: &str) -> Self {
        MColor {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 0.0,
        }
    }

    pub fn from_rgb(r: f64, g: f64, b: f64) -> Self {
        MColor {
            r: r,
            g: g,
            b: b,
            a: 1.0,
        }
    }

    pub fn from_rgba(r: f64, g: f64, b: f64, a: f64) -> Self {
        MColor {
            r: r,
            g: g,
            b: b,
            a: a,
        }
    }
}
