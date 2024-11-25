package main

func chanMakePanic() {
	ch := make(chan int, int(^uint(0)>>1)) // Channel size too big, triggers panic
	_ = ch
}

func main() {
	chanMakePanic()
}
