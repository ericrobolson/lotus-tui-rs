use crossterm::{
    QueueableCommand, cursor,
    event::{Event, KeyCode, poll, read},
    execute, queue,
    style::Print,
    terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode},
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
        while poll(Duration::from_millis(1))? {
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
    pub fn update(
        &mut self,
        elements: &Vec<Element>,
    ) -> Result<UpdateResult, Box<dyn std::error::Error>> {
        match Self::read_events() {
            Ok(events) => {
                let result = Self::handle_key_events(&events);
                if result == UpdateResult::Exit {
                    return Ok(UpdateResult::Exit);
                }
            }
            Err(e) => {
                return Err(e);
            }
        }

        // Draw things
        let mut out = stdout();
        out.queue(Clear(ClearType::All))?;
        out.queue(cursor::Hide)?;

        for element in elements {
            match element {
                Element::Label(label) => {
                    out.queue(cursor::MoveTo(0, 0))?;
                    out.queue(Print(label))?;
                }
            }
        }

        out.flush().expect("Failed to flush stdout");

        Ok(UpdateResult::Continue)
    }
}

impl Drop for Screen {
    fn drop(&mut self) {
        disable_raw_mode().expect("Failed to disable raw mode");

        // Reset the screen
        let mut out = stdout();
        out.queue(Clear(ClearType::All))
            .expect("Failed to clear screen");
        out.queue(cursor::Show).expect("Failed to show cursor");
        out.flush().expect("Failed to flush stdout");
    }
}
