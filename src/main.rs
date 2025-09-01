use ascii_engine::*;

struct Game {}
impl Game {
    pub fn new() -> Self {
        Self {}
    }
}

impl App for Game {
    fn update(&mut self, engine: &mut Engine) {
        for i in 0..5 {
            engine.draw_line((0, 0), (1920, 1080), 1.0, Color::White, 0);
            engine.draw_line((0, 0), (1920, 1080), 1.0, Color::White, 0);
            engine.draw_line((0, 0), (1920, 1080), 1.0, Color::White, 0);
        }
        engine.stop();
    }
}

fn main() {
    Engine::start(Settings::default(), &mut Game::new());
}
