use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
};

use crate::game_manager::{GameManager, View};

pub fn render(frame: &mut Frame, game_manager: &GameManager) {
    let main_layout = Layout::new(
        Direction::Horizontal,
        [Constraint::Percentage(50), Constraint::Percentage(50)],
    );

    let [left, right] = frame.area().layout(&main_layout);

    let block = Block::new().borders(Borders::ALL);

    frame.render_widget(&block, left);
    frame.render_widget(block, right);

    if let View::Menu = game_manager.active_view {}
}
