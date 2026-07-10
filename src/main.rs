use event::EventHandler;
use game_manager::GameManager;

use crate::{event::Event, update::update};

mod event;
mod game_manager;
mod ui;
mod update;

fn main() {
    let mut game_manager = GameManager::new();
    let mut terminal = ratatui::init();
    let event_handler = EventHandler::new(250);

    while !game_manager.should_quit {
        if let Event::Key(key_event) = event_handler.next().unwrap() {
            update(&mut game_manager, key_event);
        }
        let _ = terminal.draw(|frame| ui::render(frame, &game_manager));
    }

    ratatui::restore();
}
