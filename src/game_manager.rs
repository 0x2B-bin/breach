pub struct GameManager {
    matrix: Vec<Vec<u8>>,
    buffer_size: u8,
    difficulty: Difficulty,
    pub difficulty_idx: usize,
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
            difficulty_idx: 0,
            active_view: View::Menu,
            should_quit: false,
        }
    }
}
