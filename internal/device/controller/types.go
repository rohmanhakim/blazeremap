package controller

type ControllerType int

const (
	ControllerTypeUnknown ControllerType = iota
	ControllerTypeXboxOne
	ControllerTypeXboxSeries
	ControllerTypeXboxElite
	ControllerTypeDualShock4
	ControllerTypeDualSense
	ControllerTypeGeneric
)

func (ct ControllerType) String() string {
	switch ct {
	case ControllerTypeXboxOne:
		return "Xbox One"
	case ControllerTypeXboxSeries:
		return "Xbox Series X/S"
	case ControllerTypeXboxElite:
		return "Xbox Elite"
	case ControllerTypeDualShock4:
		return "DualShock 4"
	case ControllerTypeDualSense:
		return "DualSense"
	case ControllerTypeGeneric:
		return "Generic"
	default:
		return "Unknown"
	}
}

type ControllerCapability int

const (
	CapabilityFF ControllerCapability = iota
	CapabilityElitePaddles
)

func (cc ControllerCapability) String() string {
	switch cc {
	case CapabilityFF:
		return "Force Feedback"
	case CapabilityElitePaddles:
		return "Elite Paddles"
	default:
		return "Unknown"
	}
}
