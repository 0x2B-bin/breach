pub struct GameManager {
    matrix: Vec<Vec<u8>>,
    buffer_size: u8,
    difficulty: Difficulty,
    pub active_view: View,
    pub should_quit: bool,
}

enum Difficulty {
    Easy,
    Normal,
    Hard,
}

pub enum View {
    Menu,
    Game,
}

impl GameManager {
    pub fn new() -> Self {
        Self {
            matrix: Vec::new(),
            buffer_size: 4,
            difficulty: Difficulty::Easy,
            active_view: View::Menu,
            should_quit: false,
        }
    }
}
