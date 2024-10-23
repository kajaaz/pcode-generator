package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strings"

	"github.com/ethereum/go-ethereum/common/bitutil"
)

func main() {
	// Get user input for the data to be compressed and decompressed
	fmt.Println("Enter some data to compress:")
	reader := bufio.NewReader(os.Stdin)
	input, err := reader.ReadString('\n')
	if err != nil {
		log.Fatalf("Error reading input: %v\n", err)
	}
	input = strings.TrimSpace(input) 

	// Convert the input string to bytes
	data := []byte(input)

	// Compress the data
	// compressed := bitutil.CompressBytes(data)
	// fmt.Printf("Compressed Data: %v\n", compressed)

	// Decompress the data
	decompressed, err := bitutil.DecompressBytes(data, len(data) + 20)
	if err != nil {
		log.Fatalf("Decompression error: %v\n", err)
	}
	fmt.Printf("Decompressed Data: %v\n", string(decompressed))

	// Check if decompression matches the original input
	if string(decompressed) == string(data) {
		fmt.Println("Decompression successful, data matches the original!")
	} else {
		fmt.Println("Decompression failed, data does not match the original!")
	}
}