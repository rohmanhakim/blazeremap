package main

import (
	"fmt"
	"io"
	"os"

	"blazeremap.com/blazeremap/internal"
)

func main() {
	os.Exit(run(os.Stderr))
}

// run is the testable entry point that doesn't call os.Exit
func run(stderr io.Writer) int {
	app := internal.NewApp()
	if err := app.BindCli(); err != nil {
		fmt.Fprintf(stderr, "Error: %v\n", err)
		return 1
	}
	return 0
}
