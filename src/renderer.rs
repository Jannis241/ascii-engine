use crate::engine::Object;

pub struct Renderer;

impl Renderer {
    pub fn new() -> Self {
        Renderer
    }

    pub fn render_frame(&self, screen_size: &(u32, u32), objects: &Vec<Object>) {
        for obj in objects {
            let bp = obj.get_blocked_points();
            println!("Blocked points: {bp:?}");
        }
    }
}
