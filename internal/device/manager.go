package device

import (
	"blazeremap.com/blazeremap/internal/device/controller"
)

// DeviceManager handles device discovery and creation
type DeviceManager interface {
	// ListControllers returns all connected controllers
	ListControllers() (*DetectionResult, error)
}

// DetectionResult contains the results of controller detection
type DetectionResult struct {
	ControllerInfo []controller.ControllerInfo
	Errors         []DeviceError
}

// ErrorType classifies device-related errors
type ErrorType int

const (
	ErrorTypePermission    ErrorType = iota // Permission denied
	ErrorTypeNotFound                       // Device not found
	ErrorTypeInvalidDevice                  // Invalid or unsupported device
	ErrorTypeUnknown                        // Unknown error
)

// DeviceError represents an error encountered during device detection
type DeviceError struct {
	Path      string
	ErrorType ErrorType
	Err       error
}
