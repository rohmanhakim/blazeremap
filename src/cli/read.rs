use std::time::Instant;

use crate::device::controller::Controller;
use crate::platform::linux::LinuxController;
use anyhow::Result;
use clap::Command;

pub fn command() -> Command {
    Command::new("read").about("Read and display controller events (debugging)").arg(
        clap::Arg::new("device")
            .help("Device path (e.g., /dev/input/event3)")
            .required(true)
            .index(1),
    )
}

pub fn handle(matches: &clap::ArgMatches) -> Result<()> {
    let device_path = matches.get_one::<String>("device").unwrap();

    println!("Opening device: {}", device_path);
    let mut controller = LinuxController::open(device_path)?;

    println!("Reading events (Ctrl+C to stop)...\n");
    println!("Format: [elapsed since first event][Δ from previous] Event\n");

    let mut first_event_timestamp: Option<Instant> = None;
    let mut last_timestamp: Option<Instant> = None;

    loop {
        match controller.read_event()? {
            Some(event) => {
                if !matches!(event, crate::event::InputEvent::Sync { .. }) {
                    let timestamp = event.timestamp();

                    // Initialize start time on the first actual event received
                    let first = *first_event_timestamp.get_or_insert(timestamp);
                    let elapsed = timestamp.saturating_duration_since(first);

                    // Calculate delta from previous event
                    let delta = if let Some(last) = last_timestamp {
                        timestamp.saturating_duration_since(last).as_micros()
                    } else {
                        0
                    };

                    println!(
                        "[{:>8.5}ms][Δ {:>8}µs] {}",
                        elapsed.as_secs_f64() * 1000.0,
                        delta,
                        event
                    );

                    last_timestamp = Some(timestamp);
                }
            }
            None => {
                println!("Device disconnected");
                break;
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_structure() {
        let cmd = command();
        assert_eq!(cmd.get_name(), "read");
        assert!(cmd.get_about().unwrap().to_string().contains("debugging"));
    }

    #[test]
    fn test_command_has_required_device_arg() {
        let cmd = command();

        // Check that device argument exists
        let device_arg = cmd.get_arguments().find(|arg| arg.get_id() == "device");
        assert!(device_arg.is_some());

        let device_arg = device_arg.unwrap();
        assert!(device_arg.is_required_set());
        assert!(device_arg.get_help().unwrap().to_string().contains("/dev/input/event"));
    }
}
