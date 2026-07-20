use ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::game_manager::{Difficulty, GameManager, MatrixControl, MatrixDirection, View};

pub fn update(game_manager: &mut GameManager, key_event: KeyEvent) {
    match game_manager.active_view {
        View::Menu => match key_event.code {
            KeyCode::Esc | KeyCode::Char('q') => game_manager.should_quit = true,
            KeyCode::Down | KeyCode::Char('j') if game_manager.difficulty_idx < 2 => {
                game_manager.difficulty_idx += 1
            }
            KeyCode::Up | KeyCode::Char('k') if game_manager.difficulty_idx > 0 => {
                game_manager.difficulty_idx -= 1
            }
            KeyCode::Enter => {
                game_manager.active_view = View::Game;
                game_manager.difficulty = match game_manager.difficulty_idx {
                    0 => Difficulty::Easy,
                    1 => Difficulty::Normal,
                    2 => Difficulty::Hard,
                    _ => Difficulty::Easy,
                };
                game_manager.generate_matrix();
            }
            _ => {}
        },
        View::Game => match key_event.code {
            KeyCode::Esc | KeyCode::Char('q') => game_manager.should_quit = true,
            KeyCode::Right | KeyCode::Char('l')
                if let MatrixDirection::Row = game_manager.maxtrix_direction =>
            {
                game_manager.matrix_select_next(MatrixControl::Forward);
            }
            KeyCode::Left | KeyCode::Char('h')
                if let MatrixDirection::Row = game_manager.maxtrix_direction =>
            {
                game_manager.matrix_select_next(MatrixControl::Backward);
            }
            KeyCode::Up | KeyCode::Char('k')
                if let MatrixDirection::Column = game_manager.maxtrix_direction =>
            {
                game_manager.matrix_select_next(MatrixControl::Backward);
            }
            KeyCode::Down | KeyCode::Char('j')
                if let MatrixDirection::Column = game_manager.maxtrix_direction =>
            {
                game_manager.matrix_select_next(MatrixControl::Forward);
            }
            KeyCode::Enter => {
                game_manager.matrix_confirm_selection();
                game_manager.maxtrix_direction.toggle();
            }
            _ => {}
        },
    };
}
