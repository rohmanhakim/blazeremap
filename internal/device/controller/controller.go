package controller

// Controller represents a physical game controller
type Controller interface {
	// Get detailed info about the controller
	GetInfo() *ControllerInfo

	// Close releases the device
	Close() error
}

type ControllerInfo struct {
	Path         string
	Name         string
	Type         ControllerType
	VendorID     uint16
	VendorName   string
	ProductID    uint16
	Capabilities []ControllerCapability
}

func CapabilitiesToStrings(caps []ControllerCapability) []string {
	strs := make([]string, len(caps))
	for i, cap := range caps {
		strs[i] = cap.String() // ✅ Use Stringer
	}
	return strs
}
