// CLI module - command definitions and handling
mod detect;
mod read;

use clap::Command;

/// Build the root CLI command structure
pub fn build_cli() -> Command {
    Command::new("blazeremap")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Muhammad Arif Rohman Hakim")
        .about("Linux keyboard-to-gamepad remapping software")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(detect::command())
        .subcommand(read::command())
}

/// Execute the CLI and handle the result
pub fn execute() -> anyhow::Result<()> {
    let matches = build_cli().get_matches();

    match matches.subcommand() {
        Some(("detect", sub_matches)) => detect::handle(sub_matches),
        Some(("read", sub_matches)) => read::handle(sub_matches),
        _ => unreachable!("Subcommand required"),
    }
}
