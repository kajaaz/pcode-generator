package main

import (
	"fmt"
	"log"
	"runtime"

	"github.com/ethereum/go-ethereum/rlp"
)

type MyData struct {
	Field1 uint64
	Field2 string
}

func main() {
	runtime.GOMAXPROCS(1)
	
	// Step 1: Create an instance of the struct to be encoded
	data := MyData{
		Field1: 12345,
		Field2: "hello",
	}

	// Step 2: Encode the struct to RLP-encoded bytes
	encodedBytes, err := rlp.EncodeToBytes(data)
	if err != nil {
		log.Fatalf("Failed to encode to RLP: %v", err)
	}
	fmt.Printf("RLP Encoded Data: %x\n", encodedBytes)

	// Step 3: Decode the RLP-encoded bytes back to the original struct
	var decodedData MyData
	err = rlp.DecodeBytes(encodedBytes, &decodedData)
	if err != nil {
		log.Fatalf("Failed to decode RLP: %v", err)
	}

	// Step 4: Print the decoded struct to verify it matches the original data
	fmt.Printf("Decoded Data: %+v\n", decodedData)
}
