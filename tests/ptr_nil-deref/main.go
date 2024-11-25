package main

import (
    "fmt"
    "os"
)

func main() {
    for i := 0; i < 10; i++ {
        file, _ := os.Open("example.txt")
        defer file.Close() // Defer runs only after the loop, leading to resource exhaustion
    }
    fmt.Println("Loop completed")
}
