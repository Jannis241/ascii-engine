pub enum Dimension {
    drei_dimensional,
    zwei_dimensional,
}
pub struct Settings {
    pub dimension: Dimension,
    pub refresh_rate: u32,
    pub screen_size: (u32, u32),
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            dimension: Dimension::zwei_dimensional,
            refresh_rate: 60,
            screen_size: (1920, 1080),
        }
    }
}

impl Settings {
    pub fn new(dimension: Dimension, refresh_rate: u32, screen_size: (u32, u32)) -> Self {
        Self {
            dimension,
            refresh_rate,
            screen_size,
        }
    }
}
