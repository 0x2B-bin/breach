use ratatui::{
    Frame, layout::{Constraint, Direction, Layout, Rect}, style::{Style, Stylize}, text::{Line, Span}, widgets::{Block, Borders, Paragraph}
};

use crate::game_manager::{GameManager, View};

pub fn render(frame: &mut Frame, game_manager: &GameManager) {
    let main_layout = Layout::new(
        Direction::Horizontal,
        [Constraint::Percentage(50), Constraint::Percentage(50)],
    );

    let [left, right] = frame.area().layout(&main_layout);

    let block = Block::new().borders(Borders::ALL);

    if let View::Menu = game_manager.active_view {
        render_menu(frame, game_manager);
    } else {
        frame.render_widget(&block, left);
        frame.render_widget(block, right);
    }
}

fn render_menu(frame: &mut Frame, game_manager: &GameManager) {
    let mut diffs = vec![Line::from(" easy "), Line::from(" medium "), Line::from(" hard ")];

    diffs[game_manager.difficulty_idx] = diffs[game_manager.difficulty_idx].clone().style(Style::default().on_blue().bold());


    let title = Span::from(" Select Difficulty ").bold();
    let block = Block::default()
        .borders(Borders::ALL)
        .title_top(Line::from(title).centered());

    let paragraph = Paragraph::new(diffs).centered();

    let area = centered_rect(frame.area(), 20);
    let paragraph_area = centered_rect(area, 40);

    frame.render_widget(block, area);
    frame.render_widget(paragraph, paragraph_area);
}

fn centered_rect(area: Rect, percent: u16) -> Rect {
    let center = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent) / 2),
            Constraint::Percentage(percent),
            Constraint::Percentage((100 - percent) / 2),
        ])
        .split(area);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent) / 2),
            Constraint::Percentage(percent),
            Constraint::Percentage((100 - percent) / 2),
        ])
        .split(center[1])[1]
}
