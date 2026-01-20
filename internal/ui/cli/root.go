package cli

import (
	"blazeremap.com/blazeremap/internal/device"
	"github.com/spf13/cobra"
)

const AppVersion = "0.1"

type Options struct {
	Version bool
}

func NewRootCmd(opts *Options, deviceManager device.DeviceManager) *cobra.Command {
	cmd := &cobra.Command{
		Use:   "blazeremap",
		Short: "BlazeRemap",
		RunE: func(cmd *cobra.Command, args []string) error {
			if opts.Version {
				cmd.Println("BlazeRemap", AppVersion)
			}
			return nil
		},
	}

	cmd.Flags().BoolVarP(&opts.Version, "version", "v", false, "show app version")
	cmd.AddCommand(
		NewDetectCmd(opts, deviceManager),
	)
	return cmd
}
