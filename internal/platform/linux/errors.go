package linux

import "errors"

var (
	ErrPermissionDenied = errors.New("permission denied accessing device")
	ErrDeviceNotFound   = errors.New("device not found or disconnected")
	ErrInvalidDevice    = errors.New("device is not a valid controller")
)
