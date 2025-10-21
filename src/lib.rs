mod drawing;

use crate::drawing::Screen;

/// Main application for building TUI programs.
pub struct App<State> {
    state: State,
    screen: Screen,
}
impl<State> App<State> {
    /// Create a new application with the given state.
    pub fn new(state: State) -> Self {
        Self {
            state,
            screen: Screen::new(),
        }
    }

    /// Run the application.
    pub fn run(
        &mut self,
        mut update: impl FnMut(Context<State>) -> UpdateResult,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut keep_running = true;
        while keep_running {
            let mut elements = Vec::new();
            let result = {
                let context = Context {
                    state: &mut self.state,
                    elements: &mut elements,
                };

                update(context)
            };

            match result {
                UpdateResult::Continue => (),
                UpdateResult::Exit => keep_running = false,
            }

            // Update screen with elements
            let result = self.screen.update(&elements);
            if result == UpdateResult::Exit {
                keep_running = false;
            }
        }

        Ok(())
    }
}

/// Context for building UI screens.
pub struct Context<'a, State> {
    pub state: &'a mut State,
    elements: &'a mut Vec<Element>,
}

impl<'a, State> Context<'a, State> {
    /// Add a label to the context.
    pub fn label(&mut self, label: &str) -> &mut Self {
        self.elements.push(Element::Label(label.to_string()));
        self
    }
}

pub enum Element {
    Label(String),
}

/// Result of updating the application.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpdateResult {
    /// Continue to the next update.
    Continue,
    /// Exit the application.
    Exit,
}
