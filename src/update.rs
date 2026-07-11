use ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::game_manager::{self, GameManager, View};

pub fn update(game_manager: &mut GameManager, key_event: KeyEvent) {
    match game_manager.active_view {
        View::Menu => match key_event.code {
            KeyCode::Esc | KeyCode::Char('q') => game_manager.should_quit = true,
            KeyCode::Down | KeyCode::Char('j') if game_manager.difficulty_idx < 2 => {
                game_manager.difficulty_idx += 1
            }
            KeyCode::Up | KeyCode::Char('k') if game_manager.difficulty_idx > 0 => {
                game_manager.difficulty_idx -= 1
            },
            KeyCode::Enter => {
                game_manager.active_view = View::Game;
            }
            _ => {}
        },
        View::Game => match key_event.code {
            KeyCode::Esc | KeyCode::Char('q') => game_manager.should_quit = true,
            _ => {}
        },
    };
}
