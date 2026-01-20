// test_helper.go
package cli

import (
	"bytes"
	"errors"

	"blazeremap.com/blazeremap/internal/device"
	"blazeremap.com/blazeremap/internal/device/controller"
	"github.com/spf13/cobra"
)

// ExecuteCommand executes a command with the given arguments and returns output
func ExecuteCommand(cmd *cobra.Command, args ...string) (string, error) {
	buf := new(bytes.Buffer)
	cmd.SetOut(buf)
	cmd.SetErr(buf)
	cmd.SetArgs(args)

	err := cmd.Execute()
	return buf.String(), err
}

// mockDeviceManager is a mock implementation of device.DeviceManager for testing
type mockDeviceManager struct {
	controllers []controller.ControllerInfo
	errors      []device.DeviceError
	err         error
}

func (dm *mockDeviceManager) ListControllers() (*device.DetectionResult, error) {
	if dm.err != nil {
		return nil, dm.err
	}

	return &device.DetectionResult{
		ControllerInfo: dm.controllers,
		Errors:         dm.errors,
	}, nil
}

// NewMockDeviceManager creates a new mock device manager
func NewMockDeviceManager() *mockDeviceManager {
	return &mockDeviceManager{
		controllers: []controller.ControllerInfo{},
		errors:      []device.DeviceError{},
	}
}

// WithControllers adds mock controllers to the device manager
func (dm *mockDeviceManager) WithControllers(controllers ...controller.ControllerInfo) *mockDeviceManager {
	dm.controllers = append(dm.controllers, controllers...)
	return dm
}

// WithErrors adds mock errors to the device manager
func (dm *mockDeviceManager) WithErrors(errors ...device.DeviceError) *mockDeviceManager {
	dm.errors = append(dm.errors, errors...)
	return dm
}

// WithError sets an error to be returned by ListControllers
func (dm *mockDeviceManager) WithError(err error) *mockDeviceManager {
	dm.err = err
	return dm
}

// Helper function to create a mock controller info
func NewMockControllerInfo(name, path string, ctrlType controller.ControllerType) controller.ControllerInfo {
	return controller.ControllerInfo{
		Name:         name,
		Path:         path,
		Type:         ctrlType,
		VendorID:     0x045e,
		VendorName:   "Microsoft",
		ProductID:    0x02fd,
		Capabilities: []controller.ControllerCapability{},
	}
}

// Helper function to create a mock device error
func NewMockDeviceError(path string, errType device.ErrorType) device.DeviceError {
	return device.DeviceError{
		Path:      path,
		ErrorType: errType,
		Err:       errors.New("mock error"),
	}
}

// test_helper.go (additions)

// NewMockControllerInfoWithCaps creates a mock controller with specific capabilities
func NewMockControllerInfoWithCaps(
	name, path string,
	ctrlType controller.ControllerType,
	caps []controller.ControllerCapability,
) controller.ControllerInfo {
	return controller.ControllerInfo{
		Name:         name,
		Path:         path,
		Type:         ctrlType,
		VendorID:     0x045e,
		VendorName:   "Microsoft",
		ProductID:    0x02fd,
		Capabilities: caps,
	}
}

// NewMockControllerInfoFull creates a fully customizable mock controller
func NewMockControllerInfoFull(
	name, path string,
	ctrlType controller.ControllerType,
	vendorID uint16,
	vendorName string,
	productID uint16,
	caps []controller.ControllerCapability,
) controller.ControllerInfo {
	return controller.ControllerInfo{
		Name:         name,
		Path:         path,
		Type:         ctrlType,
		VendorID:     vendorID,
		VendorName:   vendorName,
		ProductID:    productID,
		Capabilities: caps,
	}
}
