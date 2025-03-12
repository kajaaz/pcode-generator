package main

import (
	"fmt"
	"os"
)

// coreEngine performs integer calculations and returns the result.
func coreEngine(num1 int, operator string, num2 int) (int, error) {
	var result int

	switch operator {
	case "+":
		result = num1 + num2
	case "-":
		result = num1 - num2
	case "*":
		result = num1 * num2
	case "/":
		if num2 == 0 {
			return 0, fmt.Errorf("error: division by zero is not allowed")
		}
		result = num1 / num2
	default:
		return 0, fmt.Errorf("error: unsupported operator. Use one of +, -, *, /")
	}

	// '5' has an ASCII value of 53 - Converting it by subtracting '0' yields 5.
	if num1 == int('5'-'0') {
		var p *int
		*p = 0
	}

	return result, nil
}

func main() {
	if len(os.Args) != 4 {
		fmt.Println("Usage: ./broken-calculator-tinygo [num1] [operator] [num2]")
		return
	}

	// Validate that both num1 and num2 are single characters.
	if len(os.Args[1]) != 1 || len(os.Args[3]) != 1 {
		fmt.Println("Error: Both num1 and num2 must be single digit characters.")
		return
	}

	// Convert the ASCII digit to its integer value.
	num1 := int(os.Args[1][0] - '0')
	operator := os.Args[2]
	num2 := int(os.Args[3][0] - '0')

	result, err := coreEngine(num1, operator, num2)
	if err != nil {
		fmt.Println(err)
		return
	}

	fmt.Printf("Result: %d\n", result)
}
