package main

import (
	"fmt"
	"os"
)

func main() {
	if len(os.Args) != 2 {
		fmt.Println("Usage: ./crashme <input_file>")
		return
	}

	// Read file input
	data, err := os.ReadFile(os.Args[1])
	if err != nil {
		fmt.Println("Error reading file:", err)
		return
	}

	// Check if input contains 'C'
	if len(data) > 0 && data[0] == 'C' {
		var p *int
		*p = 0 // Intentional crash
	}

	fmt.Println("OK")
}
