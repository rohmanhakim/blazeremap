package platform

import (
	"runtime"

	"blazeremap.com/blazeremap/internal/device"
	"blazeremap.com/blazeremap/internal/platform/linux"
)

func NewDeviceManager() device.DeviceManager {
	switch runtime.GOOS {
	case "linux":
		return linux.NewLinuxDeviceManager()
	default:
		panic("unsupported platform")
	}
}
