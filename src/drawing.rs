use crossterm::{
    QueueableCommand, cursor,
    event::{Event, KeyCode, poll, read},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::{
    io::{Write, stdout},
    time::Duration,
};

use crate::{Element, UpdateResult};

pub struct Screen {}
impl Screen {
    pub fn new() -> Self {
        enable_raw_mode().expect("Failed to enable raw mode");
        Self {}
    }

    fn read_events() -> Result<Vec<Event>, Box<dyn std::error::Error>> {
        let mut events = Vec::new();
        while poll(Duration::from_millis(1_000))? {
            let event = read()?;
            events.push(event);
        }

        Ok(events)
    }

    fn handle_key_events(events: &Vec<Event>) -> UpdateResult {
        for event in events {
            match event {
                Event::FocusGained => {}
                Event::FocusLost => {}
                Event::Key(key_event) => {
                    // Handle key events
                    // TODO: implement CTL + C exit/kill
                    if key_event.code == KeyCode::Esc {
                        return UpdateResult::Exit;
                    }
                }
                Event::Mouse(mouse_event) => {}
                Event::Paste(_) => {}
                Event::Resize(_, _) => {}
            }
        }
        UpdateResult::Continue
    }

    /// Update the screen with the given elements.
    pub fn update(&mut self, elements: &Vec<Element>) -> UpdateResult {
        match Self::read_events() {
            Ok(events) => {
                let result = Self::handle_key_events(&events);
                if result == UpdateResult::Exit {
                    return UpdateResult::Exit;
                }
            }
            Err(e) => {
                eprintln!("Error reading events: {e}");
                return UpdateResult::Exit;
            }
        }

        UpdateResult::Continue
    }
}

impl Drop for Screen {
    fn drop(&mut self) {
        let _ = stdout().flush();

        disable_raw_mode().expect("Failed to disable raw mode");
    }
}
