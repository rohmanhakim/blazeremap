use crate::output::keyboard::VirtualKeyboard;
// src/cli/test_keyboard.rs
use crate::platform::linux::LinuxVirtualKeyboard;
use anyhow::Result;
use clap::Command;
use evdev::KeyCode;
use std::thread;
use std::time::Duration;

pub fn command() -> Command {
    Command::new("test-keyboard").about("Test virtual keyboard by emitting space key every second")
}

pub fn handle(_matches: &clap::ArgMatches) -> Result<()> {
    println!("Creating virtual keyboard...");
    let mut keyboard = LinuxVirtualKeyboard::new("BlazeRemap Test Keyboard")?;

    // Try to show sysfs path
    match keyboard.sys_path() {
        Ok(path) => {
            println!("Virtual device sysfs path: {:?}", path);
            println!("Device node will be in /dev/input/ (use 'evtest' to find it)");
        }
        Err(e) => {
            println!("Note: Could not get sysfs path: {}", e);
        }
    }

    println!("\nEmitting space key every second...");
    println!("Open a text editor to see the output.");
    println!("Press Ctrl+C to stop.\n");

    // Emit space key in a loop
    for i in 1.. {
        keyboard.tap_key(KeyCode::KEY_SPACE.code())?;
        println!("[{}] Space key tapped", i);
        thread::sleep(Duration::from_secs(1));
    }

    Ok(())
}
