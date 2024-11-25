package main

func useAfterClosePanic() {
	ch := make(chan int)
	close(ch)    // Close the channel
	ch <- 1      // Sending into a closed channel, triggers panic
}

func main() {
	useAfterClosePanic()
}
