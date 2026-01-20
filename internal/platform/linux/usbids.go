// platform/linux/usbids.go
package linux

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
	"sync"
)

// usbIDDatabase implements vendor.VendorSource for Linux USB IDs
type usbIDDatabase struct {
	vendors map[uint16]string
	once    sync.Once
	err     error
}

// NewUSBIDSource creates a lazy-loading USB ID vendor source
func NewUSBIDSource() *usbIDDatabase {
	return &usbIDDatabase{}
}

// Lookup implements vendor.VendorSource
func (db *usbIDDatabase) Lookup(vendorID uint16) (string, bool) {
	// Lazy load on first use
	db.once.Do(func() {
		db.vendors, db.err = loadUSBIDDatabase()
	})

	if db.err != nil {
		return "", false
	}

	name, found := db.vendors[vendorID]
	return name, found
}

// loadUSBIDDatabase reads the Linux USB IDs file
func loadUSBIDDatabase() (map[uint16]string, error) {
	// Linux-specific paths
	paths := []string{
		"/usr/share/hwdata/usb.ids",
		"/var/lib/usbutils/usb.ids",
		"/usr/share/misc/usb.ids",
	}

	var file *os.File
	var err error

	for _, path := range paths {
		file, err = os.Open(path)
		if err == nil {
			defer file.Close()
			break
		}
	}

	if file == nil {
		return nil, fmt.Errorf("USB IDs database not found in standard Linux locations")
	}

	vendors := make(map[uint16]string)
	scanner := bufio.NewScanner(file)

	for scanner.Scan() {
		line := scanner.Text()

		// Skip comments and empty lines
		if len(line) == 0 || line[0] == '#' {
			continue
		}

		// Vendor lines start at column 0 with 4-digit hex ID
		if len(line) > 6 && line[0] != '\t' {
			parts := strings.SplitN(line, "  ", 2)
			if len(parts) == 2 {
				vendorIDStr := strings.TrimSpace(parts[0])
				vendorName := strings.TrimSpace(parts[1])

				if vendorID, err := strconv.ParseUint(vendorIDStr, 16, 16); err == nil {
					vendors[uint16(vendorID)] = vendorName
				}
			}
		}
	}

	if err := scanner.Err(); err != nil {
		return nil, fmt.Errorf("error reading USB IDs database: %w", err)
	}

	return vendors, nil
}
