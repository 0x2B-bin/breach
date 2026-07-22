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
    pub sequences: Vec<Vec<u8>>,
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

impl MatrixDirection {
    pub fn toggle(&mut self) {
        *self = match self {
            MatrixDirection::Row => MatrixDirection::Column,
            MatrixDirection::Column => MatrixDirection::Row,
        }
    }
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
            buffer_size: 6,
            difficulty: Difficulty::Easy,
            difficulty_idx: 0,
            active_view: View::Menu,
            sequences: Vec::new(),
            should_quit: false,
        }
    }

    fn generate_master_path(&mut self) -> Vec<u8> {
        let mut rng = rand::rng();
        let mut sequence = Vec::with_capacity(self.matrix_size);
        let mut direction = MatrixDirection::Row;
        let mut row = 0;
        let mut col = rng.random_range(0..self.matrix_size);
        let mut visted = Vec::with_capacity(self.matrix_size);

        while sequence.len() < self.matrix_size {
            while visted.contains(&(row, col)) {
                match direction {
                    MatrixDirection::Row => col = rng.random_range(0..self.matrix_size),
                    MatrixDirection::Column => row = rng.random_range(0..self.matrix_size),
                }
            }

            let cell_value = rng.random_range(0..=4);
            self.matrix[row][col] = cell_value;
            sequence.push(cell_value);
            visted.push((row, col));
            direction.toggle();
        }

        sequence
    }

    fn generate_sequences(&self, master_path: &[u8]) -> Vec<Vec<u8>> {
        let mut rng = rand::rng();
        let sequences_target_len = match self.difficulty {
            Difficulty::Easy => 1,
            Difficulty::Normal => 2,
            Difficulty::Hard => 3,
        };
        let mut sequences = Vec::with_capacity(sequences_target_len);

        if let Difficulty::Easy = self.difficulty {
            sequences.push(master_path[..].to_vec());
            return sequences
        }

        let mut slices : Vec<(usize, usize)> = Vec::with_capacity(sequences_target_len);
        let mut cursor = 0;
        while slices.len() < sequences_target_len {
            let mut remaining_spaces = master_path.len().saturating_sub(cursor + 1);
            let length = if remaining_spaces >= 4 {
                rng.random_range(2..=4)
            } else {
                rng.random_range(2..=remaining_spaces)
            };

            slices.push((cursor, cursor+length));

            let mut step = rng.random_range(0..=length);
            
            if step + cursor >= master_path.len() - 1 {
                step = master_path.len() - (cursor + 1)
            }

            cursor += step;
            remaining_spaces = master_path.len().saturating_sub(cursor + 1);

            if remaining_spaces < 2 { 
                cursor -= step;
                slices.pop();
            }
        }

        for slice in slices {
            sequences.push(master_path[slice.0..slice.1].to_vec());
        }

        sequences.sort_by(|a, b| {
            a.len().cmp(&b.len())
        });
        sequences
    }

    fn randomize_empty_cells(&mut self) {
        let mut rng = rand::rng();
        self.matrix.iter_mut().for_each(|row| {
            row.iter_mut().for_each(|col| {
                if *col == 255 {
                    *col = rng.random_range(0..=4) as u8;
                }
            })
        });
    }

    pub fn generate_matrix(&mut self) {
        self.matrix_size = match self.difficulty {
            Difficulty::Easy => 4,
            Difficulty::Normal => 5,
            Difficulty::Hard => 6,
        };

        self.matrix = vec![vec![255_u8; self.matrix_size]; self.matrix_size];

        let master_path = self.generate_master_path();
        self.sequences = self.generate_sequences(&master_path);
        self.randomize_empty_cells();
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
