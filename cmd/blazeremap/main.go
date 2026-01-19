package main

import (
	"os"

	"blazeremap.com/blazeremap/cmd/cli"
)

func main() {

	opts := &cli.Options{}
	cmd := cli.NewRootCmd(opts)

	if err := cmd.Execute(); err != nil {
		os.Exit(1)
	}
}
