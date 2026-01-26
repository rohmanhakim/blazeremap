use std::time::Instant;

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
    event_count: u64,
    total_latency_us: u64,

    // Statistics
    max_latency_us: u64,
    min_latency_us: u64,
}

impl EventLoop {
    pub fn new(
        controller: Box<dyn Gamepad>,
        engine: MappingEngine,
        keyboard: Box<dyn VirtualKeyboard>,
    ) -> Self {
        Self {
            gamepad: controller,
            engine,
            keyboard,
            event_count: 0,
            total_latency_us: 0,
            max_latency_us: 0,
            min_latency_us: u64::MAX,
        }
    }

    /// Run the event loop (blocking)
    pub fn run(mut self) -> Result<()> {
        tracing::info!("Event loop starting...");

        loop {
            match self.gamepad.read_event()? {
                Some(input_event) => {
                    let start = Instant::now();
                    // Process through mapping engine
                    for output_event in self.engine.process(&input_event)? {
                        #[cfg(debug_assertions)] // Only trace per button event in debug build to not interrupt latency
                        tracing::debug!("Gamepad: {} -> {}", input_event, output_event);

                        self.emit_output(output_event)?;
                    }

                    // Measure ONLY processing latency
                    let latency_us = start.elapsed().as_micros() as u64;

                    self.event_count += 1;
                    self.total_latency_us += latency_us;
                    self.max_latency_us = self.max_latency_us.max(latency_us);
                    self.min_latency_us = self.min_latency_us.min(latency_us);

                    // Log statistics every 100 events
                    if self.event_count.is_multiple_of(100) {
                        let avg = self.total_latency_us / self.event_count;
                        tracing::info!(
                            "Stats: {} events | avg: {}µs ({:.2}ms) | min: {}µs | max: {}µs",
                            self.event_count,
                            avg,
                            avg as f64 / 1000.0,
                            self.min_latency_us,
                            self.max_latency_us
                        );
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
        // Print final statistics
        if self.event_count > 0 {
            let avg = self.total_latency_us / self.event_count;
            tracing::info!(
                "Final: {} events | avg: {}µs ({:.2}ms) | min: {}µs | max: {}µs",
                self.event_count,
                avg,
                avg as f64 / 1000.0,
                self.min_latency_us,
                self.max_latency_us
            );
        }
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
