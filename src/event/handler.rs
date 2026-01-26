use anyhow::Result;

use crate::{
    Gamepad,
    event::{KeyboardEventType, OutputEvent},
    mapping::MappingEngine,
    output::keyboard::VirtualKeyboard,
};

pub struct EventLoop {
    gamepad: Box<dyn Gamepad>,
    engine: MappingEngine,
    keyboard: Box<dyn VirtualKeyboard>,
}

impl EventLoop {
    pub fn new(
        controller: Box<dyn Gamepad>,
        engine: MappingEngine,
        keyboard: Box<dyn VirtualKeyboard>,
    ) -> Self {
        Self { gamepad: controller, engine, keyboard }
    }

    /// Run the event loop (blocking)
    pub fn run(mut self) -> Result<()> {
        tracing::info!("Event loop starting...");

        loop {
            // Read event from controller (blocking)
            match self.gamepad.read_event()? {
                Some(input_event) => {
                    // Process through mapping engine
                    for output_event in self.engine.process(&input_event)? {
                        tracing::info!("Gamepad: {} -> {}", input_event, output_event);
                        self.emit_output(output_event)?;
                    }
                }
                None => {
                    // Controller disconnected
                    tracing::warn!("Controller disconnected");
                    break;
                }
            }
        }

        tracing::info!("Event loop stopped");
        Ok(())
    }

    fn emit_output(&mut self, output_event: OutputEvent) -> Result<()> {
        match output_event {
            OutputEvent::Keyboard { code, event_type } => {
                if event_type == KeyboardEventType::Press {
                    self.keyboard.press_key(code)?;
                } else if event_type == KeyboardEventType::Release {
                    self.keyboard.release_key(code)?;
                }
            }
        }

        Ok(())
    }
}
