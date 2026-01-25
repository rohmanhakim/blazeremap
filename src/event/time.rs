#[cfg(target_os = "linux")]
use std::sync::OnceLock;
use std::time::{Instant, SystemTime};

/// Global time anchor for converting SystemTime to Instant
static TIME_ANCHOR: OnceLock<TimeAnchor> = OnceLock::new();

#[derive(Clone, Copy)]
struct TimeAnchor {
    system_time: SystemTime,
    instant: Instant,
}

impl TimeAnchor {
    fn new() -> Self {
        Self { system_time: SystemTime::now(), instant: Instant::now() }
    }

    /// Convert SystemTime to Instant using this anchor
    fn to_instant(self, system_time: SystemTime) -> Instant {
        match system_time.duration_since(self.system_time) {
            Ok(duration) => self.instant + duration,
            Err(err) => self.instant - err.duration(),
        }
    }
}

/// Initialize the global time anchor (call once at startup)
///
/// This should be called early in main() before any event processing.
/// It establishes a fixed point for converting platform-specific timestamps
/// to monotonic Instant values.
pub fn init_time_anchor() {
    TIME_ANCHOR.get_or_init(TimeAnchor::new);
}

/// Convert a SystemTime to Instant (internal helper)
pub(crate) fn system_time_to_instant(system_time: SystemTime) -> Instant {
    let anchor = TIME_ANCHOR.get_or_init(TimeAnchor::new);
    anchor.to_instant(system_time)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_time_anchor_conversion() {
        let anchor_sys = SystemTime::now();
        let anchor_inst = Instant::now();
        let anchor = TimeAnchor { system_time: anchor_sys, instant: anchor_inst };

        // Test future time (relative duration should be preserved)
        let future_sys = anchor_sys + Duration::from_millis(100);
        let future_inst = anchor.to_instant(future_sys);
        assert!(future_inst > anchor_inst);
        assert_eq!(future_inst.duration_since(anchor_inst), Duration::from_millis(100));

        // Test past time (handling negative deltas correctly)
        let past_sys = anchor_sys - Duration::from_millis(50);
        let past_inst = anchor.to_instant(past_sys);
        assert!(past_inst < anchor_inst);
        assert_eq!(anchor_inst.duration_since(past_inst), Duration::from_millis(50));

        // Test exact anchor time
        assert_eq!(anchor.to_instant(anchor_sys), anchor_inst);
    }

    #[test]
    fn test_global_system_time_to_instant() {
        // Ensure anchor is initialized
        init_time_anchor();

        let now_sys = SystemTime::now();
        let inst1 = system_time_to_instant(now_sys);

        // Wait a bit to simulate event gap
        std::thread::sleep(Duration::from_millis(10));

        let later_sys = SystemTime::now();
        let inst2 = system_time_to_instant(later_sys);

        // The duration between Instants should match the duration between SystemTimes
        let sys_duration = later_sys.duration_since(now_sys).unwrap_or(Duration::ZERO);
        let inst_duration = inst2.saturating_duration_since(inst1);

        // Allow for tiny discrepancies (under 500µs) due to clock precision/jitter under load
        let diff = if inst_duration > sys_duration {
            inst_duration - sys_duration
        } else {
            sys_duration - inst_duration
        };

        assert!(
            diff < Duration::from_micros(500),
            "Duration mismatch: expected {:?}, got {:?} (diff: {:?})",
            sys_duration,
            inst_duration,
            diff
        );
    }
}
