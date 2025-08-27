use ascii_engine::*;

struct Game {}

impl Game {
    pub fn new() -> Self {
        Self {}
    }
}

impl App for Game {
    fn update(&mut self, engine: &mut Engine) {}
}

fn main() {
    let settings = Settings::default();
    let mut my_game = Game::new();
    Engine::start(settings, &mut my_game);
}
