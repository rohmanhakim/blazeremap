package linux

import (
	"errors"
	"fmt"
	"strings"

	"blazeremap.com/blazeremap/internal/device"
	"blazeremap.com/blazeremap/internal/device/vendor"
	evdev "github.com/gvalkov/golang-evdev"
)

type linuxDeviceManager struct {
	vendorResolver vendor.Resolver
}

const (
	BtnGamepadMin  = 0x130 // BTN_SOUTH/A
	BtnGamepadMax  = 0x13f // BTN_THUMBR
	BtnJoystickMin = 0x120
	BtnJoystickMax = 0x12f
)

func NewLinuxDeviceManager() device.DeviceManager {
	usbSource := NewUSBIDSource()
	return &linuxDeviceManager{
		vendorResolver: vendor.NewResolver(usbSource),
	}
}

func NewLinuxDeviceManagerWithResolver(resolver vendor.Resolver) device.DeviceManager {
	return &linuxDeviceManager{
		vendorResolver: resolver,
	}
}

func (dm *linuxDeviceManager) ListControllers() (*device.DetectionResult, error) {
	devices, err := evdev.ListInputDevices()
	if err != nil {
		return nil, fmt.Errorf("failed to enumerate /dev/input devices: %w", err)
	}

	detectionResult := device.DetectionResult{}

	for _, d := range devices {
		if isGameController(d) {
			vendorName := dm.vendorResolver.GetVendorName(d.Vendor)

			cb := NewControllerBuilder(d.Fn)
			cb.WithVendorName(vendorName)
			c, err := cb.Build()
			if err != nil {
				detectionResult.Errors = append(detectionResult.Errors, device.DeviceError{
					Path:      d.Fn,
					ErrorType: classifyError(err),
					Err:       err,
				})
			} else {
				info := c.GetInfo()
				if closeErr := c.Close(); closeErr != nil {
					// We got the info, so just log the close error
					// TODO: Add proper logging in Phase 2
				}

				detectionResult.ControllerInfo = append(detectionResult.ControllerInfo, *info)
			}
		}
	}

	return &detectionResult, nil
}

// isGameController checks if device is a game controller
func isGameController(device *evdev.InputDevice) bool {
	var buttons []evdev.CapabilityCode
	var axes []evdev.CapabilityCode
	var hasButtons, hasAxes bool

	// Iterate over capabilities
	for capType, codes := range device.Capabilities {
		switch capType.Type {
		case evdev.EV_KEY: // Button events
			buttons = codes
			hasButtons = len(codes) > 0
		case evdev.EV_ABS: // Absolute axis events
			axes = codes
			hasAxes = len(codes) > 0
		}
	}

	if !hasButtons || !hasAxes {
		return false
	}

	// Check for gamepad buttons (BTN_GAMEPAD or BTN_JOYSTICK range)
	hasGamepadButton := false
	for _, code := range buttons {
		// BTN_SOUTH/A through BTN_THUMBR (0x130-0x13f / 304-319)
		if code.Code >= BtnGamepadMin && code.Code <= BtnGamepadMax {
			hasGamepadButton = true
			break
		}
		// BTN_JOYSTICK range (0x120-0x12f / 288-303)
		if code.Code >= BtnJoystickMin && code.Code <= BtnJoystickMax {
			hasGamepadButton = true
			break
		}
	}

	if !hasGamepadButton {
		return false
	}

	// Check for gamepad axes (analog sticks)
	hasGamepadAxis := false
	for _, code := range axes {
		// ABS_X, ABS_Y, ABS_RX, ABS_RY (0, 1, 3, 4)
		if code.Code == evdev.ABS_X || code.Code == evdev.ABS_Y ||
			code.Code == evdev.ABS_RX || code.Code == evdev.ABS_RY {
			hasGamepadAxis = true
			break
		}
	}

	if !hasGamepadAxis {
		return false
	}

	// Exclude non-controllers by name
	return !isExcludedByName(device.Name)
}

func isExcludedByName(name string) bool {
	nameLower := strings.ToLower(name)

	excludeKeywords := []string{
		"keyboard", "mouse", "touchpad",
		"power button", "sleep button",
		"hdmi", "audio", "speaker", "headphone", "microphone",
		"line out", "line in",
		"led", "lamplight", "rgb",
		"system control", "consumer control",
	}

	for _, keyword := range excludeKeywords {
		if strings.Contains(nameLower, keyword) {
			return true
		}
	}

	return false
}

func classifyError(err error) device.ErrorType {
	if errors.Is(err, ErrPermissionDenied) {
		return device.ErrorTypePermission
	}
	if errors.Is(err, ErrDeviceNotFound) {
		return device.ErrorTypeNotFound
	}
	if errors.Is(err, ErrInvalidDevice) {
		return device.ErrorTypeInvalidDevice
	}

	return device.ErrorTypeUnknown
}
