package main

import (
    "fmt"
    "unsafe"
)

func main() {
    var x int = 42
    p := unsafe.Pointer(&x)
    p = unsafe.Pointer(uintptr(p) + 1) // Improper pointer manipulation
    fmt.Println(*(*int)(p))            // Accessing invalid memory, leads to runtime error
}
