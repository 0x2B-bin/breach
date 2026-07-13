use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

use crate::game_manager::{GameManager, MatrixDirection, View};

pub fn render(frame: &mut Frame, game_manager: &GameManager) {
    let main_layout = Layout::new(
        Direction::Horizontal,
        [Constraint::Percentage(30), Constraint::Percentage(70)],
    );

    let [left, right] = frame.area().layout(&main_layout);

    let block = Block::new().borders(Borders::ALL);

    if let View::Menu = game_manager.active_view {
        render_menu(frame, game_manager);
    } else {
        render_code_matrix(frame, left, game_manager);
        frame.render_widget(block, right);
    }
}

fn render_menu(frame: &mut Frame, game_manager: &GameManager) {
    let mut diffs = vec![
        Line::from(" easy "),
        Line::from(" medium "),
        Line::from(" hard "),
    ];

    diffs[game_manager.difficulty_idx] = diffs[game_manager.difficulty_idx]
        .clone()
        .style(Style::default().on_blue().bold());

    let title = Span::from(" Select Difficulty ").bold();
    let block = Block::default()
        .borders(Borders::ALL)
        .title_top(Line::from(title).centered());

    let paragraph = Paragraph::new(diffs).centered();

    let area = centered_rect(frame.area(), 20, 20);
    let paragraph_area = centered_rect(area, 50, 50);

    frame.render_widget(block, area);
    frame.render_widget(paragraph, paragraph_area);
}

fn render_code_matrix(frame: &mut Frame, area: Rect, game_manager: &GameManager) {
    let mut rows = Vec::with_capacity(game_manager.matrix_size);

    for (row_idx, row) in game_manager.matrix.iter().enumerate() {
        let mut spans = Vec::with_capacity(game_manager.matrix_size);
        for (col_idx, col) in row.iter().enumerate() {
            let matrix_chr = match col {
                0 => " 1C ",
                1 => " 55 ",
                2 => " BD ",
                3 => " E9 ",
                _ => "    ",
            };

            let mut span = Span::from(matrix_chr);

            if row_idx == game_manager.matrix_row_idx && col_idx == game_manager.matrix_col_idx {
                span = span.black();
            }

            match game_manager.maxtrix_direction {
                MatrixDirection::Row if row_idx == game_manager.matrix_row_idx => {
                    span = span.on_red();
                }
                MatrixDirection::Column if col_idx == game_manager.matrix_col_idx => {
                    span = span.on_red();
                }
                _ => {}
            }
            spans.push(span);
        }
        rows.push(Line::from(spans));
    }

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length((game_manager.matrix_size as u16) + 2),
            Constraint::Min(0),
        ])
        .split(area);

    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Code Matrix ".white())
        .green();

    let paragraph = Paragraph::new(rows).block(block).centered().white();

    frame.render_widget(paragraph, layout[0]);
}

fn centered_rect(area: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let center = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(area);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(center[1])[1]
}
