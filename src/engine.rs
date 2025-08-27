use crate::{object::Object, renderer::Renderer, Settings};

pub trait App {
    fn update(&mut self, engine: &mut Engine);
}

pub struct Engine {
    running: bool,
    objects: Vec<Object>,
    settings: Settings,
}

impl Engine {
    pub fn get_screenheight(&self) -> u32 {
        self.settings.screen_size.1
    }

    pub fn get_screenwidth(&self) -> u32 {
        self.settings.screen_size.0
    }

    // help functions -> eigentlich soll alles über draw object lafuen.
    pub fn draw_line(&self) {}
    pub fn draw_circle(&self) {}
    pub fn draw_square(&self) {}
    pub fn draw_triangle(&self) {}

    pub fn draw_object(&mut self, object: Object) {
        self.objects.push(object);
    }

    pub fn draw_multiple_objects(&mut self, objects: Vec<Object>) {
        self.objects.extend(objects);
    }

    fn create(settings: Settings) -> Self {
        Engine {
            running: true,
            objects: vec![],
            settings,
        }
    }

    pub fn start(settings: Settings, app_struct: &mut impl App) {
        let mut engine = Engine::create(settings.clone());
        let renderer = Renderer::new();

        // Todo: Ascii "fenster" starten.

        while engine.running {
            app_struct.update(&mut engine);

            // Todo: Refresh rate einbauen -> konstat aktualisieren.
            renderer.render_frame(&settings.screen_size, &engine.objects);

            engine.objects.clear();
        }

        // Todo: Ascii "fenster" beenden.
    }
    pub fn stop(&mut self) {
        self.running = false;
    }
}
