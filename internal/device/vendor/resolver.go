// internal/device/vendor/resolver.go
package vendor

import "fmt"

// Resolver resolves vendor IDs to human-readable names
type Resolver interface {
	GetVendorName(vendorID uint16) string
}

// resolver implements multi-tier vendor name resolution
type resolver struct {
	hardcoded map[uint16]string
	sources   []VendorSource // Platform-specific sources (USB DB, registry, etc.)
}

// NewResolver creates a resolver with hardcoded vendors
func NewResolver(sources ...VendorSource) Resolver {
	return &resolver{
		hardcoded: getHardcodedVendors(),
		sources:   sources,
	}
}

func (r *resolver) GetVendorName(vendorID uint16) string {
	// Tier 1: Hardcoded (fastest)
	if name, exists := r.hardcoded[vendorID]; exists {
		return name
	}

	// Tier 2: Platform-specific sources
	for _, source := range r.sources {
		if name, found := source.Lookup(vendorID); found {
			return name
		}
	}

	// Tier 3: Fallback
	return fmt.Sprintf("Unknown (0x%04x)", vendorID)
}

// getHardcodedVendors returns common gaming controller vendors
// This is domain knowledge, not platform knowledge
func getHardcodedVendors() map[uint16]string {
	return map[uint16]string{
		0x045e: "Microsoft",
		0x054c: "Sony",
		0x057e: "Nintendo",
		0x046d: "Logitech",
		0x0e6f: "Logic3",
		0x0f0d: "Hori",
		0x1532: "Razer",
		0x2dc8: "8BitDo",
		0x28de: "Valve",
	}
}
