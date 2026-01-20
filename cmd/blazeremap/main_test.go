package main

import (
	"bytes"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestRun_Success(t *testing.T) {
	stderr := new(bytes.Buffer)

	// This is a basic test - full coverage requires integration tests
	// because NewApp() creates real dependencies
	exitCode := run(stderr)

	// If no controllers are connected, this should still succeed
	// (the detect command succeeds even with 0 controllers)
	assert.Equal(t, 0, exitCode)
	assert.Empty(t, stderr.String())
}

func TestRun_ErrorOutput(t *testing.T) {
	// This test shows the structure, but is hard to trigger
	// without mocking App creation

	// In practice, you'd test this at the integration level
	t.Skip("Requires integration test with binary")
}
