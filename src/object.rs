pub trait Drawable {
    fn draw(&self) {}
}

pub enum Color {
    White,
    Black,
}

pub struct Circle {
    center: (u32, u32),
    radius: u32,
    color: Color,
}

impl Circle {
    pub fn new(center: (u32, u32), radius: u32, color: Color) -> Self {
        Circle {
            center,
            radius,
            color,
        }
    }
}

pub struct Square {
    top_left: (u32, u32),
    bottom_right: (u32, u32),
    color: Color,
}

impl Square {
    pub fn new(top_left: (u32, u32), bottom_right: (u32, u32), color: Color) -> Self {
        Square {
            top_left,
            bottom_right,
            color,
        }
    }
}

impl Drawable for Square {}
impl Drawable for Circle {}
