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

            let object1 = Object::square((32, 32), (63, 82), Color::Blue);
            let object2 = Object::square((72, 32), (63, 82), Color::Blue);
            let object3 = Object::square((22, 52), (63, 82), Color::Blue);
            let objects = vec![object1, object2, object3];

            engine.draw_object(object);
            engine.draw_multiple_objects(objects);
        }
        engine.stop();
    }
}

fn main() {
    Engine::start(Settings::default(), &mut Game::new());
}
