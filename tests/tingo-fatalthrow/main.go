package main

import (
	"fmt"
	"os"
	"strconv"
)

// Function to induce stack overflow without direct unsafe calls
func recursiveOverflow(input int) {
	if input <= 0 {
		return
	}
	fmt.Println("Recursion depth:", input)
	recursiveOverflow(input - 1) // Recursive call without termination for deep stack
}

func main() {
	// Accept command-line input to control recursion depth
	if len(os.Args) < 2 {
		fmt.Println("Please provide a recursion depth as an argument")
		return
	}
	input, err := strconv.Atoi(os.Args[1])
	if err != nil || input < 1 {
		fmt.Println("Invalid input; please provide a positive integer")
		return
	}

	fmt.Println("Starting recursive overflow with depth:", input)
	recursiveOverflow(input)
}
