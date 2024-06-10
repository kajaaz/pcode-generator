package main

import "fmt"

func main() {
    a := 1
    b := 2
    c := a + b
	var p *int = nil   // Create a nil pointer of type *int
    fmt.Println(*p)    // Dereference the nil pointer, which will cause a runtime panic
    println(c)
}
