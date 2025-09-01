use crate::{renderer::Renderer, Settings};

type Pos = (u32, u32);

pub trait App {
    fn update(&mut self, engine: &mut Engine);
}

pub enum Color {
    Black,
    White,
    Grey,
    Green,
    Blue,
    Yellow,
}

pub struct Object {
    blocked_points: Vec<u32>,
    z_index: u32,
    color: Color,
}

// Todo: Später alle methoden public machen, damit man selber objecte erstellen kann
// und nicht erst zb draw_line aufrufen muss sondern das lineobject erstellen kann
// und es direkt drawn kann.
impl Object {
    fn custom(blocked_points: Vec<u32>, z_index: u32, color: Color) -> Self {
        Object {
            blocked_points,
            z_index,
            color,
        }
    }
    pub fn get_blocked_points(&self) -> &Vec<u32> {
        return &self.blocked_points;
    }
    pub fn get_color(&self) -> &Color {
        return &self.color;
    }
    pub fn get_z_index(&self) -> &u32 {
        return &self.z_index;
    }
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

    // Todo: index einrichten -> dass man gesteuert bestimmte dinge über andere zeichnen kann.

    pub fn draw_line(
        &mut self,
        start_pos: Pos,
        end_pos: Pos,
        dicke: f32,
        color: Color,
        z_index: u32,
    ) {
        // todo: wirklich genau die symbole und alles ausrechnen und dann in self.objects packen
        let object = Object::custom(vec![3, 5, 6, 7], 1, Color::White);
        self.objects.push(object);
    }
    pub fn draw_circle(&self) {}
    pub fn draw_square(&self) {}
    pub fn draw_triangle(&self) {}

    fn create(settings: Settings) -> Self {
        Engine {
            running: true,
            objects: Vec::new(),
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
