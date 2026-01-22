// Application composition
use crate::cli;

pub struct App;

impl App {
    pub fn new() -> Self {
        Self
    }

    /// Run the application
    pub fn run(&self) -> anyhow::Result<()> {
        cli::execute()
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
