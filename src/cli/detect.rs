// Detect command - list connected gamepads
use crate::platform;
use clap::{ArgMatches, Command};
use std::io::Write;

pub fn command() -> Command {
    Command::new("detect").about("Detect gamepads connected to your computer").arg(
        clap::Arg::new("verbose")
            .short('v')
            .long("verbose")
            .help("Show detailed information")
            .action(clap::ArgAction::SetTrue),
    )
}

pub fn handle(matches: &ArgMatches) -> anyhow::Result<()> {
    let verbose = matches.get_flag("verbose");

    println!("Detecting gamepads...\n");

    let device_manager = platform::new_device_manager();
    let result = device_manager.list_gamepads()?;

    display_results(&result, verbose);

    Ok(())
}

/// Display detection results in a user-friendly format
fn display_results(result: &crate::input::InputDetectionResult, verbose: bool) {
    let mut output = std::io::stdout();
    write_results(&mut output, result, verbose).unwrap();
}

/// Internal function that writes to any writer (testable!)
fn write_results<W: Write>(
    writer: &mut W,
    result: &crate::input::InputDetectionResult,
    verbose: bool,
) -> std::io::Result<()> {
    use crate::input::gamepad::capabilities_to_strings;

    if result.gamepad_info.is_empty() {
        writeln!(writer, "No gamepads found.")?;

        if !result.errors.is_empty() {
            writeln!(writer, "\nErrors encountered:")?;
            for error in &result.errors {
                writeln!(writer, "  • {}", error)?;
            }
        }

        return Ok(());
    }

    writeln!(writer, "Found {} gamepad(s):\n", result.gamepad_info.len())?;

    for (i, info) in result.gamepad_info.iter().enumerate() {
        writeln!(writer, "[{}] {} ({})", i, info.name, info.path)?;
        writeln!(writer, " ├─ Type: {}", info.gamepad_type)?;
        writeln!(writer, " ├─ Vendor:")?;
        writeln!(writer, " │  ├─ ID: {:04X}", info.vendor_id)?;
        writeln!(writer, " │  └─ Name: {}", info.vendor_name)?;
        writeln!(writer, " ├─ Product ID: {:04X}", info.product_id)?;
        writeln!(writer, " └─ Capabilities:")?;

        let caps = capabilities_to_strings(&info.capabilities);
        if caps.is_empty() {
            writeln!(writer, "    └─ None detected")?;
        } else {
            for (j, cap) in caps.iter().enumerate() {
                let prefix = if j == caps.len() - 1 { "    └─ " } else { "    ├─ " };
                writeln!(writer, "{}{}", prefix, cap)?;
            }
        }

        writeln!(writer)?;
    }

    if verbose {
        writeln!(writer, "Verbose Information:")?;
        for (i, info) in result.gamepad_info.iter().enumerate() {
            writeln!(writer, "  [{}] Full path: {}", i, info.path)?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::{GamepadCapability, GamepadInfo, GamepadType, InputDetectionResult};

    /// Helper to create a test gamepad
    fn make_test_gamepad(name: &str) -> GamepadInfo {
        GamepadInfo {
            path: "/dev/input/event99".to_string(),
            name: name.to_string(),
            gamepad_type: GamepadType::DualShock4,
            vendor_id: 0x054C,
            vendor_name: "Sony".to_string(),
            product_id: 0x09CC,
            capabilities: vec![GamepadCapability::ForceFeedback],
        }
    }

    #[test]
    fn test_display_no_gamepads() {
        let result = InputDetectionResult { gamepad_info: vec![], errors: vec![] };

        let mut output = Vec::new();
        write_results(&mut output, &result, false).unwrap();

        let text = String::from_utf8(output).unwrap();
        assert!(text.contains("No gamepads found"));
    }

    #[test]
    fn test_display_one_gamepad() {
        let result = InputDetectionResult {
            gamepad_info: vec![make_test_gamepad("Test Gamepad")],
            errors: vec![],
        };

        let mut output = Vec::new();
        write_results(&mut output, &result, false).unwrap();

        let text = String::from_utf8(output).unwrap();

        // Check for expected content
        assert!(text.contains("Found 1 gamepad(s)"));
        assert!(text.contains("Test Gamepad"));
        assert!(text.contains("DualShock 4"));
        assert!(text.contains("Sony"));
        assert!(text.contains("054C"));
        assert!(text.contains("Force Feedback"));
    }

    #[test]
    fn test_display_multiple_gamepads() {
        let result = InputDetectionResult {
            gamepad_info: vec![make_test_gamepad("Gamepad 1"), make_test_gamepad("Gamepad 2")],
            errors: vec![],
        };

        let mut output = Vec::new();
        write_results(&mut output, &result, false).unwrap();

        let text = String::from_utf8(output).unwrap();

        assert!(text.contains("Found 2 gamepad(s)"));
        assert!(text.contains("[0] Gamepad 1"));
        assert!(text.contains("[1] Gamepad 2"));
    }

    #[test]
    fn test_verbose_mode() {
        let result = InputDetectionResult {
            gamepad_info: vec![make_test_gamepad("Test Gamepad")],
            errors: vec![],
        };

        // Test without verbose
        let mut output = Vec::new();
        write_results(&mut output, &result, false).unwrap();
        let text = String::from_utf8(output).unwrap();
        assert!(!text.contains("Verbose Information"));

        // Test with verbose
        let mut output = Vec::new();
        write_results(&mut output, &result, true).unwrap();
        let text = String::from_utf8(output).unwrap();
        assert!(text.contains("Verbose Information"));
        assert!(text.contains("Full path: /dev/input/event99"));
    }

    #[test]
    fn test_tree_formatting() {
        let result =
            InputDetectionResult { gamepad_info: vec![make_test_gamepad("Test")], errors: vec![] };

        let mut output = Vec::new();
        write_results(&mut output, &result, false).unwrap();
        let text = String::from_utf8(output).unwrap();

        // Check for tree characters
        assert!(text.contains("├─"));
        assert!(text.contains("└─"));
        assert!(text.contains("│"));
    }
}
