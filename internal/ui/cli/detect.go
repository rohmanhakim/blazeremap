package cli

import (
	"blazeremap.com/blazeremap/internal/device"
	"blazeremap.com/blazeremap/internal/device/controller"
	"github.com/spf13/cobra"
)

func NewDetectCmd(
	opts *Options,
	deviceManager device.DeviceManager,
) *cobra.Command {

	cmd := &cobra.Command{
		Use:   "detect",
		Short: "Detect controllers connected to your computer",
		Long:  "Detect controllers connected to your computer",
		RunE: func(cmd *cobra.Command, args []string) error {
			result, err := deviceManager.ListControllers()
			if err != nil {
				return err
			}

			cmd.Printf("Found %d controller(s):\n\n", len(result.ControllerInfo))

			for i, info := range result.ControllerInfo {
				cmd.Printf("[%d] %s (%s)\n", i, info.Name, info.Path)
				cmd.Printf(" ├─ Type: %s\n", info.Type)
				cmd.Println(" ├─ Vendor:")
				cmd.Printf(" │  ├─ ID: %04X\n", info.VendorID)
				cmd.Printf(" │  └─ Name: %s\n", info.VendorName)
				cmd.Printf(" ├─ Product ID: %04X\n", info.ProductID)
				cmd.Println(" └─ Capabilities:")
				caps := controller.CapabilitiesToStrings(info.Capabilities)
				for i, cap := range caps {
					prefix := "    ├─ "
					if i == len(caps)-1 {
						prefix = "    └─ "
					}
					cmd.Printf("%s%s\n", prefix, cap)
				}
			}

			return nil
		},
	}

	return cmd
}
