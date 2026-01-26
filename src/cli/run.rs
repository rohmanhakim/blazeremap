use anyhow::{Context, Result};
use clap::Command;

use crate::{
    InputManager,
    event::EventLoop,
    mapping::MappingEngine,
    output::keyboard::VirtualKeyboard,
    platform::{new_input_manager, new_virtual_keyboard},
};

/// Build the 'run' command
pub fn command() -> Command {
    Command::new("run").about("Run the remapping daemon").arg(
        clap::Arg::new("device")
            .short('d')
            .long("device")
            .help("Specific device path (auto-detect if not specified)"),
    )
}

/// CLI handle for the 'run' command
pub fn handle(matches: &clap::ArgMatches) -> Result<()> {
    let manager = new_input_manager();

    run_internal(matches, manager.as_ref(), new_virtual_keyboard)
}

/// Internal run logic that is decoupled from platform-specific implementations for testing
///
/// This split enables:
/// - Testing without real hardware (via mocks)
/// - Future cross-platform support (Windows/macOS)
/// - Independent testing of business logic vs. platform integration
fn run_internal<F>(
    matches: &clap::ArgMatches,
    manager: &dyn InputManager,
    make_keyboard: F,
) -> Result<()>
where
    F: FnOnce(&str) -> Result<Box<dyn VirtualKeyboard>>,
{
    tracing::info!("BlazeRemap v{} starting...", env!("CARGO_PKG_VERSION"));

    // Get device path
    let device_path = if let Some(path) = matches.get_one::<String>("device") {
        path.clone() // User specified a device path
    } else {
        // Auto-detect first controller
        println!("Detecting controllers...");
        let gamepads = manager.list_gamepads()?;

        if gamepads.gamepad_info.is_empty() {
            anyhow::bail!("No controllers detected. Please connect a controller.");
        }

        println!("Found {} gamepad(s)", gamepads.gamepad_info.len());
        println!("Using: {}", gamepads.gamepad_info[0].name);
        gamepads.gamepad_info[0].path.clone()
    };

    // Open controller
    println!("Opening device: {}", device_path);
    let controller = manager.open_gamepad(&device_path).context("Failed to open controller")?;

    // Create mapping engine
    println!("Loading hardcoded mappings...");
    let engine = MappingEngine::new_hardcoded();

    // Create virtual keyboard
    println!("Creating virtual keyboard...");
    let keyboard = make_keyboard("BlazeRemap Virtual Keyboard")
        .context("Failed to create virtual keyboard")?;

    println!("\nBlazeRemap is now running!");
    println!("Mappings:");
    println!("  D-pad button → Arrow");
    println!("  South button → S");
    println!("  West button → A");
    println!("  East button → D");
    println!("\nPress Ctrl+C to exit.\n");

    // Create and run event loop
    let event_loop = EventLoop::new(controller, engine, keyboard);
    event_loop.run()?;

    println!("BlazeRemap stopped.");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::InputDetectionResult;
    use crate::input::gamepad::{GamepadInfo, GamepadType, MockGamepad};
    use crate::input::manager::MockInputManager;
    use crate::output::keyboard::MockVirtualKeyboard;

    #[test]
    fn test_run_logic_auto_detect_success() {
        let mut mock_manager = MockInputManager::new();
        let gamepad_path = "/dev/input/eventX";

        // Mock gamepad listing
        mock_manager.expect_list_gamepads().returning(move || {
            Ok(InputDetectionResult {
                gamepad_info: vec![GamepadInfo {
                    path: gamepad_path.to_string(),
                    name: "Test Gamepad".to_string(),
                    gamepad_type: GamepadType::XboxOne,
                    vendor_id: 0,
                    vendor_name: "".to_string(),
                    product_id: 0,
                    capabilities: vec![],
                }],
                errors: vec![],
            })
        });

        // Mock gamepad opening
        mock_manager.expect_open_gamepad().with(mockall::predicate::eq(gamepad_path)).returning(
            |_| {
                let mut mock_gamepad = MockGamepad::new();
                // Simulation of controller disconnection to exit loop
                mock_gamepad.expect_read_event().returning(|| Ok(None));
                Ok(Box::new(mock_gamepad))
            },
        );

        let matches = command().get_matches_from(vec!["run"]);

        let result =
            run_internal(&matches, &mock_manager, |_| Ok(Box::new(MockVirtualKeyboard::new())));

        assert!(result.is_ok());
    }

    #[test]
    fn test_run_logic_no_gamepads_error() {
        let mut mock_manager = MockInputManager::new();

        mock_manager
            .expect_list_gamepads()
            .returning(|| Ok(InputDetectionResult { gamepad_info: vec![], errors: vec![] }));

        let matches = command().get_matches_from(vec!["run"]);

        let result =
            run_internal(&matches, &mock_manager, |_| Ok(Box::new(MockVirtualKeyboard::new())));

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "No controllers detected. Please connect a controller."
        );
    }

    #[test]
    fn test_run_logic_manual_device() {
        let mut mock_manager = MockInputManager::new();
        let manual_path = "/dev/custom/path";

        // Should NOT call list_gamepads when path is specified
        mock_manager.expect_list_gamepads().never();

        mock_manager.expect_open_gamepad().with(mockall::predicate::eq(manual_path)).returning(
            |_| {
                let mut mock_gamepad = MockGamepad::new();
                mock_gamepad.expect_read_event().returning(|| Ok(None));
                Ok(Box::new(mock_gamepad))
            },
        );

        let matches = command().get_matches_from(vec!["run", "--device", manual_path]);

        let result =
            run_internal(&matches, &mock_manager, |_| Ok(Box::new(MockVirtualKeyboard::new())));

        assert!(result.is_ok());
    }

    #[test]
    fn test_run_logic_event_processing() {
        use crate::event::{ButtonCode, InputEvent, KeyboardCode};

        let mut mock_manager = MockInputManager::new();
        let manual_path = "/dev/input/eventX";

        mock_manager.expect_open_gamepad().returning(move |_| {
            let mut mock_gamepad = MockGamepad::new();
            // Sequence of events: 1 press, then None to exit
            mock_gamepad
                .expect_read_event()
                .times(1)
                .returning(|| Ok(Some(InputEvent::button_press(ButtonCode::South))));
            mock_gamepad.expect_read_event().returning(|| Ok(None));
            Ok(Box::new(mock_gamepad))
        });

        let mut mock_keyboard = MockVirtualKeyboard::new();
        // The hardcoded engine maps ButtonCode::South to KeyboardCode::S
        mock_keyboard
            .expect_press_key()
            .with(mockall::predicate::eq(KeyboardCode::S))
            .times(1)
            .returning(|_| Ok(()));

        let matches = command().get_matches_from(vec!["run", "--device", manual_path]);

        let result = run_internal(&matches, &mock_manager, |_| Ok(Box::new(mock_keyboard)));

        assert!(result.is_ok());
    }
}
