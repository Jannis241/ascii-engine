use crate::object::Drawable;

pub struct Renderer;

impl Renderer {
    pub fn new() -> Self {
        Renderer
    }

    pub fn render_frame<T>(&self, objects: Vec<Box<T>>)
    where
        T: Drawable + 'static + ?Sized,
    {
    }
}
