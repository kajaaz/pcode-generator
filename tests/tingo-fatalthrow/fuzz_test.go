// +build go1.18

package main

import "testing"

func FuzzRecursiveOverflow(f *testing.F) {
	// Seed with typical inputs, including potentially dangerous depths
	f.Add(10)
	f.Add(100)
	f.Add(1000)
	f.Add(10000)
	
	// Fuzzing function to provide random depth values
	f.Fuzz(func(t *testing.T, depth int) {
		if depth < 1 || depth > 1000000 { // Limit depth to prevent uncontrolled crashes
			return
		}
		recursiveOverflow(depth)
	})
}
