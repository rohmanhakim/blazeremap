package internal

import (
	"blazeremap.com/blazeremap/internal/device"
	"blazeremap.com/blazeremap/internal/platform"
	"blazeremap.com/blazeremap/internal/ui/cli"
)

type App struct {
	deviceManager device.DeviceManager
	cli           cli.Cli
}

func (a *App) BindCli() error {
	err := a.cli.Execute()
	return err
}

// NewApp creates an App with production dependencies.
// For testing, use NewTestApp() to inject mocks.
func NewApp() *App {
	manager := platform.NewDeviceManager()
	return &App{
		deviceManager: manager,
		cli:           cli.NewRootCmd(&cli.Options{}, manager),
	}
}

// NewTestApp creates an App with injected dependencies for testing.
func NewTestApp(manager device.DeviceManager, cli cli.Cli) *App {
	return &App{
		deviceManager: manager, // Inject mock
		cli:           cli,     // Inject mock
	}
}
