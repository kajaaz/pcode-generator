package main

import "fmt"

func computeSum(limit int) int {
	sum := 0
	for i := 1; i <= limit; i++ {
		if i%2 == 0 {
			sum += i
		} else {
			sum += 2 * i
		}
	}
	return sum
}

func main() {
	fmt.Println("Sum computation:")
	result := computeSum(10)
	fmt.Printf("The sum is: %d\n", result)
}
