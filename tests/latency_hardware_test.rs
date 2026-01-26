use std::time::{Duration, Instant};

use blazeremap::mapping::MappingEngine;

#[test]
#[ignore]
fn test_real_hardware_latency() {
    print_header("Input Latency Measurement (Processing Only)");

    wait_for_user("Connect your controller and prepare to rapidly press buttons");

    let manager = blazeremap::platform::new_input_manager();
    let gamepads = manager.list_gamepads().unwrap();
    assert!(!gamepads.gamepad_info.is_empty());

    println!("\n📱 Using controller: {}", gamepads.gamepad_info[0].name);

    let mut controller = manager.open_gamepad(&gamepads.gamepad_info[0].path).unwrap();
    let mut keyboard =
        blazeremap::platform::new_virtual_keyboard("BlazeRemap Latency Test").unwrap();
    let mut engine = MappingEngine::new_hardcoded();

    println!("\n👉 Rapidly press buttons for 10 seconds...");
    println!("   (Measuring PROCESSING latency only)\n");

    std::thread::sleep(Duration::from_secs(2));

    let test_start = Instant::now();
    let mut latencies = Vec::new();
    let mut event_count = 0;

    while test_start.elapsed() < Duration::from_secs(10) {
        // Don't time the blocking read_event() call
        match controller.read_event() {
            Ok(Some(event)) => {
                // Start timing ONLY processing
                let process_start = Instant::now();

                // Process through mapping
                let outputs = engine.process(&event).unwrap();

                // Emit to keyboard
                for output in outputs {
                    match output {
                        blazeremap::event::OutputEvent::Keyboard { code, event_type } => {
                            use blazeremap::event::KeyboardEventType;
                            match event_type {
                                KeyboardEventType::Press => keyboard.press_key(code).ok(),
                                KeyboardEventType::Release => keyboard.release_key(code).ok(),
                                _ => Some(()),
                            };
                        }
                    }
                }

                // Measure ONLY processing latency
                let latency = process_start.elapsed();
                latencies.push(latency);
                event_count += 1;
            }
            Ok(None) => break,
            Err(_) => continue,
        }
    }

    assert!(!latencies.is_empty(), "❌ No events received - please press buttons!");

    let total: Duration = latencies.iter().sum();
    let avg = total / latencies.len() as u32;
    let max = *latencies.iter().max().unwrap();
    let min = *latencies.iter().min().unwrap();

    // Calculate percentiles
    let mut sorted = latencies.clone();
    sorted.sort();
    let p95 = sorted[(sorted.len() as f32 * 0.95) as usize];
    let p99 = sorted[(sorted.len() as f32 * 0.99) as usize];

    println!("\n📊 Processing Latency Statistics:");
    println!("   Events processed: {}", event_count);
    println!("   Average latency:  {:?} ({:.3}ms)", avg, avg.as_secs_f64() * 1000.0);
    println!("   Min latency:      {:?} ({:.3}ms)", min, min.as_secs_f64() * 1000.0);
    println!("   Max latency:      {:?} ({:.3}ms)", max, max.as_secs_f64() * 1000.0);
    println!("   95th percentile:  {:?} ({:.3}ms)", p95, p95.as_secs_f64() * 1000.0);
    println!("   99th percentile:  {:?} ({:.3}ms)", p99, p99.as_secs_f64() * 1000.0);

    println!("\n📋 Phase 2 Requirements:");
    println!("   Target: <1ms average (processing only)");

    if avg.as_micros() < 1000 {
        println!("   ✅ EXCELLENT: Processing latency <1ms ({}µs)", avg.as_micros());
    } else if avg.as_millis() < 16 {
        println!("   ✅ GOOD: Processing latency <16ms");
    } else {
        println!("   ⚠️  WARNING: Processing latency >16ms");
    }

    // Update assertion to 1ms target
    assert!(
        avg.as_millis() < 1,
        "❌ FAILED: Processing latency {}µs exceeds 1ms target",
        avg.as_micros()
    );

    println!("\n✅ PASSED: Latency meets requirements");
}

/// Helper to print test header
fn print_header(test_name: &str) {
    println!("\n╔═══════════════════════════════════════════════════════════════╗");
    println!("║  {:^60} ║", test_name);
    println!("╚═══════════════════════════════════════════════════════════════╝");
}

/// Helper to wait for user input
fn wait_for_user(prompt: &str) {
    use std::io::{self, Write};
    print!("\n{}\nPress Enter to continue...", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
}
