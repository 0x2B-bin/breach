use ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::game_manager::GameManager;

pub fn update(game_manager: &mut GameManager, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Esc | KeyCode::Char('q') => game_manager.should_quit = true,
        _ => {}
    }
}
