use ascii_engine::*;

struct Game {}

impl Game {
    pub fn new() -> Self {
        Self {}
    }
}

impl App for Game {
    fn update(&mut self, engine: &mut Engine) {
        for i in 0..10 {
            let object = Object::circle((32, 32), 6, Color::Red);
            engine.draw_object(object);
        }
        engine.stop();
    }
}

fn main() {
    Engine::start(Settings::default(), &mut Game::new());
}
