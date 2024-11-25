package main

import (
	"fmt"
	"unsafe"
)

func recursiveFrameSizeManipulation(depth int, _ *[]byte) {
	if depth == 0 {
		return
	}

	// Introducing dynamic allocation within recursion
	tmp := make([]byte, depth*10)

	// Unsafe pointer manipulation to simulate the misalignment
	ptr := unsafe.Pointer(&tmp[0])
	*(*uintptr)(ptr) ^= uintptr(unsafe.Sizeof(ptr))

	// Recurse with manipulated frame size potential
	recursiveFrameSizeManipulation(depth-1, &tmp)
}

func main() {
	fmt.Println("Starting frameSizeMismatch demo...")

	// Trigger the recursive function with a large depth
	data := make([]byte, 10)
	recursiveFrameSizeManipulation(50, &data)

	fmt.Println("Finished frameSizeMismatch demo.")
}
