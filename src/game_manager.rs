use rand::RngExt;

pub struct GameManager {
    pub matrix: Vec<Vec<u8>>,
    pub matrix_size: usize,
    pub matrix_row_idx: usize,
    pub matrix_col_idx: usize,
    pub maxtrix_direction: MatrixDirection,
    pub buffer: Vec<u8>,
    pub buffer_size: usize,
    pub difficulty: Difficulty,
    pub difficulty_idx: usize,
    pub active_view: View,
    pub should_quit: bool,
}

pub enum Difficulty {
    Easy,
    Normal,
    Hard,
}

pub enum MatrixDirection {
    Row,
    Column,
}

pub enum View {
    Menu,
    Game,
}

impl GameManager {
    pub fn new() -> Self {
        Self {
            matrix: Vec::new(),
            matrix_size: 0,
            matrix_row_idx: 0,
            matrix_col_idx: 0,
            maxtrix_direction: MatrixDirection::Row,
            buffer: Vec::new(),
            buffer_size: 4,
            difficulty: Difficulty::Easy,
            difficulty_idx: 0,
            active_view: View::Menu,
            should_quit: false,
        }
    }

    pub fn generate_matrix(&mut self) {
        let mut rng = rand::rng();
        let size = match self.difficulty {
            Difficulty::Easy => 4,
            Difficulty::Normal => 5,
            Difficulty::Hard => 6,
        };

        self.matrix_size = size;

        let mut matrix = Vec::new();

        for _ in 0..size {
            let mut row = Vec::new();
            for _ in 0..size {
                row.push(rng.random_range(0..=3));
            }
            matrix.push(row);
        }

        self.matrix = matrix;
    }
}
