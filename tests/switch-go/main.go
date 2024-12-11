package main

import (
	"fmt"
	"os"
	"runtime"
	"strconv"
)

func main() {
    runtime.GOMAXPROCS(1)
    // Check if the correct number of arguments are provided
    if len(os.Args) < 2 {
        fmt.Println("Usage: ./switch-tinygo <number>")
        os.Exit(1)
    }

    x, err := strconv.Atoi(os.Args[1])
    if err != nil {
        fmt.Println("Invalid number. Please provide a valid integer.")
        os.Exit(1)
    }

    // Handle cases with a switch statement
    switch x {
    case 1:
        fmt.Println("One")
    case 5:
        fmt.Println("Five")
    case 10:
        fmt.Println("Ten")
    default:
        fmt.Println("Other")
    }
}
