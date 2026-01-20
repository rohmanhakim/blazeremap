package controller

type controllerSignature struct {
	vendorID  uint16
	productID uint16
	ctrlType  ControllerType
}

// Map of vendor IDs to vendor names
var knownVendors = map[uint16]string{
	0x045e: "Microsoft",
	0x054c: "Sony",
	0x057e: "Nintendo",
	0x046d: "Logitech",
	0x0e6f: "Logic3",
	0x0f0d: "Hori",
	0x1532: "Razer",
	0x2dc8: "8BitDo",
	0x28de: "Valve",
	// Add more as needed
}

var knownControllers = []controllerSignature{
	// Xbox One Controllers
	{0x045e, 0x02dd, ControllerTypeXboxOne},    // Xbox One Controller (2013, Firmware 2015)
	{0x045e, 0x02ea, ControllerTypeXboxOne},    // Xbox One S Controller (wireless via dongle)
	{0x045e, 0x02fd, ControllerTypeXboxOne},    // Xbox One S Controller (Bluetooth) ✅ YOUR CONTROLLER
	{0x045e, 0x0b12, ControllerTypeXboxSeries}, // Xbox Series X/S Controller (USB)
	{0x045e, 0x0b13, ControllerTypeXboxSeries}, // Xbox Series X/S Controller (Bluetooth)

	// Xbox Elite Controllers
	{0x045e, 0x02e3, ControllerTypeXboxElite}, // Xbox Elite Series 1
	{0x045e, 0x0b00, ControllerTypeXboxElite}, // Xbox Elite Series 2

	// PlayStation Controllers
	{0x054c, 0x05c4, ControllerTypeDualShock4}, // DualShock 4 Gen 1
	{0x054c, 0x09cc, ControllerTypeDualShock4}, // DualShock 4 Gen 2 ✅ YOUR BOOTLEG
	{0x054c, 0x0ce6, ControllerTypeDualSense},  // DualSense (PS5)
}

// IdentifyController returns the controller type based on vendor/product ID
func IdentifyController(vendorID, productID uint16) ControllerType {
	for _, sig := range knownControllers {
		if sig.vendorID == vendorID && sig.productID == productID {
			return sig.ctrlType
		}
	}
	return ControllerTypeGeneric
}

func GetKnownVendorDatabase() map[uint16]string {
	return knownVendors
}
