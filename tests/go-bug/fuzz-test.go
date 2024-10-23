package main

import (
	"fmt"
	"os"
	"os/exec"
	"testing"
)

func FuzzHandleSensitiveFile(f *testing.F) {
	// Seed corpus with some initial values for fuzzing
	f.Add("/etc/passwd")
	f.Add("/proc/cpuinfo")
	f.Add("/dev/null")
	f.Add("invalid/path")

	// Fuzzing function: Fuzz different file paths to simulate file descriptor handling
	f.Fuzz(func(t *testing.T, filePath string) {
		// Simulate handling a sensitive file with the fuzzed filePath
		file, err := os.Open(filePath)
		if err != nil {
			// Invalid file paths can happen, so return early if we can't open the file
			t.Skip()
			return
		}
		defer file.Close()

		// Perform the file descriptor leak scenario by accessing /proc/self/fd
		fd := file.Fd()
		_, err = os.Open(fmt.Sprintf("/proc/self/fd/%d", fd))
		if err != nil {
			t.Fatalf("Failed to access file via file descriptor: %v", err)
		}
	})
}

func FuzzRunCommand(f *testing.F) {
	// Seed corpus with some initial commands for fuzzing
	f.Add("ls -alh")
	f.Add("cat /etc/passwd")
	f.Add("echo 'Hello, world!'")
	f.Add("invalidcommand")

	// Fuzzing function: Fuzz different shell commands
	f.Fuzz(func(t *testing.T, command string) {
		// Run the fuzzed command
		cmd := exec.Command("sh", "-c", command)
		cmd.Stdout = os.Stdout
		cmd.Stderr = os.Stderr

		// Run the command and check for any errors
		err := cmd.Run()
		if err != nil {
			// Command failed, but that's expected for some fuzzed inputs
			t.Skip()
		}
	})
}
