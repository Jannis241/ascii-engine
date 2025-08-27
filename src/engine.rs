use crate::{renderer::Renderer, Settings};

type Pos = (u32, u32);

pub trait App {
    fn update(&mut self, engine: &mut Engine);
}

enum Color {
    Black,
    White,
    Grey,
    Green,
    Blue,
    Yellow,
}

pub struct Character {
    content: char,
    pos: Pos,
    color: Color,
}

impl Character {
    pub fn new(content: char, pos: Pos, color: Color) -> Self {
        Self {
            content,
            pos,
            color,
        }
    }
}

pub struct Engine {
    running: bool,
    frame: Vec<Vec<Character>>,
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

    pub fn draw_line(&self, start_pos: Pos, end_pos: Pos, dicke: f32, color: Color) {
        // todo: wirklich genau die symbole und alles ausrechnen und dann in self.objects packen
    }
    pub fn draw_circle(&self) {}
    pub fn draw_square(&self) {}
    pub fn draw_triangle(&self) {}

    fn create(settings: Settings) -> Self {
        Engine {
            running: true,
            frame: vec![vec![]],
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
            renderer.render_frame(&settings.screen_size, &engine.frame);

            engine.frame.clear();
        }

        // Todo: Ascii "fenster" beenden.
    }
    pub fn stop(&mut self) {
        self.running = false;
    }
}
