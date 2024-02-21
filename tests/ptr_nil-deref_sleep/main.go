package main

import (
    "fmt"
    "time"
)

func main() {
    var ptr *int

    fmt.Println("Sleeping for 5 minutes...")
    time.Sleep(5 * time.Minute)

    fmt.Println(*ptr)
}
