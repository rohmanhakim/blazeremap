// Binary entry point for BlazeRemap
use blazeremap::app::App;
use std::process;

fn main() {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Run the app and exit with appropriate code
    process::exit(run());
}

/// Testable entry point that doesn't call process::exit
fn run() -> i32 {
    let app = App::new();

    match app.run() {
        Ok(_) => 0,
        Err(e) => {
            eprintln!("Error: {}", e);
            1
        }
    }
}
