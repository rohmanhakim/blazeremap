package internal

import (
	"bytes"

	"blazeremap.com/blazeremap/internal/device"
	"blazeremap.com/blazeremap/internal/device/controller"
	"blazeremap.com/blazeremap/internal/ui/cli"
	"github.com/spf13/cobra"
)

type MockCli struct {
	ExecuteCalled bool
	ExecuteError  error
	Cmd           *cobra.Command
	output        *bytes.Buffer
}

func NewMockCli() *MockCli {
	return &MockCli{
		ExecuteCalled: false,
		ExecuteError:  nil,
	}
}

func NewMockCliWithCommand() *MockCli {
	mockDM := NewMockDeviceManager()
	opts := &cli.Options{}
	cmd := cli.NewRootCmd(opts, mockDM)

	buf := new(bytes.Buffer)
	cmd.SetOut(buf)
	cmd.SetErr(buf)

	return &MockCli{
		ExecuteCalled: false,
		Cmd:           cmd,
		output:        buf,
	}
}

func (m *MockCli) Execute() error {
	m.ExecuteCalled = true
	if m.ExecuteError != nil {
		return m.ExecuteError
	}
	if m.Cmd != nil {
		return m.Cmd.Execute()
	}
	return nil
}

func (m *MockCli) WithError(err error) *MockCli {
	m.ExecuteError = err
	return m
}

func (m *MockCli) GetOutput() string {
	if m.output != nil {
		return m.output.String()
	}
	return ""
}

// MockDeviceManager implements device.DeviceManager for testing
type MockDeviceManager struct {
	controllers []controller.ControllerInfo
	errors      []device.DeviceError
	err         error
}

func NewMockDeviceManager() *MockDeviceManager {
	return &MockDeviceManager{
		controllers: []controller.ControllerInfo{},
		errors:      []device.DeviceError{},
	}
}

func (dm *MockDeviceManager) ListControllers() (*device.DetectionResult, error) {
	if dm.err != nil {
		return nil, dm.err
	}

	return &device.DetectionResult{
		ControllerInfo: dm.controllers,
		Errors:         dm.errors,
	}, nil
}

func (dm *MockDeviceManager) WithControllers(controllers ...controller.ControllerInfo) *MockDeviceManager {
	dm.controllers = append(dm.controllers, controllers...)
	return dm
}

func (dm *MockDeviceManager) WithErrors(errors ...device.DeviceError) *MockDeviceManager {
	dm.errors = append(dm.errors, errors...)
	return dm
}

func (dm *MockDeviceManager) WithError(err error) *MockDeviceManager {
	dm.err = err
	return dm
}

// Helper to create mock controller info
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
