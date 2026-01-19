package cli

import (
	"github.com/spf13/cobra"
)

const AppVersion = "0.1"

type Options struct {
	Version bool
}

func NewRootCmd(opts *Options) *cobra.Command {
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
	return cmd
}
