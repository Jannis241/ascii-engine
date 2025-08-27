pub enum Color {
    White,
    Black,
    Red,
    Blue,
    Yellow,
}

pub struct Object {
    blocked_points: Vec<(u32, u32)>,
}

impl Object {
    pub fn custom(blocked_points: Vec<(u32, u32)>, color: Color) -> Object {
        Object { blocked_points }
    }

    pub fn circle(center: (u32, u32), radius: u32, color: Color) -> Self {
        let blocked_points = vec![];
        Object { blocked_points }
    }
    pub fn get_blocked_points(&self) -> &Vec<(u32, u32)> {
        &self.blocked_points
    }
}
