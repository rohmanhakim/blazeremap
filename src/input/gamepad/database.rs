// Known gamepad database

use super::types::GamepadType;
use std::collections::HashMap;

/// Gamepad signature for identification
struct GamepadSignature {
    vendor_id: u16,
    product_id: u16,
    gamepad_type: GamepadType,
}

/// Known gamepad signatures
const KNOWN_GAMEPADS: &[GamepadSignature] = &[
    // Xbox One
    GamepadSignature { vendor_id: 0x045e, product_id: 0x02dd, gamepad_type: GamepadType::XboxOne }, // Xbox One Controller (2013, Firmware 2015)
    GamepadSignature { vendor_id: 0x045e, product_id: 0x02ea, gamepad_type: GamepadType::XboxOne }, // Xbox One S Controller (wireless via dongle)
    GamepadSignature { vendor_id: 0x045e, product_id: 0x02fd, gamepad_type: GamepadType::XboxOne }, // Xbox One S Controller (Bluetooth)
    // Xbox Series
    GamepadSignature {
        vendor_id: 0x045e,
        product_id: 0x0b12,
        gamepad_type: GamepadType::XboxSeries,
    }, // Xbox Series X/S Controller (USB)
    GamepadSignature {
        vendor_id: 0x045e,
        product_id: 0x0b13,
        gamepad_type: GamepadType::XboxSeries,
    }, // Xbox Series X/S Controller (Bluetooth)
    // Xbox Elite
    GamepadSignature {
        vendor_id: 0x045e,
        product_id: 0x02e3,
        gamepad_type: GamepadType::XboxElite,
    }, // Xbox Elite Series 1
    GamepadSignature {
        vendor_id: 0x045e,
        product_id: 0x0b00,
        gamepad_type: GamepadType::XboxElite,
    }, // Xbox Elite Series 2
    // PlayStation Gamepads
    GamepadSignature {
        vendor_id: 0x054c,
        product_id: 0x05c4,
        gamepad_type: GamepadType::DualShock4,
    }, // DualShock 4 Gen 1
    GamepadSignature {
        vendor_id: 0x054c,
        product_id: 0x09cc,
        gamepad_type: GamepadType::DualShock4,
    }, // DualShock 4 Gen 2
    GamepadSignature {
        vendor_id: 0x054c,
        product_id: 0x0ce6,
        gamepad_type: GamepadType::DualSense,
    }, // DualSense (PS5)
];

/// Identify gamepad type based on vendor/product ID
pub fn identify_gamepad(vendor_id: u16, product_id: u16) -> GamepadType {
    for sig in KNOWN_GAMEPADS {
        if sig.vendor_id == vendor_id && sig.product_id == product_id {
            return sig.gamepad_type;
        }
    }
    GamepadType::Generic
}

/// Get the known vendor database
pub fn get_known_vendor_database() -> HashMap<u16, &'static str> {
    let mut vendors = HashMap::new();
    vendors.insert(0x045e, "Microsoft");
    vendors.insert(0x054c, "Sony");
    vendors.insert(0x057e, "Nintendo");
    vendors.insert(0x046d, "Logitech");
    vendors.insert(0x0e6f, "Logic3");
    vendors.insert(0x0f0d, "Hori");
    vendors.insert(0x1532, "Razer");
    vendors.insert(0x2dc8, "8BitDo");
    vendors.insert(0x28de, "Valve");
    vendors
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identify_xbox_one() {
        assert_eq!(identify_gamepad(0x045e, 0x02fd), GamepadType::XboxOne);
    }

    #[test]
    fn test_identify_dualshock4() {
        assert_eq!(identify_gamepad(0x054c, 0x09cc), GamepadType::DualShock4);
    }

    #[test]
    fn test_identify_unknown() {
        assert_eq!(identify_gamepad(0xFFFF, 0xFFFF), GamepadType::Generic);
    }

    #[test]
    fn test_vendor_database() {
        let vendors = get_known_vendor_database();
        assert_eq!(vendors.get(&0x045e), Some(&"Microsoft"));
        assert_eq!(vendors.get(&0x054c), Some(&"Sony"));
    }
}
