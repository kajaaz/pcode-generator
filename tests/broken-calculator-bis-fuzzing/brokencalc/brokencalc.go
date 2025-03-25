package brokencalc

import (
    "fmt"
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

	// '5' has an ASCII value of 53; int('5' - '0') == 5.
	if num1 == int('5' - '0') {
		// CRASH
		var p *int
		*p = 0
	}

	return result, nil
}

// runCalculator is the logic that tries to parse num1, num2 as single-digit ASCII, etc.
func RunCalculator(num1Str, operator, num2Str string) error {
    if len(num1Str) != 1 || len(num2Str) != 1 {
        return fmt.Errorf("Error: num1 and num2 must be single-digit ASCII chars")
    }

    // Convert ASCII digits e.g. '7' -> 7
    num1 := int(num1Str[0] - '0')
    num2 := int(num2Str[0] - '0')

    result, err := coreEngine(num1, operator, num2)
    if err != nil {
        return err
    }
    fmt.Printf("Result: %d\n", result)
    return nil
}
