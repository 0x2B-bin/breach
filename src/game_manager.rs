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

pub enum MatrixControl {
    Forward,
    Backward,
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

    pub fn matrix_select_next(&mut self, control: MatrixControl) {
        fn next_open_cell(
            row: usize,
            col: usize,
            game_manager: &GameManager,
            direction: &MatrixDirection,
            control: MatrixControl,
        ) -> Option<(usize, usize)> {
            let mut row = row;
            let mut col = col;

            match direction {
                MatrixDirection::Row => match control {
                    MatrixControl::Forward => {
                        if col < game_manager.matrix_size - 1 {
                            col += 1;
                            if game_manager.matrix[row][col] != 255 {
                                Some((row, col))
                            } else {
                                next_open_cell(row, col, game_manager, direction, control)
                            }
                        } else {
                            None
                        }
                    }
                    MatrixControl::Backward => {
                        if col > 0 {
                            col -= 1;
                            if game_manager.matrix[row][col] != 255 {
                                Some((row, col))
                            } else {
                                next_open_cell(row, col, game_manager, direction, control)
                            }
                        } else {
                            None
                        }
                    }
                },
                MatrixDirection::Column => match control {
                    MatrixControl::Forward => {
                        if row < game_manager.matrix_size - 1 {
                            row += 1;
                            if game_manager.matrix[row][col] != 255 {
                                Some((row, col))
                            } else {
                                next_open_cell(row, col, game_manager, direction, control)
                            }
                        } else {
                            None
                        }
                    }
                    MatrixControl::Backward => {
                        if row > 0 {
                            row -= 1;
                            if game_manager.matrix[row][col] != 255 {
                                Some((row, col))
                            } else {
                                next_open_cell(row, col, game_manager, direction, control)
                            }
                        } else {
                            None
                        }
                    }
                },
            }
        }

        if let Some((row, col)) = next_open_cell(
            self.matrix_row_idx,
            self.matrix_col_idx,
            self,
            &self.maxtrix_direction,
            control,
        ) {
            self.matrix_row_idx = row;
            self.matrix_col_idx = col;
        }
    }

    pub fn matrix_confirm_selection(&mut self) {
        let current_cell = &mut self.matrix[self.matrix_row_idx][self.matrix_col_idx];

        if *current_cell != 255 {
            if self.buffer.len() < self.buffer_size {
                self.buffer.push(*current_cell);
            }
            *current_cell = 255;
        }
    }
}
