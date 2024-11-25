package main

func negativeShiftPanic() {
	var x int = 1
	var y int = -1
	_ = x << y // Negative shift, triggers panic
}

func main() {
	negativeShiftPanic()
}
