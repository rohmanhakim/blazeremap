package internal_test

import (
	"errors"
	"testing"

	"blazeremap.com/blazeremap/internal"
	"blazeremap.com/blazeremap/internal/device/controller"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

// Test the production NewApp constructor
func TestNewApp(t *testing.T) {
	t.Run("creates app with production dependencies", func(t *testing.T) {
		app := internal.NewApp()

		require.NotNil(t, app)
		// App should be created successfully
		// We can't inspect internal fields, but we can test behavior
	})

	t.Run("app is ready to execute", func(t *testing.T) {
		app := internal.NewApp()

		require.NotNil(t, app)
		// Should not panic when calling BindCli
		// We won't actually execute to avoid needing real devices
	})
}

// Test the test constructor NewTestApp
func TestNewTestApp(t *testing.T) {
	t.Run("creates app with injected dependencies", func(t *testing.T) {
		mockDM := internal.NewMockDeviceManager()
		mockCli := internal.NewMockCli()

		app := internal.NewTestApp(mockDM, mockCli)

		require.NotNil(t, app)
	})

	t.Run("uses injected device manager", func(t *testing.T) {
		mockDM := internal.NewMockDeviceManager()
		mockCli := internal.NewMockCli()

		app := internal.NewTestApp(mockDM, mockCli)
		err := app.BindCli()

		assert.NoError(t, err)
		assert.True(t, mockCli.ExecuteCalled, "CLI Execute should have been called")
	})

	t.Run("propagates cli execution errors", func(t *testing.T) {
		mockDM := internal.NewMockDeviceManager()
		expectedErr := errors.New("cli execution failed")
		mockCli := internal.NewMockCli().WithError(expectedErr)

		app := internal.NewTestApp(mockDM, mockCli)
		err := app.BindCli()

		assert.Error(t, err)
		assert.Equal(t, expectedErr, err)
	})
}

// Test BindCli method
func TestApp_BindCli(t *testing.T) {
	t.Run("executes cli successfully", func(t *testing.T) {
		mockDM := internal.NewMockDeviceManager()
		mockCli := internal.NewMockCli()

		app := internal.NewTestApp(mockDM, mockCli)
		err := app.BindCli()

		assert.NoError(t, err)
		assert.True(t, mockCli.ExecuteCalled)
	})

	t.Run("returns error from cli execution", func(t *testing.T) {
		mockDM := internal.NewMockDeviceManager()
		expectedErr := errors.New("command failed")
		mockCli := internal.NewMockCli().WithError(expectedErr)

		app := internal.NewTestApp(mockDM, mockCli)
		err := app.BindCli()

		assert.Error(t, err)
		assert.Equal(t, expectedErr, err)
	})

	t.Run("can be called multiple times", func(t *testing.T) {
		mockDM := internal.NewMockDeviceManager()
		mockCli := internal.NewMockCli()

		app := internal.NewTestApp(mockDM, mockCli)

		// First call
		err1 := app.BindCli()
		assert.NoError(t, err1)

		// Reset mock
		mockCli.ExecuteCalled = false

		// Second call
		err2 := app.BindCli()
		assert.NoError(t, err2)
		assert.True(t, mockCli.ExecuteCalled)
	})
}

// Integration-style tests
func TestApp_Integration(t *testing.T) {
	t.Run("detect command with no controllers", func(t *testing.T) {
		mockDM := internal.NewMockDeviceManager()
		mockCli := internal.NewMockCliWithCommand()

		app := internal.NewTestApp(mockDM, mockCli)
		mockCli.Cmd.SetArgs([]string{"detect"})

		err := app.BindCli()

		assert.NoError(t, err)
		output := mockCli.GetOutput()
		assert.Contains(t, output, "Found 0 controller(s)")
	})

	t.Run("detect command with controllers", func(t *testing.T) {
		mockDM := internal.NewMockDeviceManager().
			WithControllers(
				internal.NewMockControllerInfo(
					"Xbox Controller",
					"/dev/input/event3",
					controller.ControllerTypeXboxOne,
				),
			)
		mockCli := internal.NewMockCliWithCommand()

		app := internal.NewTestApp(mockDM, mockCli)
		mockCli.Cmd.SetArgs([]string{"detect"})

		err := app.BindCli()

		assert.NoError(t, err)
		output := mockCli.GetOutput()
		assert.Contains(t, output, "Found 1 controller(s)")
		assert.Contains(t, output, "Xbox Controller")
	})

	t.Run("version flag", func(t *testing.T) {
		mockDM := internal.NewMockDeviceManager()
		mockCli := internal.NewMockCliWithCommand()

		app := internal.NewTestApp(mockDM, mockCli)
		mockCli.Cmd.SetArgs([]string{"--version"})

		err := app.BindCli()

		assert.NoError(t, err)
		output := mockCli.GetOutput()
		assert.Contains(t, output, "FlexRemap")
		assert.Contains(t, output, "0.1")
	})

	t.Run("device manager error propagates", func(t *testing.T) {
		expectedErr := errors.New("failed to enumerate devices")
		mockDM := internal.NewMockDeviceManager().WithError(expectedErr)
		mockCli := internal.NewMockCliWithCommand()

		app := internal.NewTestApp(mockDM, mockCli)
		mockCli.Cmd.SetArgs([]string{"detect"})

		err := app.BindCli()

		assert.Error(t, err)
		assert.Equal(t, expectedErr, err)
	})
}

// Test that App properly isolates dependencies
func TestApp_DependencyIsolation(t *testing.T) {
	t.Run("different apps use different dependencies", func(t *testing.T) {
		mockDM1 := internal.NewMockDeviceManager()
		mockCli1 := internal.NewMockCli()
		app1 := internal.NewTestApp(mockDM1, mockCli1)

		mockDM2 := internal.NewMockDeviceManager()
		mockCli2 := internal.NewMockCli()
		app2 := internal.NewTestApp(mockDM2, mockCli2)

		// Execute first app
		err1 := app1.BindCli()
		assert.NoError(t, err1)
		assert.True(t, mockCli1.ExecuteCalled)
		assert.False(t, mockCli2.ExecuteCalled, "Second app's CLI should not be called")

		// Execute second app
		err2 := app2.BindCli()
		assert.NoError(t, err2)
		assert.True(t, mockCli2.ExecuteCalled)
	})

	t.Run("test app does not affect production app", func(t *testing.T) {
		// Create test app with mocks
		mockDM := internal.NewMockDeviceManager()
		mockCli := internal.NewMockCli()
		testApp := internal.NewTestApp(mockDM, mockCli)

		// Create production app
		prodApp := internal.NewApp()

		// Both should be independent
		require.NotNil(t, testApp)
		require.NotNil(t, prodApp)
	})
}

// Test error scenarios
func TestApp_ErrorHandling(t *testing.T) {
	t.Run("handles nil device manager gracefully", func(t *testing.T) {
		// This test verifies that passing nil doesn't panic
		// In production, this shouldn't happen, but defensive programming is good
		mockCli := internal.NewMockCli()

		// This should not panic
		app := internal.NewTestApp(nil, mockCli)
		require.NotNil(t, app)

		// Executing might fail, but shouldn't panic
		// (actual behavior depends on your implementation)
	})

	t.Run("handles nil cli gracefully", func(t *testing.T) {
		mockDM := internal.NewMockDeviceManager()

		// This should not panic
		app := internal.NewTestApp(mockDM, nil)
		require.NotNil(t, app)
	})
}

// Benchmark tests
func BenchmarkNewApp(b *testing.B) {
	for i := 0; i < b.N; i++ {
		_ = internal.NewApp()
	}
}

func BenchmarkNewTestApp(b *testing.B) {
	mockDM := internal.NewMockDeviceManager()
	mockCli := internal.NewMockCli()

	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		_ = internal.NewTestApp(mockDM, mockCli)
	}
}

func BenchmarkBindCli(b *testing.B) {
	mockDM := internal.NewMockDeviceManager()
	mockCli := internal.NewMockCli()
	app := internal.NewTestApp(mockDM, mockCli)

	b.ResetTimer()
	for i := 0; b.Loop(); i++ {
		_ = app.BindCli()
	}
}
