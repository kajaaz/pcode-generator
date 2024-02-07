package main

import (
	"fmt"
)

func main() {
	var x, y *int // Changed to pointers
	fmt.Print("Enter two numbers (separated by space): ")
	// Assuming input is being simulated or abstracted away, not reading into pointers directly for simplicity
	var xInput, yInput int
	fmt.Scanf("%d %d", &xInput, &yInput)

	x = &xInput // Assign addresses to pointers
	y = &yInput

	sumPtr := add(x, y) // Use a function that returns a pointer to the sum
	fmt.Println("Sum:", *sumPtr) // Dereference the result to print it

	if *sumPtr > 10 { // Dereference to compare
		fmt.Println("Sum is greater than 10.")
	} else {
		fmt.Println("Sum is less than or equal to 10.")
	}

	for i := 0; i < 5; i++ {
		fmt.Println("Iteration:", i)
	}

	product := multiply(*x, *y) // Dereference pointers to get their values
	fmt.Println("Product:", product)

	// Introducing potential nil dereference
	var nilPointer *int
	fmt.Println("This will cause a nil dereference error:", *nilPointer) // Deliberate nil dereference
}

func add(a, b *int) *int {
	result := *a + *b // Dereference pointers to add their values
	return &result // Return a pointer to the result
}

func multiply(a, b int) int {
	return a * b
}
