package main

import (
	"sync"
	"testing"
	"time"
)

func FuzzCounter(f *testing.F) {
	f.Fuzz(func(t *testing.T, countOps int, shouldReset bool) {
		var counter Counter
		var wg sync.WaitGroup

		// Bound the input to avoid excessive operations
		if countOps < 1 || countOps > 100 {
			t.Skip() // Skip invalid or excessive operations
		}

		// Goroutine to increment and decrement
		wg.Add(1)
		go func() {
			defer wg.Done()

			for i := 0; i < countOps; i++ {
				counter.Increment()
				time.Sleep(10 * time.Millisecond)
			}
			counter.Decrement()
		}()

		// Optionally reset the counter in another goroutine
		if shouldReset {
			wg.Add(1)
			go func() {
				defer wg.Done()
				time.Sleep(50 * time.Millisecond)
				counter.Reset() // This may trigger a double unlock panic
			}()
		}

		// Wait for all goroutines to finish
		wg.Wait()

		t.Logf("Final counter value: %d", counter.count)
	})
}
