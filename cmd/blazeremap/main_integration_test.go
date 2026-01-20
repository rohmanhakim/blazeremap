//go:build integration
// +build integration

package main_test

import (
	"os/exec"
	"testing"

	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

func TestBinary_Version(t *testing.T) {
	cmd := exec.Command("go", "run", ".", "--version")
	output, err := cmd.CombinedOutput()

	require.NoError(t, err)
	assert.Contains(t, string(output), "FlexRemap 0.1")
}

func TestBinary_Help(t *testing.T) {
	cmd := exec.Command("go", "run", ".", "--help")
	output, err := cmd.CombinedOutput()

	require.NoError(t, err)
	assert.Contains(t, string(output), "Usage:")
}

func TestBinary_ExitCode_Success(t *testing.T) {
	cmd := exec.Command("go", "run", ".", "--version")
	err := cmd.Run()

	assert.NoError(t, err, "Should exit with code 0")
}

func TestBinary_ExitCode_Error(t *testing.T) {
	cmd := exec.Command("go", "run", ".", "--invalid-flag")
	err := cmd.Run()

	assert.Error(t, err, "Should exit with non-zero code")
}
