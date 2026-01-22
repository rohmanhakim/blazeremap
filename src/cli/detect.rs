// Detect command - list connected controllers
use crate::platform;
use clap::{ArgMatches, Command};
use std::io::Write;

pub fn command() -> Command {
    Command::new("detect").about("Detect controllers connected to your computer").arg(
        clap::Arg::new("verbose")
            .short('v')
            .long("verbose")
            .help("Show detailed information")
            .action(clap::ArgAction::SetTrue),
    )
}

pub fn handle(matches: &ArgMatches) -> anyhow::Result<()> {
    let verbose = matches.get_flag("verbose");

    println!("Detecting controllers...\n");

    let device_manager = platform::new_device_manager();
    let result = device_manager.list_controllers()?;

    display_results(&result, verbose);

    Ok(())
}

/// Display detection results in a user-friendly format
fn display_results(result: &crate::device::DetectionResult, verbose: bool) {
    let mut output = std::io::stdout();
    write_results(&mut output, result, verbose).unwrap();
}

/// Internal function that writes to any writer (testable!)
fn write_results<W: Write>(
    writer: &mut W,
    result: &crate::device::DetectionResult,
    verbose: bool,
) -> std::io::Result<()> {
    use crate::device::controller::capabilities_to_strings;

    if result.controller_info.is_empty() {
        writeln!(writer, "No controllers found.")?;

        if !result.errors.is_empty() {
            writeln!(writer, "\nErrors encountered:")?;
            for error in &result.errors {
                writeln!(writer, "  • {}", error)?;
            }
        }

        return Ok(());
    }

    writeln!(writer, "Found {} controller(s):\n", result.controller_info.len())?;

    for (i, info) in result.controller_info.iter().enumerate() {
        writeln!(writer, "[{}] {} ({})", i, info.name, info.path)?;
        writeln!(writer, " ├─ Type: {}", info.controller_type)?;
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
        for (i, info) in result.controller_info.iter().enumerate() {
            writeln!(writer, "  [{}] Full path: {}", i, info.path)?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::device::{ControllerCapability, ControllerInfo, ControllerType, DetectionResult};

    /// Helper to create a test controller
    fn make_test_controller(name: &str) -> ControllerInfo {
        ControllerInfo {
            path: "/dev/input/event99".to_string(),
            name: name.to_string(),
            controller_type: ControllerType::DualShock4,
            vendor_id: 0x054C,
            vendor_name: "Sony".to_string(),
            product_id: 0x09CC,
            capabilities: vec![ControllerCapability::ForceFeedback],
        }
    }

    #[test]
    fn test_display_no_controllers() {
        let result = DetectionResult { controller_info: vec![], errors: vec![] };

        let mut output = Vec::new();
        write_results(&mut output, &result, false).unwrap();

        let text = String::from_utf8(output).unwrap();
        assert!(text.contains("No controllers found"));
    }

    #[test]
    fn test_display_one_controller() {
        let result = DetectionResult {
            controller_info: vec![make_test_controller("Test Controller")],
            errors: vec![],
        };

        let mut output = Vec::new();
        write_results(&mut output, &result, false).unwrap();

        let text = String::from_utf8(output).unwrap();

        // Check for expected content
        assert!(text.contains("Found 1 controller(s)"));
        assert!(text.contains("Test Controller"));
        assert!(text.contains("DualShock 4"));
        assert!(text.contains("Sony"));
        assert!(text.contains("054C"));
        assert!(text.contains("Force Feedback"));
    }

    #[test]
    fn test_display_multiple_controllers() {
        let result = DetectionResult {
            controller_info: vec![
                make_test_controller("Controller 1"),
                make_test_controller("Controller 2"),
            ],
            errors: vec![],
        };

        let mut output = Vec::new();
        write_results(&mut output, &result, false).unwrap();

        let text = String::from_utf8(output).unwrap();

        assert!(text.contains("Found 2 controller(s)"));
        assert!(text.contains("[0] Controller 1"));
        assert!(text.contains("[1] Controller 2"));
    }

    #[test]
    fn test_verbose_mode() {
        let result = DetectionResult {
            controller_info: vec![make_test_controller("Test Controller")],
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
            DetectionResult { controller_info: vec![make_test_controller("Test")], errors: vec![] };

        let mut output = Vec::new();
        write_results(&mut output, &result, false).unwrap();
        let text = String::from_utf8(output).unwrap();

        // Check for tree characters
        assert!(text.contains("├─"));
        assert!(text.contains("└─"));
        assert!(text.contains("│"));
    }
}
