package main

import "fmt"

func main() {	
	a := 1
	b := 2
	c := a + b
	var p *int = nil   
    fmt.Println(*p)	// Dereference the pointer
	println(c)
}
