use std::rc::Rc;

use crate::{object::Drawable, renderer::Renderer, Circle, Settings};

pub trait App {
    fn update(&mut self, engine: &mut Engine);
}

pub struct Engine {
    running: bool,
    objects: Vec<Box<dyn Drawable>>,
    settings: Settings,
}

impl Engine {
    pub fn get_screenheight(&self) -> u32 {
        self.settings.screen_size.1
    }

    pub fn get_screenwidth(&self) -> u32 {
        self.settings.screen_size.0
    }

    pub fn draw_line(&self) {}
    pub fn draw_circle(&self) {}
    pub fn draw_square(&self) {}
    pub fn draw_triangle(&self) {}
    pub fn draw_object<T>(&mut self, object: T)
    where
        T: Drawable + 'static + ?Sized,
    {
        self.objects.push(Box::new(object));
    }

    pub fn draw_multiple_objects(&self, objects: &Vec<impl Drawable>) {
        for object in objects {
            object.draw();
        }
    }
    fn create(settings: Settings) -> Self {
        Engine {
            running: true,
            objects: vec![],
            settings,
        }
    }

    pub fn start(settings: Settings, app_struct: &mut impl App) {
        let mut engine = Rc::new(Engine::create(settings));
        let renderer = Renderer::new();

        // window createn

        while engine.running {
            app_struct.update(&mut engine);
            renderer.render_frame(engine.objects);
            engine.objects.clear(); // nach jedem frame alles wieder clearen
        }

        // window closen
    }
    pub fn quit(&mut self) {
        self.running = false;
    }
}
