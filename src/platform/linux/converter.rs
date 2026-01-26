/*
Conversion utilities for translating evdev events to domain events.

 This module provides converters that transform platform-specific event types
 (from the Linux evdev subsystem) into BlazeRemap's platform-agnostic event types.

 # Purpose
 The converter acts as a boundary layer between the Linux input subsystem and
 BlazeRemap's core logic, allowing the rest of the application to work with
 clean, domain-specific types regardless of the underlying platform.

 # Event Mapping
 | evdev EventType      | BlazeRemap EventType | Description          |
 |---------------------|----------------------|-----------------------|
 | `KEY`               | `Button`             | Gamepad buttons    |
 | `ABSOLUTE`          | `Axis`               | Analog sticks/triggers|
 | `SYNCHRONIZATION`   | `Sync`               | Frame boundaries      |
 | `SWITCH`            | `DPad`               | Directional pad       |
 | Others              | `None`               | Filtered out          |

 # Timestamp Handling
 The converter discards evdev's kernel timestamp and captures a new [`Instant`]
 when the event is converted. This timestamp represents when BlazeRemap's userspace
 code received the event, which is the starting point for latency measurement.

 # Note
 Unsupported evdev event types (LED, SOUND, etc.) are filtered out and return
 `None`, as they are not relevant for gamepad input remapping.
*/

use crate::event::{AxisCode, ButtonCode, InputEvent, KeyboardCode, system_time_to_instant};

pub fn evdev_to_input(ev: evdev::InputEvent) -> Option<InputEvent> {
    //  Convert kernel's SystemTime to Instant (preserves timing)
    let timestamp = system_time_to_instant(ev.timestamp());

    match ev.destructure() {
        evdev::EventSummary::Key(_, key_code, _value) => {
            let button_code = key_to_button_code(key_code);
            let pressed = _value > 0;
            Some(InputEvent::Button { code: button_code, pressed, timestamp })
        }
        evdev::EventSummary::AbsoluteAxis(_, axis_code, value) => {
            let axis_code = absolute_axis_to_axis_code(axis_code);
            Some(InputEvent::Axis { code: axis_code, value, timestamp })
        }
        evdev::EventSummary::Switch(_, _switch_code, _value) => {
            // DPad events are typically handled as axes (ABS_HAT0X/Y) rather than switches
            // For now, we'll skip switch events as they're not commonly used for gamepads
            None
        }
        evdev::EventSummary::Synchronization(_, _, _) => Some(InputEvent::Sync { timestamp }),
        _ => None,
    }
}

fn key_to_button_code(key: evdev::KeyCode) -> ButtonCode {
    match key {
        evdev::KeyCode::BTN_SOUTH => ButtonCode::South,
        evdev::KeyCode::BTN_EAST => ButtonCode::East,
        evdev::KeyCode::BTN_NORTH => ButtonCode::North,
        evdev::KeyCode::BTN_WEST => ButtonCode::West,
        evdev::KeyCode::BTN_TL => ButtonCode::LeftShoulder,
        evdev::KeyCode::BTN_TR => ButtonCode::RightShoulder,
        evdev::KeyCode::BTN_TL2 => ButtonCode::LeftTrigger,
        evdev::KeyCode::BTN_TR2 => ButtonCode::RightTrigger,
        evdev::KeyCode::BTN_SELECT => ButtonCode::Select,
        evdev::KeyCode::BTN_START => ButtonCode::Start,
        evdev::KeyCode::BTN_MODE => ButtonCode::Mode,
        evdev::KeyCode::BTN_THUMBL => ButtonCode::LeftStick,
        evdev::KeyCode::BTN_THUMBR => ButtonCode::RightStick,
        evdev::KeyCode::BTN_TRIGGER_HAPPY1 => ButtonCode::Paddle1,
        evdev::KeyCode::BTN_TRIGGER_HAPPY2 => ButtonCode::Paddle2,
        evdev::KeyCode::BTN_TRIGGER_HAPPY3 => ButtonCode::Paddle3,
        evdev::KeyCode::BTN_TRIGGER_HAPPY4 => ButtonCode::Paddle4,
        _ => ButtonCode::Unknown,
    }
}

pub fn keyboard_code_to_evdev_key(code: KeyboardCode) -> evdev::KeyCode {
    match code {
        KeyboardCode::Reserved => evdev::KeyCode::KEY_RESERVED,
        KeyboardCode::Escape => evdev::KeyCode::KEY_ESC,
        KeyboardCode::Num1 => evdev::KeyCode::KEY_1,
        KeyboardCode::Num2 => evdev::KeyCode::KEY_2,
        KeyboardCode::Num3 => evdev::KeyCode::KEY_3,
        KeyboardCode::Num4 => evdev::KeyCode::KEY_4,
        KeyboardCode::Num5 => evdev::KeyCode::KEY_5,
        KeyboardCode::Num6 => evdev::KeyCode::KEY_6,
        KeyboardCode::Num7 => evdev::KeyCode::KEY_7,
        KeyboardCode::Num8 => evdev::KeyCode::KEY_8,
        KeyboardCode::Num9 => evdev::KeyCode::KEY_9,
        KeyboardCode::Num0 => evdev::KeyCode::KEY_0,
        KeyboardCode::Minus => evdev::KeyCode::KEY_MINUS,
        KeyboardCode::Equal => evdev::KeyCode::KEY_EQUAL,
        KeyboardCode::Backspace => evdev::KeyCode::KEY_BACKSPACE,
        KeyboardCode::Tab => evdev::KeyCode::KEY_TAB,
        KeyboardCode::Q => evdev::KeyCode::KEY_Q,
        KeyboardCode::W => evdev::KeyCode::KEY_W,
        KeyboardCode::E => evdev::KeyCode::KEY_E,
        KeyboardCode::R => evdev::KeyCode::KEY_R,
        KeyboardCode::T => evdev::KeyCode::KEY_T,
        KeyboardCode::Y => evdev::KeyCode::KEY_Y,
        KeyboardCode::U => evdev::KeyCode::KEY_U,
        KeyboardCode::I => evdev::KeyCode::KEY_I,
        KeyboardCode::O => evdev::KeyCode::KEY_O,
        KeyboardCode::P => evdev::KeyCode::KEY_P,
        KeyboardCode::LeftBrace => evdev::KeyCode::KEY_LEFTBRACE,
        KeyboardCode::RightBrace => evdev::KeyCode::KEY_RIGHTBRACE,
        KeyboardCode::Enter => evdev::KeyCode::KEY_ENTER,
        KeyboardCode::LeftControl => evdev::KeyCode::KEY_LEFTCTRL,
        KeyboardCode::A => evdev::KeyCode::KEY_A,
        KeyboardCode::S => evdev::KeyCode::KEY_S,
        KeyboardCode::D => evdev::KeyCode::KEY_D,
        KeyboardCode::F => evdev::KeyCode::KEY_F,
        KeyboardCode::G => evdev::KeyCode::KEY_G,
        KeyboardCode::H => evdev::KeyCode::KEY_H,
        KeyboardCode::J => evdev::KeyCode::KEY_J,
        KeyboardCode::K => evdev::KeyCode::KEY_K,
        KeyboardCode::L => evdev::KeyCode::KEY_L,
        KeyboardCode::Semicolon => evdev::KeyCode::KEY_SEMICOLON,
        KeyboardCode::Apostrophe => evdev::KeyCode::KEY_APOSTROPHE,
        KeyboardCode::Grave => evdev::KeyCode::KEY_GRAVE,
        KeyboardCode::LeftShift => evdev::KeyCode::KEY_LEFTSHIFT,
        KeyboardCode::Backslash => evdev::KeyCode::KEY_BACKSLASH,
        KeyboardCode::Z => evdev::KeyCode::KEY_Z,
        KeyboardCode::X => evdev::KeyCode::KEY_X,
        KeyboardCode::C => evdev::KeyCode::KEY_C,
        KeyboardCode::V => evdev::KeyCode::KEY_V,
        KeyboardCode::B => evdev::KeyCode::KEY_B,
        KeyboardCode::N => evdev::KeyCode::KEY_N,
        KeyboardCode::M => evdev::KeyCode::KEY_M,
        KeyboardCode::Comma => evdev::KeyCode::KEY_COMMA,
        KeyboardCode::Dot => evdev::KeyCode::KEY_DOT,
        KeyboardCode::Slash => evdev::KeyCode::KEY_SLASH,
        KeyboardCode::RightShift => evdev::KeyCode::KEY_RIGHTSHIFT,
        KeyboardCode::KpAsterisk => evdev::KeyCode::KEY_KPASTERISK,
        KeyboardCode::LeftAlt => evdev::KeyCode::KEY_LEFTALT,
        KeyboardCode::Space => evdev::KeyCode::KEY_SPACE,
        KeyboardCode::CapsLock => evdev::KeyCode::KEY_CAPSLOCK,
        KeyboardCode::F1 => evdev::KeyCode::KEY_F1,
        KeyboardCode::F2 => evdev::KeyCode::KEY_F2,
        KeyboardCode::F3 => evdev::KeyCode::KEY_F3,
        KeyboardCode::F4 => evdev::KeyCode::KEY_F4,
        KeyboardCode::F5 => evdev::KeyCode::KEY_F5,
        KeyboardCode::F6 => evdev::KeyCode::KEY_F6,
        KeyboardCode::F7 => evdev::KeyCode::KEY_F7,
        KeyboardCode::F8 => evdev::KeyCode::KEY_F8,
        KeyboardCode::F9 => evdev::KeyCode::KEY_F9,
        KeyboardCode::F10 => evdev::KeyCode::KEY_F10,
        KeyboardCode::NumLock => evdev::KeyCode::KEY_NUMLOCK,
        KeyboardCode::ScrollLock => evdev::KeyCode::KEY_SCROLLLOCK,
        KeyboardCode::Kp7 => evdev::KeyCode::KEY_KP7,
        KeyboardCode::Kp8 => evdev::KeyCode::KEY_KP8,
        KeyboardCode::Kp9 => evdev::KeyCode::KEY_KP9,
        KeyboardCode::KpMinus => evdev::KeyCode::KEY_KPMINUS,
        KeyboardCode::Kp4 => evdev::KeyCode::KEY_KP4,
        KeyboardCode::Kp5 => evdev::KeyCode::KEY_KP5,
        KeyboardCode::Kp6 => evdev::KeyCode::KEY_KP6,
        KeyboardCode::KpPlus => evdev::KeyCode::KEY_KPPLUS,
        KeyboardCode::Kp1 => evdev::KeyCode::KEY_KP1,
        KeyboardCode::Kp2 => evdev::KeyCode::KEY_KP2,
        KeyboardCode::Kp3 => evdev::KeyCode::KEY_KP3,
        KeyboardCode::Kp0 => evdev::KeyCode::KEY_KP0,
        KeyboardCode::KpDot => evdev::KeyCode::KEY_KPDOT,
        KeyboardCode::KpEnter => evdev::KeyCode::KEY_KPENTER,
        KeyboardCode::RightControl => evdev::KeyCode::KEY_RIGHTCTRL,
        KeyboardCode::KpSlash => evdev::KeyCode::KEY_KPSLASH,
        KeyboardCode::SysRq => evdev::KeyCode::KEY_SYSRQ,
        KeyboardCode::RightAlt => evdev::KeyCode::KEY_RIGHTALT,
        KeyboardCode::LineFeed => evdev::KeyCode::KEY_LINEFEED,
        KeyboardCode::Home => evdev::KeyCode::KEY_HOME,
        KeyboardCode::Up => evdev::KeyCode::KEY_UP,
        KeyboardCode::PageUp => evdev::KeyCode::KEY_PAGEUP,
        KeyboardCode::Left => evdev::KeyCode::KEY_LEFT,
        KeyboardCode::Right => evdev::KeyCode::KEY_RIGHT,
        KeyboardCode::End => evdev::KeyCode::KEY_END,
        KeyboardCode::Down => evdev::KeyCode::KEY_DOWN,
        KeyboardCode::PageDown => evdev::KeyCode::KEY_PAGEDOWN,
        KeyboardCode::Insert => evdev::KeyCode::KEY_INSERT,
        KeyboardCode::Delete => evdev::KeyCode::KEY_DELETE,
        KeyboardCode::Macro => evdev::KeyCode::KEY_MACRO,
        KeyboardCode::Mute => evdev::KeyCode::KEY_MUTE,
        KeyboardCode::VolumeDown => evdev::KeyCode::KEY_VOLUMEDOWN,
        KeyboardCode::VolumeUp => evdev::KeyCode::KEY_VOLUMEUP,
        KeyboardCode::Power => evdev::KeyCode::KEY_POWER,
        KeyboardCode::KpEqual => evdev::KeyCode::KEY_KPEQUAL,
        KeyboardCode::KpPlusMinus => evdev::KeyCode::KEY_KPPLUSMINUS,
        KeyboardCode::Pause => evdev::KeyCode::KEY_PAUSE,
        KeyboardCode::Scale => evdev::KeyCode::KEY_SCALE,
        KeyboardCode::KpComma => evdev::KeyCode::KEY_KPCOMMA,
        KeyboardCode::LeftMeta => evdev::KeyCode::KEY_LEFTMETA,
        KeyboardCode::RightMeta => evdev::KeyCode::KEY_RIGHTMETA,
        KeyboardCode::Compose => evdev::KeyCode::KEY_COMPOSE,
        KeyboardCode::Stop => evdev::KeyCode::KEY_STOP,
        KeyboardCode::Again => evdev::KeyCode::KEY_AGAIN,
        KeyboardCode::Props => evdev::KeyCode::KEY_PROPS,
        KeyboardCode::Undo => evdev::KeyCode::KEY_UNDO,
        KeyboardCode::Front => evdev::KeyCode::KEY_FRONT,
        KeyboardCode::Copy => evdev::KeyCode::KEY_COPY,
        KeyboardCode::Open => evdev::KeyCode::KEY_OPEN,
        KeyboardCode::Paste => evdev::KeyCode::KEY_PASTE,
        KeyboardCode::Find => evdev::KeyCode::KEY_FIND,
        KeyboardCode::Cut => evdev::KeyCode::KEY_CUT,
        KeyboardCode::Help => evdev::KeyCode::KEY_HELP,
        KeyboardCode::Menu => evdev::KeyCode::KEY_MENU,
        KeyboardCode::Calc => evdev::KeyCode::KEY_CALC,
        KeyboardCode::Setup => evdev::KeyCode::KEY_SETUP,
        KeyboardCode::Sleep => evdev::KeyCode::KEY_SLEEP,
        KeyboardCode::WakeUp => evdev::KeyCode::KEY_WAKEUP,
        KeyboardCode::File => evdev::KeyCode::KEY_FILE,
        KeyboardCode::SendFile => evdev::KeyCode::KEY_SENDFILE,
        KeyboardCode::DeleteFile => evdev::KeyCode::KEY_DELETEFILE,
        KeyboardCode::Xfer => evdev::KeyCode::KEY_XFER,
        KeyboardCode::Prog1 => evdev::KeyCode::KEY_PROG1,
        KeyboardCode::Prog2 => evdev::KeyCode::KEY_PROG2,
        KeyboardCode::Www => evdev::KeyCode::KEY_WWW,
        KeyboardCode::Msdos => evdev::KeyCode::KEY_MSDOS,
        KeyboardCode::Coffee => evdev::KeyCode::KEY_COFFEE,
        KeyboardCode::Direction => evdev::KeyCode::KEY_DIRECTION,
        KeyboardCode::RotateDisplay => evdev::KeyCode::KEY_DIRECTION,
        KeyboardCode::CycleWindows => evdev::KeyCode::KEY_CYCLEWINDOWS,
        KeyboardCode::Mail => evdev::KeyCode::KEY_MAIL,
        KeyboardCode::Bookmarks => evdev::KeyCode::KEY_BOOKMARKS,
        KeyboardCode::Computer => evdev::KeyCode::KEY_COMPUTER,
        KeyboardCode::Back => evdev::KeyCode::KEY_BACK,
        KeyboardCode::Forward => evdev::KeyCode::KEY_FORWARD,
        KeyboardCode::CloseCd => evdev::KeyCode::KEY_CLOSECD,
        KeyboardCode::EjectCd => evdev::KeyCode::KEY_EJECTCD,
        KeyboardCode::EjectCloseCd => evdev::KeyCode::KEY_EJECTCLOSECD,
        KeyboardCode::NextSong => evdev::KeyCode::KEY_NEXTSONG,
        KeyboardCode::PlayPause => evdev::KeyCode::KEY_PLAYPAUSE,
        KeyboardCode::PreviousSong => evdev::KeyCode::KEY_PREVIOUSSONG,
        KeyboardCode::StopCd => evdev::KeyCode::KEY_STOPCD,
        KeyboardCode::Record => evdev::KeyCode::KEY_RECORD,
        KeyboardCode::Rewind => evdev::KeyCode::KEY_REWIND,
        KeyboardCode::Phone => evdev::KeyCode::KEY_PHONE,
        KeyboardCode::Iso => evdev::KeyCode::KEY_ISO,
        KeyboardCode::Config => evdev::KeyCode::KEY_CONFIG,
        KeyboardCode::HomePage => evdev::KeyCode::KEY_HOMEPAGE,
        KeyboardCode::Refresh => evdev::KeyCode::KEY_REFRESH,
        KeyboardCode::Exit => evdev::KeyCode::KEY_EXIT,
        KeyboardCode::Move => evdev::KeyCode::KEY_MOVE,
        KeyboardCode::Edit => evdev::KeyCode::KEY_EDIT,
        KeyboardCode::ScrollUp => evdev::KeyCode::KEY_SCROLLUP,
        KeyboardCode::ScrollDown => evdev::KeyCode::KEY_SCROLLDOWN,
        KeyboardCode::KpLeftParen => evdev::KeyCode::KEY_KPLEFTPAREN,
        KeyboardCode::KpRightParen => evdev::KeyCode::KEY_KPRIGHTPAREN,
        KeyboardCode::New => evdev::KeyCode::KEY_NEW,
        KeyboardCode::Redo => evdev::KeyCode::KEY_REDO,
        KeyboardCode::F13 => evdev::KeyCode::KEY_F13,
        KeyboardCode::F14 => evdev::KeyCode::KEY_F14,
        KeyboardCode::F15 => evdev::KeyCode::KEY_F15,
        KeyboardCode::F16 => evdev::KeyCode::KEY_F16,
        KeyboardCode::F17 => evdev::KeyCode::KEY_F17,
        KeyboardCode::F18 => evdev::KeyCode::KEY_F18,
        KeyboardCode::F19 => evdev::KeyCode::KEY_F19,
        KeyboardCode::F20 => evdev::KeyCode::KEY_F20,
        KeyboardCode::F21 => evdev::KeyCode::KEY_F21,
        KeyboardCode::F22 => evdev::KeyCode::KEY_F22,
        KeyboardCode::F23 => evdev::KeyCode::KEY_F23,
        KeyboardCode::F24 => evdev::KeyCode::KEY_F24,
        KeyboardCode::PlayCd => evdev::KeyCode::KEY_PLAYCD,
        KeyboardCode::PauseCd => evdev::KeyCode::KEY_PAUSECD,
        KeyboardCode::Prog3 => evdev::KeyCode::KEY_PROG3,
        KeyboardCode::Prog4 => evdev::KeyCode::KEY_PROG4,
        KeyboardCode::Dashboard => evdev::KeyCode::KEY_DASHBOARD,
        KeyboardCode::Suspend => evdev::KeyCode::KEY_SUSPEND,
        KeyboardCode::Close => evdev::KeyCode::KEY_CLOSE,
        KeyboardCode::Play => evdev::KeyCode::KEY_PLAY,
        KeyboardCode::FastForward => evdev::KeyCode::KEY_FASTFORWARD,
        KeyboardCode::BassBoost => evdev::KeyCode::KEY_BASSBOOST,
        KeyboardCode::Print => evdev::KeyCode::KEY_PRINT,
        KeyboardCode::Hp => evdev::KeyCode::KEY_HP,
        KeyboardCode::Camera => evdev::KeyCode::KEY_CAMERA,
        KeyboardCode::Sound => evdev::KeyCode::KEY_SOUND,
        KeyboardCode::Question => evdev::KeyCode::KEY_QUESTION,
        KeyboardCode::Email => evdev::KeyCode::KEY_EMAIL,
        KeyboardCode::Chat => evdev::KeyCode::KEY_CHAT,
        KeyboardCode::Search => evdev::KeyCode::KEY_SEARCH,
        KeyboardCode::Connect => evdev::KeyCode::KEY_CONNECT,
        KeyboardCode::Finance => evdev::KeyCode::KEY_FINANCE,
        KeyboardCode::Sport => evdev::KeyCode::KEY_SPORT,
        KeyboardCode::Shop => evdev::KeyCode::KEY_SHOP,
        KeyboardCode::AlterErase => evdev::KeyCode::KEY_ALTERASE,
        KeyboardCode::Cancel => evdev::KeyCode::KEY_CANCEL,
        KeyboardCode::BrightnessDown => evdev::KeyCode::KEY_BRIGHTNESSDOWN,
        KeyboardCode::BrightnessUp => evdev::KeyCode::KEY_BRIGHTNESSUP,
        KeyboardCode::Media => evdev::KeyCode::KEY_MEDIA,
        KeyboardCode::SwitchVideoMode => evdev::KeyCode::KEY_SWITCHVIDEOMODE,
        KeyboardCode::KbdIllumToggle => evdev::KeyCode::KEY_KBDILLUMTOGGLE,
        KeyboardCode::KbdIllumDown => evdev::KeyCode::KEY_KBDILLUMDOWN,
        KeyboardCode::KbdIllumUp => evdev::KeyCode::KEY_KBDILLUMUP,
        KeyboardCode::Send => evdev::KeyCode::KEY_SEND,
        KeyboardCode::Reply => evdev::KeyCode::KEY_REPLY,
        KeyboardCode::ForwardMail => evdev::KeyCode::KEY_FORWARDMAIL,
        KeyboardCode::Save => evdev::KeyCode::KEY_SAVE,
        KeyboardCode::Documents => evdev::KeyCode::KEY_DOCUMENTS,
        KeyboardCode::Battery => evdev::KeyCode::KEY_BATTERY,
        KeyboardCode::Bluetooth => evdev::KeyCode::KEY_BLUETOOTH,
        KeyboardCode::Wlan => evdev::KeyCode::KEY_WLAN,
        KeyboardCode::Uwb => evdev::KeyCode::KEY_UWB,
        KeyboardCode::Unknown => evdev::KeyCode::KEY_RESERVED,
    }
}

fn absolute_axis_to_axis_code(axis: evdev::AbsoluteAxisCode) -> AxisCode {
    match axis {
        evdev::AbsoluteAxisCode::ABS_X => AxisCode::LeftX,
        evdev::AbsoluteAxisCode::ABS_Y => AxisCode::LeftY,
        evdev::AbsoluteAxisCode::ABS_RX => AxisCode::RightX,
        evdev::AbsoluteAxisCode::ABS_RY => AxisCode::RightY,
        evdev::AbsoluteAxisCode::ABS_Z => AxisCode::LeftTrigger,
        evdev::AbsoluteAxisCode::ABS_RZ => AxisCode::RightTrigger,
        evdev::AbsoluteAxisCode::ABS_HAT0X => AxisCode::DPadX,
        evdev::AbsoluteAxisCode::ABS_HAT0Y => AxisCode::DPadY,
        _ => AxisCode::Unknown,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use evdev::InputEvent as EvdevEvent;
    use std::time::Duration;

    #[test]
    fn test_evdev_key_to_button() {
        let evdev_event = EvdevEvent::new(evdev::EventType::KEY.0, 0x130, 1);
        let result = evdev_to_input(evdev_event);
        assert!(result.is_some());
        let event = result.unwrap();
        assert!(matches!(event, InputEvent::Button { code: ButtonCode::South, pressed: true, .. }));
    }

    #[test]
    fn test_evdev_abs_to_axis() {
        use crate::event::init_time_anchor;
        init_time_anchor();

        let evdev_event = EvdevEvent::new_now(evdev::EventType::ABSOLUTE.0, 0x00, 15234);
        let result = evdev_to_input(evdev_event);
        assert!(result.is_some());
        let event = result.unwrap();
        assert!(matches!(event, InputEvent::Axis { code: AxisCode::LeftX, value: 15234, .. }));
    }

    #[test]
    fn test_evdev_sync_returns_sync() {
        let evdev_event = EvdevEvent::new(evdev::EventType::SYNCHRONIZATION.0, 0, 0);
        let result = evdev_to_input(evdev_event);
        assert!(result.is_some());
        let event = result.unwrap();
        assert!(matches!(event, InputEvent::Sync { .. }));
    }

    #[test]
    fn test_unsupported_event_type_returns_none() {
        let evdev_event = EvdevEvent::new(evdev::EventType::LED.0, 0, 1);
        let result = evdev_to_input(evdev_event);
        assert!(result.is_none());
    }

    #[test]
    fn test_timestamp_is_recent() {
        use crate::event::init_time_anchor;

        init_time_anchor();

        let evdev_event = EvdevEvent::new_now(evdev::EventType::KEY.0, 0x130, 1);
        let event = evdev_to_input(evdev_event).unwrap();

        let age = event.timestamp().elapsed();
        assert!(age < Duration::from_secs(1), "Event timestamp is too old: {:?}", age);
    }

    #[test]
    fn test_timestamps_are_monotonic() {
        use crate::event::init_time_anchor;

        init_time_anchor();

        // Create two events in sequence
        let event1 =
            evdev_to_input(EvdevEvent::new_now(evdev::EventType::KEY.0, 0x130, 1)).unwrap();

        // Small delay to ensure time advances
        std::thread::sleep(Duration::from_millis(10));

        let event2 =
            evdev_to_input(EvdevEvent::new_now(evdev::EventType::KEY.0, 0x131, 1)).unwrap();

        assert!(
            event2.timestamp() >= event1.timestamp(),
            "Timestamps not monotonic: event2 {:?} < event1 {:?}",
            event2.timestamp(),
            event1.timestamp()
        );
    }

    #[test]
    fn test_elapsed_is_non_negative() {
        let evdev_event = EvdevEvent::new(evdev::EventType::KEY.0, 0x130, 1);
        let event = evdev_to_input(evdev_event).unwrap();

        // Elapsed time is always >= 0 (Instant is monotonic)
        assert!(event.timestamp().elapsed() >= Duration::ZERO);
    }

    #[test]
    fn test_duration_preservation() {
        use crate::event::init_time_anchor;

        init_time_anchor();

        let event1 =
            evdev_to_input(EvdevEvent::new_now(evdev::EventType::KEY.0, 0x130, 1)).unwrap();

        // Known delay
        std::thread::sleep(Duration::from_millis(50));

        let event2 =
            evdev_to_input(EvdevEvent::new_now(evdev::EventType::KEY.0, 0x131, 1)).unwrap();

        // ✅ Delta should be approximately 50ms
        let delta = event2.timestamp().duration_since(event1.timestamp());
        assert!(delta >= Duration::from_millis(50), "Delta too small: {:?}", delta);
        assert!(delta < Duration::from_millis(100), "Delta too large: {:?}", delta);
    }

    #[test]
    fn test_all_button_code_mappings() {
        // Test a few key button mappings to ensure they work
        assert_eq!(key_to_button_code(evdev::KeyCode::BTN_SOUTH), ButtonCode::South);
        assert_eq!(key_to_button_code(evdev::KeyCode::BTN_EAST), ButtonCode::East);
        assert_eq!(key_to_button_code(evdev::KeyCode::BTN_NORTH), ButtonCode::North);
        assert_eq!(key_to_button_code(evdev::KeyCode::BTN_WEST), ButtonCode::West);
        assert_eq!(key_to_button_code(evdev::KeyCode::BTN_START), ButtonCode::Start);
        assert_eq!(key_to_button_code(evdev::KeyCode::BTN_SELECT), ButtonCode::Select);
        assert_eq!(key_to_button_code(evdev::KeyCode::BTN_MODE), ButtonCode::Mode);
    }

    #[test]
    fn test_all_keyboard_code_to_evdev_mappings() {
        assert_eq!(keyboard_code_to_evdev_key(KeyboardCode::Escape), evdev::KeyCode::KEY_ESC);
        assert_eq!(keyboard_code_to_evdev_key(KeyboardCode::A), evdev::KeyCode::KEY_A);
        assert_eq!(keyboard_code_to_evdev_key(KeyboardCode::Num1), evdev::KeyCode::KEY_1);
        assert_eq!(keyboard_code_to_evdev_key(KeyboardCode::Enter), evdev::KeyCode::KEY_ENTER);
        assert_eq!(
            keyboard_code_to_evdev_key(KeyboardCode::LeftControl),
            evdev::KeyCode::KEY_LEFTCTRL
        );
        assert_eq!(keyboard_code_to_evdev_key(KeyboardCode::F1), evdev::KeyCode::KEY_F1);
        assert_eq!(keyboard_code_to_evdev_key(KeyboardCode::Unknown), evdev::KeyCode::KEY_RESERVED);
    }

    #[test]
    fn test_all_axis_code_mappings() {
        // Test all axis mappings
        assert_eq!(absolute_axis_to_axis_code(evdev::AbsoluteAxisCode::ABS_X), AxisCode::LeftX);
        assert_eq!(absolute_axis_to_axis_code(evdev::AbsoluteAxisCode::ABS_Y), AxisCode::LeftY);
        assert_eq!(absolute_axis_to_axis_code(evdev::AbsoluteAxisCode::ABS_RX), AxisCode::RightX);
        assert_eq!(absolute_axis_to_axis_code(evdev::AbsoluteAxisCode::ABS_RY), AxisCode::RightY);
        assert_eq!(
            absolute_axis_to_axis_code(evdev::AbsoluteAxisCode::ABS_Z),
            AxisCode::LeftTrigger
        );
        assert_eq!(
            absolute_axis_to_axis_code(evdev::AbsoluteAxisCode::ABS_RZ),
            AxisCode::RightTrigger
        );
        assert_eq!(absolute_axis_to_axis_code(evdev::AbsoluteAxisCode::ABS_HAT0X), AxisCode::DPadX);
        assert_eq!(absolute_axis_to_axis_code(evdev::AbsoluteAxisCode::ABS_HAT0Y), AxisCode::DPadY);
    }

    #[test]
    fn test_unknown_codes_map_to_unknown() {
        // Test that unknown codes map to Unknown variants
        // We can't easily create unknown enum variants, so we'll test
        // that our mapping functions are total (cover all expected cases)
        // This test passes as long as no panics occur for any input

        // KEY_A is a valid keyboard key but an unknown gamepad button
        assert_eq!(key_to_button_code(evdev::KeyCode::KEY_A), ButtonCode::Unknown);

        let _result2 = absolute_axis_to_axis_code(evdev::AbsoluteAxisCode::ABS_X);
    }
}
