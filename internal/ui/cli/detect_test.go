package cli_test

import (
	"testing"

	"blazeremap.com/blazeremap/internal/device/controller"
	"blazeremap.com/blazeremap/internal/ui/cli"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

func TestDetectCommand(t *testing.T) {
	t.Run("no controllers found", func(t *testing.T) {
		opts := &cli.Options{}
		mockDM := cli.NewMockDeviceManager()
		cmd := cli.NewRootCmd(opts, mockDM)

		output, err := cli.ExecuteCommand(cmd, "detect")

		require.NoError(t, err)
		assert.Contains(t, output, "Found 0 controller(s)")
	})

	t.Run("single controller found", func(t *testing.T) {
		opts := &cli.Options{}
		mockDM := cli.NewMockDeviceManager().
			WithControllers(
				cli.NewMockControllerInfo(
					"Xbox Wireless Controller",
					"/dev/input/event3",
					controller.ControllerTypeXboxOne,
				),
			)
		cmd := cli.NewRootCmd(opts, mockDM)

		output, err := cli.ExecuteCommand(cmd, "detect")

		require.NoError(t, err)
		assert.Contains(t, output, "Found 1 controller(s)")
		assert.Contains(t, output, "[0] Xbox Wireless Controller")
		assert.Contains(t, output, "/dev/input/event3")
		assert.Contains(t, output, "Type: Xbox One")
		assert.Contains(t, output, "Vendor:")
		assert.Contains(t, output, "ID: 045E")
		assert.Contains(t, output, "Name: Microsoft")
		assert.Contains(t, output, "Product ID: 02FD")
	})

	t.Run("multiple controllers found", func(t *testing.T) {
		opts := &cli.Options{}
		mockDM := cli.NewMockDeviceManager().
			WithControllers(
				cli.NewMockControllerInfo(
					"Xbox Wireless Controller",
					"/dev/input/event3",
					controller.ControllerTypeXboxOne,
				),
				cli.NewMockControllerInfo(
					"Sony DualShock 4",
					"/dev/input/event4",
					controller.ControllerTypeDualShock4,
				),
			)
		cmd := cli.NewRootCmd(opts, mockDM)

		output, err := cli.ExecuteCommand(cmd, "detect")

		require.NoError(t, err)
		assert.Contains(t, output, "Found 2 controller(s)")
		assert.Contains(t, output, "[0] Xbox Wireless Controller")
		assert.Contains(t, output, "[1] Sony DualShock 4")
		assert.Contains(t, output, "Type: Xbox One")
		assert.Contains(t, output, "Type: DualShock 4")
	})

	t.Run("controller with force feedback capability", func(t *testing.T) {
		opts := &cli.Options{}
		mockDM := cli.NewMockDeviceManager().
			WithControllers(
				cli.NewMockControllerInfoWithCaps(
					"Xbox Wireless Controller",
					"/dev/input/event3",
					controller.ControllerTypeXboxOne,
					[]controller.ControllerCapability{controller.CapabilityFF},
				),
			)
		cmd := cli.NewRootCmd(opts, mockDM)

		output, err := cli.ExecuteCommand(cmd, "detect")

		require.NoError(t, err)
		assert.Contains(t, output, "Found 1 controller(s)")
		assert.Contains(t, output, "Capabilities:")
		assert.Contains(t, output, "Force Feedback")
	})

	t.Run("controller with elite paddles capability", func(t *testing.T) {
		opts := &cli.Options{}
		mockDM := cli.NewMockDeviceManager().
			WithControllers(
				cli.NewMockControllerInfoWithCaps(
					"Xbox Elite Wireless Controller",
					"/dev/input/event3",
					controller.ControllerTypeXboxElite,
					[]controller.ControllerCapability{
						controller.CapabilityFF,
						controller.CapabilityElitePaddles,
					},
				),
			)
		cmd := cli.NewRootCmd(opts, mockDM)

		output, err := cli.ExecuteCommand(cmd, "detect")

		require.NoError(t, err)
		assert.Contains(t, output, "Found 1 controller(s)")
		assert.Contains(t, output, "Type: Xbox Elite")
		assert.Contains(t, output, "Capabilities:")
		assert.Contains(t, output, "Force Feedback")
		assert.Contains(t, output, "Elite Paddles")
	})

	t.Run("controller with no capabilities", func(t *testing.T) {
		opts := &cli.Options{}
		mockDM := cli.NewMockDeviceManager().
			WithControllers(
				cli.NewMockControllerInfoWithCaps(
					"Generic Gamepad",
					"/dev/input/event5",
					controller.ControllerTypeGeneric,
					[]controller.ControllerCapability{},
				),
			)
		cmd := cli.NewRootCmd(opts, mockDM)

		output, err := cli.ExecuteCommand(cmd, "detect")

		require.NoError(t, err)
		assert.Contains(t, output, "Found 1 controller(s)")
		assert.Contains(t, output, "Type: Generic")
		assert.Contains(t, output, "Capabilities:")
		// Should show Capabilities header but no items under it
	})

	t.Run("controller with unknown vendor", func(t *testing.T) {
		opts := &cli.Options{}
		mockDM := cli.NewMockDeviceManager().
			WithControllers(
				controller.ControllerInfo{
					Name:         "Unknown Controller",
					Path:         "/dev/input/event6",
					Type:         controller.ControllerTypeGeneric,
					VendorID:     0x9999,
					VendorName:   "Unknown (0x9999)",
					ProductID:    0x0001,
					Capabilities: []controller.ControllerCapability{},
				},
			)
		cmd := cli.NewRootCmd(opts, mockDM)

		output, err := cli.ExecuteCommand(cmd, "detect")

		require.NoError(t, err)
		assert.Contains(t, output, "Found 1 controller(s)")
		assert.Contains(t, output, "Name: Unknown (0x9999)")
		assert.Contains(t, output, "ID: 9999")
	})

	t.Run("device manager returns error", func(t *testing.T) {
		opts := &cli.Options{}
		mockDM := cli.NewMockDeviceManager().
			WithError(assert.AnError)
		cmd := cli.NewRootCmd(opts, mockDM)

		_, err := cli.ExecuteCommand(cmd, "detect")

		assert.Error(t, err)
		assert.Equal(t, assert.AnError, err)
	})

	t.Run("output formatting - tree structure", func(t *testing.T) {
		opts := &cli.Options{}
		mockDM := cli.NewMockDeviceManager().
			WithControllers(
				cli.NewMockControllerInfoWithCaps(
					"Test Controller",
					"/dev/input/event3",
					controller.ControllerTypeXboxOne,
					[]controller.ControllerCapability{controller.CapabilityFF},
				),
			)
		cmd := cli.NewRootCmd(opts, mockDM)

		output, err := cli.ExecuteCommand(cmd, "detect")

		require.NoError(t, err)
		// Verify tree structure characters
		assert.Contains(t, output, "├─") // Branch
		assert.Contains(t, output, "│")  // Vertical line
		assert.Contains(t, output, "└─") // Last branch
	})

	t.Run("output formatting - capabilities tree", func(t *testing.T) {
		opts := &cli.Options{}
		mockDM := cli.NewMockDeviceManager().
			WithControllers(
				cli.NewMockControllerInfoWithCaps(
					"Test Controller",
					"/dev/input/event3",
					controller.ControllerTypeXboxOne,
					[]controller.ControllerCapability{
						controller.CapabilityFF,
						controller.CapabilityElitePaddles,
					},
				),
			)
		cmd := cli.NewRootCmd(opts, mockDM)

		output, err := cli.ExecuteCommand(cmd, "detect")

		require.NoError(t, err)
		// First capability should have ├─
		assert.Regexp(t, `Capabilities:\s+├─ Force Feedback`, output)
		// Last capability should have └─
		assert.Regexp(t, `└─ Elite Paddles`, output)
	})

	t.Run("handles all controller types", func(t *testing.T) {
		tests := []struct {
			name         string
			ctrlType     controller.ControllerType
			expectedType string
		}{
			{"Xbox One", controller.ControllerTypeXboxOne, "Type: Xbox One"},
			{"Xbox Series", controller.ControllerTypeXboxSeries, "Type: Xbox Series X/S"},
			{"Xbox Elite", controller.ControllerTypeXboxElite, "Type: Xbox Elite"},
			{"DualShock 4", controller.ControllerTypeDualShock4, "Type: DualShock 4"},
			{"DualSense", controller.ControllerTypeDualSense, "Type: DualSense"},
			{"Generic", controller.ControllerTypeGeneric, "Type: Generic"},
			{"Unknown", controller.ControllerTypeUnknown, "Type: Unknown"},
		}

		for _, tt := range tests {
			t.Run(tt.name, func(t *testing.T) {
				opts := &cli.Options{}
				mockDM := cli.NewMockDeviceManager().
					WithControllers(
						cli.NewMockControllerInfo(
							"Test Controller",
							"/dev/input/event3",
							tt.ctrlType,
						),
					)
				cmd := cli.NewRootCmd(opts, mockDM)

				output, err := cli.ExecuteCommand(cmd, "detect")

				require.NoError(t, err)
				assert.Contains(t, output, tt.expectedType)
			})
		}
	})

	t.Run("vendor ID formatting", func(t *testing.T) {
		opts := &cli.Options{}
		mockDM := cli.NewMockDeviceManager().
			WithControllers(
				controller.ControllerInfo{
					Name:         "Test Controller",
					Path:         "/dev/input/event3",
					Type:         controller.ControllerTypeXboxOne,
					VendorID:     0x045e, // Should format as 045E
					VendorName:   "Microsoft",
					ProductID:    0x02fd, // Should format as 02FD
					Capabilities: []controller.ControllerCapability{},
				},
			)
		cmd := cli.NewRootCmd(opts, mockDM)

		output, err := cli.ExecuteCommand(cmd, "detect")

		require.NoError(t, err)
		// Verify uppercase hex formatting with leading zeros
		assert.Contains(t, output, "ID: 045E")
		assert.Contains(t, output, "Product ID: 02FD")
	})

	t.Run("product ID formatting", func(t *testing.T) {
		opts := &cli.Options{}
		mockDM := cli.NewMockDeviceManager().
			WithControllers(
				controller.ControllerInfo{
					Name:         "Test Controller",
					Path:         "/dev/input/event3",
					Type:         controller.ControllerTypeXboxOne,
					VendorID:     0x045e,
					VendorName:   "Microsoft",
					ProductID:    0x0001, // Should format as 0001 (with leading zeros)
					Capabilities: []controller.ControllerCapability{},
				},
			)
		cmd := cli.NewRootCmd(opts, mockDM)

		output, err := cli.ExecuteCommand(cmd, "detect")

		require.NoError(t, err)
		assert.Contains(t, output, "Product ID: 0001")
	})

	t.Run("multiple controllers indexed correctly", func(t *testing.T) {
		opts := &cli.Options{}
		mockDM := cli.NewMockDeviceManager().
			WithControllers(
				cli.NewMockControllerInfo("Controller 0", "/dev/input/event3", controller.ControllerTypeXboxOne),
				cli.NewMockControllerInfo("Controller 1", "/dev/input/event4", controller.ControllerTypeXboxOne),
				cli.NewMockControllerInfo("Controller 2", "/dev/input/event5", controller.ControllerTypeXboxOne),
			)
		cmd := cli.NewRootCmd(opts, mockDM)

		output, err := cli.ExecuteCommand(cmd, "detect")

		require.NoError(t, err)
		assert.Contains(t, output, "Found 3 controller(s)")
		assert.Contains(t, output, "[0] Controller 0")
		assert.Contains(t, output, "[1] Controller 1")
		assert.Contains(t, output, "[2] Controller 2")
	})
}

func TestDetectCommandUsage(t *testing.T) {
	t.Run("has correct usage", func(t *testing.T) {
		opts := &cli.Options{}
		mockDM := cli.NewMockDeviceManager()
		rootCmd := cli.NewRootCmd(opts, mockDM)

		detectCmd, _, err := rootCmd.Find([]string{"detect"})

		require.NoError(t, err)
		assert.Equal(t, "detect", detectCmd.Use)
		assert.Equal(t, "Detect controllers connected to your computer", detectCmd.Short)
		assert.Equal(t, "Detect controllers connected to your computer", detectCmd.Long)
	})

	t.Run("has no flags", func(t *testing.T) {
		opts := &cli.Options{}
		mockDM := cli.NewMockDeviceManager()
		rootCmd := cli.NewRootCmd(opts, mockDM)

		detectCmd, _, err := rootCmd.Find([]string{"detect"})

		require.NoError(t, err)
		// Detect command should not have any local flags
		assert.False(t, detectCmd.Flags().HasFlags())
	})

	t.Run("does not accept arguments", func(t *testing.T) {
		opts := &cli.Options{}
		mockDM := cli.NewMockDeviceManager()
		cmd := cli.NewRootCmd(opts, mockDM)

		output, err := cli.ExecuteCommand(cmd, "detect", "unexpected-arg")

		// Cobra doesn't error on extra args by default, but the command should still work
		require.NoError(t, err)
		assert.Contains(t, output, "Found 0 controller(s)")
	})
}

// Benchmark tests
func BenchmarkDetectCommand(b *testing.B) {
	opts := &cli.Options{}
	mockDM := cli.NewMockDeviceManager().
		WithControllers(
			cli.NewMockControllerInfo("Xbox Controller", "/dev/input/event3", controller.ControllerTypeXboxOne),
			cli.NewMockControllerInfo("PS4 Controller", "/dev/input/event4", controller.ControllerTypeDualShock4),
		)

	b.ResetTimer()
	for i := 0; b.Loop(); i++ {
		cmd := cli.NewRootCmd(opts, mockDM)
		_, _ = cli.ExecuteCommand(cmd, "detect")
	}
}
