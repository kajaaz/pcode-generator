package main

import (
    "fmt"
    "os"
    "strconv"
)

func main() {
    // this is so that the Go compiler doesn't optimize drasticvally the code
    if os.Args[1] == "" {
        fmt.Println("Please provide a number (1, 5, 10, or other)")
        os.Exit(1)
    }
    x, err := strconv.Atoi(os.Args[1])
    if err != nil {
        fmt.Println("Invalid number")
        os.Exit(1)
    }

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
