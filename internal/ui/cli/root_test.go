// root_test.go
package cli_test

import (
	"testing"

	"blazeremap.com/blazeremap/internal/ui/cli"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

func TestVersionFlag(t *testing.T) {
	tests := []struct {
		name     string
		args     []string
		expected string
	}{
		{
			name:     "long version flag",
			args:     []string{"--version"},
			expected: "BlazeRemap 0.1\n",
		},
		{
			name:     "short version flag",
			args:     []string{"-v"},
			expected: "BlazeRemap 0.1\n",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			opts := &cli.Options{}
			mockDM := cli.NewMockDeviceManager()
			cmd := cli.NewRootCmd(opts, mockDM)

			output, err := cli.ExecuteCommand(cmd, tt.args...)

			require.NoError(t, err)
			assert.Equal(t, tt.expected, output)
		})
	}
}

func TestRootCommandWithNoArgs(t *testing.T) {
	opts := &cli.Options{}
	mockDM := cli.NewMockDeviceManager()
	cmd := cli.NewRootCmd(opts, mockDM)

	output, err := cli.ExecuteCommand(cmd)

	assert.NoError(t, err)
	// Root command with no args should show help or do nothing (depending on your design)
	// Adjust this assertion based on your intended behavior
	assert.Empty(t, output) // Currently does nothing
}

func TestDetectSubcommandExists(t *testing.T) {
	opts := &cli.Options{}
	mockDM := cli.NewMockDeviceManager()
	cmd := cli.NewRootCmd(opts, mockDM)

	// Check that detect subcommand exists
	detectCmd, _, err := cmd.Find([]string{"detect"})

	require.NoError(t, err)
	assert.NotNil(t, detectCmd)
	assert.Equal(t, "detect", detectCmd.Use)
}

func TestInvalidFlag(t *testing.T) {
	opts := &cli.Options{}
	mockDM := cli.NewMockDeviceManager()
	cmd := cli.NewRootCmd(opts, mockDM)

	_, err := cli.ExecuteCommand(cmd, "--invalid-flag")

	assert.Error(t, err)
	assert.Contains(t, err.Error(), "unknown flag")
}
