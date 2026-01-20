// internal/device/vendor/source.go
package vendor

// VendorSource is a platform-agnostic interface for vendor lookup
// Implementations are platform-specific (e.g., USB IDs on Linux, Registry on Windows)
type VendorSource interface {
	Lookup(vendorID uint16) (string, bool)
}
