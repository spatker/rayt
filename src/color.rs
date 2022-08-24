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
