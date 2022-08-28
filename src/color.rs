use std::ops;

#[derive(Debug, Clone, Copy, Default)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32
}

#[derive(Debug, Clone)]
pub struct HexParseError;

impl From<std::num::ParseIntError> for HexParseError{
    fn from(_: std::num::ParseIntError) -> Self { HexParseError }
}

impl Color {
    pub fn new(a: f32) -> Color {
        Color{r: a, g: a, b: a}
    }

    pub fn from_hex(hex: &str) -> Result<Color, HexParseError> {
        if hex.starts_with("#") {
            let r = hex.get(1..3).ok_or(HexParseError)?;
            let g = hex.get(3..5).ok_or(HexParseError)?;
            let b = hex.get(5..7).ok_or(HexParseError)?;


            let r =  u8::from_str_radix(r, 16)? as f32 / 255.;
            let g =  u8::from_str_radix(g, 16)? as f32 / 255.;
            let b =  u8::from_str_radix(b, 16)? as f32 / 255.;
            Ok(Color{r, g, b})
        } else {
            Err(HexParseError)
        }
    }

    pub fn pow(&self, exp: f32) -> Color {
        Color {
            r: f32::powf(self.r, exp),
            g: f32::powf(self.g, exp),
            b: f32::powf(self.b, exp),
        }
    }

    pub fn fix(&self) -> Self {
        if f32::is_nan(self.r) || f32::is_nan(self.g) || f32::is_nan(self.b) {
            Color::default()
        } else {
            *self
        }
    }
}

impl_op_ex!(+ |a: &Color, b: &Color| -> Color {
    Color {
        r: a.r + b.r,
        g: a.g + b.g,
        b: a.b + b.b,
    }
});

impl_op_ex!(- |a: &Color, b: &Color| -> Color {
    Color {
        r: a.r - b.r,
        g: a.g - b.g,
        b: a.b - b.b,
    }
});

impl_op_ex!(* |a: &Color, b: &Color| -> Color {
    Color {
        r: a.r * b.r,
        g: a.g * b.g,
        b: a.b * b.b,
    }
});

impl_op_ex_commutative!(* |a: &Color, b: f32| -> Color {
    Color {
        r: a.r * b,
        g: a.g * b,
        b: a.b * b,
    }
});

impl_op_ex_commutative!(+ |a: &Color, b: f32| -> Color {
    Color {
        r: a.r + b,
        g: a.g + b,
        b: a.b + b,
    }
});

impl_op_ex!(- |a: &Color, b: f32| -> Color {
    Color {
        r: a.r - b,
        g: a.g - b,
        b: a.b - b,
    }
});

impl_op_ex!(/ |a: &Color, b: f32| -> Color {
    Color {
        r: a.r / b,
        g: a.g / b,
        b: a.b / b,
    }
});

impl_op_ex!(/ |a: &Color, b: &Color| -> Color {
    Color {
        r: a.r / b.r,
        g: a.g / b.g,
        b: a.b / b.b,
    }
});
