package main

import (
	"fmt"
	"sync"
	"time"
)

type Counter struct {
	count int
	lock  sync.Mutex
}

func (c *Counter) Increment() {
	// Lock the mutex before incrementing
	c.lock.Lock()

	// Increment the counter
	c.count++

	// Unlock the mutex after incrementing
	c.lock.Unlock()
}

func (c *Counter) Decrement() {
	// Lock the mutex before decrementing
	c.lock.Lock()

	// Decrement the counter
	c.count--

	// Unlock the mutex after decrementing
	c.lock.Unlock()
}

func (c *Counter) Reset() {
	// Lock the mutex before resetting
	c.lock.Lock()

	// Reset the counter
	c.count = 0

	// Unlock the mutex after resetting
	c.lock.Unlock()

	// Introduce the bug: Unlock the mutex again without locking
	// This will cause a panic when executed
	c.lock.Unlock() // Double unlock - this will trigger a panic
}

func main() {
	// Create a shared counter
	counter := &Counter{}

	// Create a wait group to manage goroutines
	var wg sync.WaitGroup

	// Start multiple goroutines to increment and decrement the counter
	for i := 0; i < 5; i++ {
		wg.Add(1)
		go func(id int) {
			defer wg.Done()

			// Simulate work: Increment the counter
			for j := 0; j < 10; j++ {
				counter.Increment()
				time.Sleep(100 * time.Millisecond)
			}

			// Decrement the counter
			counter.Decrement()

			fmt.Printf("Goroutine %d finished\n", id)
		}(i)
	}

	// Start another goroutine to reset the counter after some time
	wg.Add(1)
	go func() {
		defer wg.Done()

		time.Sleep(500 * time.Millisecond)
		counter.Reset() // This will eventually cause the program to panic
		fmt.Println("Counter reset")
	}()

	// Wait for all goroutines to complete
	wg.Wait()

	// Print the final counter value (if the program hasn't panicked)
	fmt.Println("Final counter value:", counter.count)
}
