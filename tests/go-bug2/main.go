package main

func main() {
	// Create a channel
	ch := make(chan int)

	// Close the channel
	close(ch)

	// Attempt to send a value to the closed channel
	// This will cause a panic because the channel is closed
	ch <- 1
}
