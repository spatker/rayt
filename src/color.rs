use std::ops;

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32
}

impl Color {
    pub fn new(a: f32) -> Color {
        Color{r: a, g: a, b: a}
    }
}

impl Default for Color {
    fn default() -> Self { Color::new(0.0) }
}

macro_rules! hex_color {
    ($hex_str:literal) => {
        if ($hex_str.starts_with("#")) {
            let r =  u8::from_str_radix($hex_str.get(1..3).unwrap(),16).unwrap() as f32 / 255.;
            let g =  u8::from_str_radix($hex_str.get(3..5).unwrap(),16).unwrap() as f32 / 255.;
            let b =  u8::from_str_radix($hex_str.get(5..7).unwrap(),16).unwrap() as f32 / 255.;
            Color{r, g,b}
        } else {
            panic!("Not hex")
        }
    };
}

pub(crate) use hex_color;

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

impl_op_ex!(/ |a: &Color, b: f32| -> Color {
    Color {
        r: a.r / b,
        g: a.g / b,
        b: a.b / b,
    }
});
