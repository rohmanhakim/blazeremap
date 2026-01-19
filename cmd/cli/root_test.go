package cli_test

import (
	"testing"

	"blazeremap.com/blazeremap/cmd/cli"
	"github.com/stretchr/testify/assert"
)

func TestVersionFlag(t *testing.T) {
	opts := &cli.Options{}
	cmd := cli.NewRootCmd(opts)

	output, err := cli.ExecuteCommand(cmd, "--version")

	assert.NoError(t, err)
	assert.Equal(t, "BlazeRemap 0.1\n", output)

	output, err = cli.ExecuteCommand(cmd, "-v")

	assert.NoError(t, err)
	assert.Equal(t, "BlazeRemap 0.1\n", output)
}
