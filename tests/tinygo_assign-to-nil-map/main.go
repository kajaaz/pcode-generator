package main

func nilMapPanic() {
	var m map[string]int // nil map
	m["key"] = 42        // Assignment to entry in nil map, triggers panic
}

func main() {
	nilMapPanic()
}
