// Known controller database
// Mirrors: internal/device/controller/database.go

use super::types::ControllerType;
use std::collections::HashMap;

/// Controller signature for identification
struct ControllerSignature {
    vendor_id: u16,
    product_id: u16,
    controller_type: ControllerType,
}

/// Known controller signatures
const KNOWN_CONTROLLERS: &[ControllerSignature] = &[
    // Xbox One Controllers
    ControllerSignature {
        vendor_id: 0x045e,
        product_id: 0x02dd,
        controller_type: ControllerType::XboxOne,
    }, // Xbox One Controller (2013, Firmware 2015)
    ControllerSignature {
        vendor_id: 0x045e,
        product_id: 0x02ea,
        controller_type: ControllerType::XboxOne,
    }, // Xbox One S Controller (wireless via dongle)
    ControllerSignature {
        vendor_id: 0x045e,
        product_id: 0x02fd,
        controller_type: ControllerType::XboxOne,
    }, // Xbox One S Controller (Bluetooth)
    // Xbox Series Controllers
    ControllerSignature {
        vendor_id: 0x045e,
        product_id: 0x0b12,
        controller_type: ControllerType::XboxSeries,
    }, // Xbox Series X/S Controller (USB)
    ControllerSignature {
        vendor_id: 0x045e,
        product_id: 0x0b13,
        controller_type: ControllerType::XboxSeries,
    }, // Xbox Series X/S Controller (Bluetooth)
    // Xbox Elite Controllers
    ControllerSignature {
        vendor_id: 0x045e,
        product_id: 0x02e3,
        controller_type: ControllerType::XboxElite,
    }, // Xbox Elite Series 1
    ControllerSignature {
        vendor_id: 0x045e,
        product_id: 0x0b00,
        controller_type: ControllerType::XboxElite,
    }, // Xbox Elite Series 2
    // PlayStation Controllers
    ControllerSignature {
        vendor_id: 0x054c,
        product_id: 0x05c4,
        controller_type: ControllerType::DualShock4,
    }, // DualShock 4 Gen 1
    ControllerSignature {
        vendor_id: 0x054c,
        product_id: 0x09cc,
        controller_type: ControllerType::DualShock4,
    }, // DualShock 4 Gen 2
    ControllerSignature {
        vendor_id: 0x054c,
        product_id: 0x0ce6,
        controller_type: ControllerType::DualSense,
    }, // DualSense (PS5)
];

/// Identify controller type based on vendor/product ID
pub fn identify_controller(vendor_id: u16, product_id: u16) -> ControllerType {
    for sig in KNOWN_CONTROLLERS {
        if sig.vendor_id == vendor_id && sig.product_id == product_id {
            return sig.controller_type;
        }
    }
    ControllerType::Generic
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
        assert_eq!(identify_controller(0x045e, 0x02fd), ControllerType::XboxOne);
    }

    #[test]
    fn test_identify_dualshock4() {
        assert_eq!(identify_controller(0x054c, 0x09cc), ControllerType::DualShock4);
    }

    #[test]
    fn test_identify_unknown() {
        assert_eq!(identify_controller(0xFFFF, 0xFFFF), ControllerType::Generic);
    }

    #[test]
    fn test_vendor_database() {
        let vendors = get_known_vendor_database();
        assert_eq!(vendors.get(&0x045e), Some(&"Microsoft"));
        assert_eq!(vendors.get(&0x054c), Some(&"Sony"));
    }
}
