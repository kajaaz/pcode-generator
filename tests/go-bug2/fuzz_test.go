package main

import (
	"testing"
)

func FuzzChannelSend(f *testing.F) {
	// Seed the fuzzing test with initial values
	f.Add(0) 

	f.Fuzz(func(t *testing.T, value int) {
		// Create a channel
		ch := make(chan int)

		// Close the channel
		close(ch)

		// Attempt to send a value to the closed channel
		// The value being sent is fuzzed
		ch <- value
	})
}
