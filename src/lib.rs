/// Main application for building TUI programs.
pub struct App<State> {
    state: State,
}
impl<State> App<State> {
    /// Create a new application with the given state.
    pub fn new(state: State) -> Self {
        Self { state }
    }

    /// Run the application.
    pub fn run(
        &mut self,
        mut update: impl FnMut(Context<State>) -> UpdateResult,
    ) -> Result<(), Box<dyn std::error::Error>> {
        loop {
            let context = Context {
                state: &mut self.state,
            };
            match update(context) {
                UpdateResult::Continue => (),
                UpdateResult::Exit => break,
            }
        }

        Ok(())
    }
}

/// Context for building UI screens.
pub struct Context<'a, State> {
    pub state: &'a mut State,
}

/// Result of updating the application.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpdateResult {
    /// Continue to the next update.
    Continue,
    /// Exit the application.
    Exit,
}
