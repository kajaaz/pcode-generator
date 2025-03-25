package main

import (
    "fmt"
    "os"
    "broken-calculator-bis-fuzzing/brokencalc" 
)

func main() {
    if len(os.Args) != 4 {
        fmt.Println("Usage: ./broken-calculator [num1] [operator] [num2]")
        return
    }
    err := brokencalc.RunCalculator(os.Args[1], os.Args[2], os.Args[3])
    if err != nil {
        fmt.Println(err)
    }
}
