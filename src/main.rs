use ascii_engine::*;

struct Game {}
impl Game {
    pub fn new() -> Self {
        Self {}
    }
}

impl App for Game {
    fn update(&mut self, engine: &mut Engine) {
        engine.draw_line((0, 0), (1920, 1080), 1, Color::White);
        engine.stop();
    }
}

fn main() {
    Engine::start(Settings::default(), &mut Game::new());
}
