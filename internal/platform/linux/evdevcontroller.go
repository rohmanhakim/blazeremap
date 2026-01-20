package linux

import (
	"fmt"
	"os"

	"blazeremap.com/blazeremap/internal/device/controller"

	evdev "github.com/gvalkov/golang-evdev"
)

// evdevController implements the Controller interface using the golang-evdev library.
// It wraps a physical input device and provides controller-specific operations.
type evdevController struct {
	device     *evdev.InputDevice
	vendorName string
}

type ControllerBuilder struct {
	path       string
	vendorName string
}

const (
	BtnTriggerHappy1 = 0x2c0
	BtnTriggerHappy4 = 0x2c3
	ElitePaddleCount = 4
)

func NewControllerBuilder(path string) *ControllerBuilder {
	return &ControllerBuilder{path: path}
}

func (b *ControllerBuilder) Build() (controller.Controller, error) {
	if b.vendorName == "" {
		b.vendorName = "Unknown"
	}
	device, err := evdev.Open(b.path)
	if err != nil {
		if os.IsNotExist(err) {
			return nil, fmt.Errorf("%w: %s", ErrDeviceNotFound, b.path)
		}
		if os.IsPermission(err) {
			return nil, fmt.Errorf("%w: %s", ErrPermissionDenied, b.path)
		}
		return nil, fmt.Errorf("failed to open device %s: %w", b.path, err)
	}

	return &evdevController{
		device:     device,
		vendorName: b.vendorName,
	}, nil
}

func (b *ControllerBuilder) WithVendorName(name string) *ControllerBuilder {
	b.vendorName = name
	return b
}

func (c *evdevController) GetName() string      { return c.device.Name }
func (c *evdevController) GetPath() string      { return c.device.Fn }
func (c *evdevController) GetVendorID() uint16  { return c.device.Vendor }
func (c *evdevController) GetProductID() uint16 { return c.device.Product }

func hasForceFeedback(codes []evdev.CapabilityCode) bool {
	return len(codes) > 0
}

func hasElitePaddles(codes []evdev.CapabilityCode) bool {
	paddleCount := 0
	for _, code := range codes {
		// BTN_TRIGGER_HAPPY1-4 (0x2c0-0x2c3 / 704-707)
		if code.Code >= BtnTriggerHappy1 && code.Code <= BtnTriggerHappy4 {
			paddleCount++
		}
	}
	return paddleCount >= ElitePaddleCount
}

func (c *evdevController) GetCapabilities() []controller.ControllerCapability {
	capabilities := make([]controller.ControllerCapability, 0, 2) // Max 2 caps
	for capType, codes := range c.device.Capabilities {
		switch capType.Type {
		case evdev.EV_KEY:
			if hasElitePaddles(codes) {
				capabilities = append(capabilities, controller.CapabilityElitePaddles)
			}
		case evdev.EV_FF:
			if hasForceFeedback(codes) {
				capabilities = append(capabilities, controller.CapabilityFF)
			}
		}
	}
	return capabilities
}

func (c *evdevController) Close() error {
	if c.device == nil || c.device.File == nil {
		return nil // Already closed or never opened
	}
	return c.device.File.Close()
}

func (c *evdevController) GetInfo() *controller.ControllerInfo {
	name := c.GetName()
	path := c.GetPath()
	vendorID := c.GetVendorID()
	productID := c.GetProductID()
	ctrlType := controller.IdentifyController(vendorID, productID)
	capabilities := c.GetCapabilities()

	return &controller.ControllerInfo{
		Path:         path,
		Name:         name,
		Type:         ctrlType,
		VendorID:     vendorID,
		VendorName:   c.vendorName,
		ProductID:    productID,
		Capabilities: capabilities,
	}
}
