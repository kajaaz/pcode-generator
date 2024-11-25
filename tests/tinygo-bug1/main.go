package main

import (
    "fmt"
    "sync/atomic"
)

func main() {
    var counter int64 = 0

    atomic.AddInt64(&counter, 1) // Correct atomic increment

    fmt.Println("Counter:", counter)
}
