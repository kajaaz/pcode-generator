//go:build go1.18
// +build go1.18

package brokencalc

import "testing"

func FuzzRunCalculator(f *testing.F) {
    // Optional seeds
    f.Add("1", "+", "2")
    f.Add("5", "*", "9") // might help discover the '5' crash

    // The fuzz target: tries random strings for num1, operator, num2
    f.Fuzz(func(t *testing.T, num1Str, op, num2Str string) {
        // Just call the code that checks ASCII digits and runs coreEngine
        _ = RunCalculator(num1Str, op, num2Str)
    })
}
