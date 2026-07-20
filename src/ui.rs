use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

use crate::game_manager::{GameManager, MatrixDirection, View};

trait MatrixDisplay {
    fn display(&self) -> &str;
}

impl MatrixDisplay for u8 {
    fn display(&self) -> &str {
        match self {
            0 => " 1C ",
            1 => " 55 ",
            2 => " BD ",
            3 => " E9 ",
            4 => " 7A ",
            255 => " [] ",
            _ => "    ",
        }
    }
}

pub fn render(frame: &mut Frame, game_manager: &GameManager) {
    let area = centered_rect(frame.area(), 40, 50);
    let areas = Layout::new(
        Direction::Vertical,
        [
            Constraint::Length((game_manager.matrix_size as u16) + 2),
            Constraint::Length(3),
            Constraint::Fill(1),
        ],
    )
    .split(area);

    if let View::Menu = game_manager.active_view {
        render_menu(frame, game_manager);
    } else {
        render_code_matrix(frame, areas[0], game_manager);
        render_buffer(frame, areas[1], game_manager);
        render_sequences(frame, areas[2], game_manager);
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
            let matrix_chr = col.display();

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

    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Code Matrix ".white())
        .green();

    let paragraph = Paragraph::new(rows).block(block).centered().white();

    frame.render_widget(paragraph, area);
}

fn render_buffer(frame: &mut Frame, area: Rect, game_manager: &GameManager) {
    let block = Block::default().borders(Borders::ALL).title(" Buffer ");
    let buffer_area = centered_rect(area, 100, 50);

    let mut codes = Vec::new();
    for code in game_manager.buffer.iter() {
        codes.push(Span::from(code.display()));
    }

    let empty_slots_amount = game_manager.buffer_size - game_manager.buffer.len();

    if empty_slots_amount > 0 {
        for _ in 0..empty_slots_amount {
            codes.push(Span::from(" [] "));
        }
    }

    let buffer_line = Line::from(codes).centered();

    frame.render_widget(block, area);
    frame.render_widget(buffer_line, buffer_area);
}

fn render_sequences(frame: &mut Frame, area: Rect, game_manager: &GameManager) {
    let block = Block::default().borders(Borders::ALL).title(" Sequences ");
    let center_area = centered_rect(area, 100, 50);

    let mut sequence_lines = Vec::with_capacity(game_manager.sequences.len());

    for sequence in game_manager.sequences.iter() {
        let mut sequence_txt = String::with_capacity(sequence.len());
        for cell in sequence {
            sequence_txt += cell.display();
        }
        sequence_lines.push(Line::from(sequence_txt));
    }

    let paragraph = Paragraph::new(sequence_lines).centered();

    frame.render_widget(block, area);
    frame.render_widget(paragraph, center_area);
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
