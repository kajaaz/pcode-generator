package main

import (
	"fmt"
	"log"
	"os"
	"strings"

	"github.com/ethereum/go-ethereum/common/bitutil"
)

func main() {
	if len(os.Args) < 2 {
		fmt.Println("Usage: ./tinygo-compress <input>")
		return
	}

	input := os.Args[1]
	input = strings.TrimSpace(input)

	// Convert the input string to bytes
	data := []byte(input)

	// Compress the data
	compressed := bitutil.CompressBytes(data)
	fmt.Printf("Compressed Data: %v\n", compressed)

	// Decompress the data using the compressed output
	decompressed, err := bitutil.DecompressBytes(compressed, len(data))
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
