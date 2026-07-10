use anyhow::Result;
use ratatui::crossterm::event::{self, KeyEvent};
use std::{
    sync::mpsc,
    thread::{self, JoinHandle},
    time::{Duration, Instant},
};

pub struct EventHandler {
    sender: mpsc::Sender<Event>,
    receiver: mpsc::Receiver<Event>,
    handler: JoinHandle<()>,
}

impl EventHandler {
    pub fn new(tick_rate: u64) -> Self {
        let tick_rate = Duration::from_millis(tick_rate);
        let (sender, receiver) = mpsc::channel();

        let handler = {
            let sender = sender.clone();
            let mut last_tick = Instant::now();
            thread::spawn(move || {
                loop {
                    let timeout = tick_rate
                        .checked_sub(last_tick.elapsed())
                        .unwrap_or(tick_rate);
                    if event::poll(timeout).unwrap() {
                        if let event::Event::Key(key_event) = event::read().unwrap()
                            && key_event.kind == event::KeyEventKind::Press
                        {
                            let _ = sender.send(Event::Key(key_event));
                        }
                    }

                    if last_tick.elapsed() >= tick_rate {
                        let _ = sender.send(Event::Tick);
                        last_tick = Instant::now();
                    }
                }
            })
        };

        Self {
            sender,
            receiver,
            handler,
        }
    }

    pub fn next(&self) -> Result<Event> {
        Ok(self.receiver.recv()?)
    }
}

pub enum Event {
    Tick,
    Key(KeyEvent),
}
