package main

import "runtime"

func main() {
	runtime.GOMAXPROCS(1)
	a := 1
	b := 2
	c := a + b
	println(c)
}
