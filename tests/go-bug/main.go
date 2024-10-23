package main

import (
	"fmt"
	"os"
	"os/exec"
)

func handleSensitiveFile() error {
	// Open a sensitive file (e.g., in this case, it could be a system file like /etc/passwd)
	file, err := os.Open("/etc/passwd")
	if err != nil {
		return fmt.Errorf("failed to open file: %w", err)
	}
	defer file.Close()

	// Perform some operations with the file
	_, err = file.Stat()
	if err != nil {
		return fmt.Errorf("failed to stat file: %w", err)
	}

	// Here we use the file descriptor for some low-level operations
	fd := file.Fd()

	// Simulate accessing the file descriptor via /proc/self/fd/<fd>
	_, err = os.Open(fmt.Sprintf("/proc/self/fd/%d", fd))

	if err != nil {
		return fmt.Errorf("failed to access file via file descriptor: %w", err)
	}

	return nil
}

func runCommand() error {
	// Execute some command (as an example, we run "ls" on the current directory)
	cmd := exec.Command("sh", "-c", "ls -alh")
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr
	err := cmd.Run()
	if err != nil {
		return fmt.Errorf("command failed: %w", err)
	}
	
	return nil
}

func main() {
	// Process some sensitive files
	err := handleSensitiveFile()
	if err != nil {
		fmt.Println("Error handling sensitive file:", err)
	}

	// Run some external command
	err = runCommand()
	if err != nil {
		fmt.Println("Error running command:", err)
	}

	// Program completes normally
	fmt.Println("Program completed.")
}
