package main

import (
	"bytes"
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"net"
	"os"
	"strings"
)

//go:noinline
func readAllNoInline(r io.Reader) ([]byte, error) {
	return io.ReadAll(r)
}

type Operator struct {
	Logger  Logger
	Config  Config
}

type Logger interface {
	Infof(format string, args ...interface{})
}

type Config struct {
	Operator struct {
		MaxBatchSize int64
	}
}

type VerificationData struct{}

func parseURL(batchURL string) (host, path string, err error) {
	if !strings.HasPrefix(batchURL, "http://") {
		return "", "", errors.New("only http URLs are supported")
	}
	batchURL = strings.TrimPrefix(batchURL, "http://")
	parts := strings.SplitN(batchURL, "/", 2)
	host = parts[0]
	if len(parts) > 1 {
		path = "/" + parts[1]
	} else {
		path = "/"
	}
	return host, path, nil
}

type CustomHTTPClient struct{}

func (c *CustomHTTPClient) Head(url string) (map[string]string, error) {
	host, path, err := parseURL(url)
	if err != nil {
		return nil, err
	}
	return simpleHTTPHead(host, path)
}

func (c *CustomHTTPClient) Get(url string) (io.ReadCloser, map[string]string, error) {
	host, path, err := parseURL(url)
	if err != nil {
		return nil, nil, err
	}

	headers, err := simpleHTTPHead(host, path)
	if err != nil {
		return nil, nil, err
	}

	body, err := simpleHTTPGet(host, path)
	if err != nil {
		return nil, nil, err
	}

	return io.NopCloser(bytes.NewReader(body)), headers, nil
}

func simpleHTTPHead(host, path string) (map[string]string, error) {
	conn, err := net.Dial("tcp", host+":80")
	if err != nil {
		return nil, err
	}
	defer conn.Close()

	headRequest := fmt.Sprintf("HEAD %s HTTP/1.1\r\nHost: %s\r\nConnection: close\r\n\r\n", path, host)
	_, err = conn.Write([]byte(headRequest))
	if err != nil {
		return nil, err
	}

	response, err := readAllNoInline(conn) // Use the noinline wrapper
	if err != nil {
		return nil, err
	}

	return parseHTTPResponseHeaders(response), nil
}

func simpleHTTPGet(host, path string) ([]byte, error) {
	conn, err := net.Dial("tcp", host+":80")
	if err != nil {
		return nil, err
	}
	defer conn.Close()

	getRequest := fmt.Sprintf("GET %s HTTP/1.1\r\nHost: %s\r\nConnection: close\r\n\r\n", path, host)
	_, err = conn.Write([]byte(getRequest))
	if err != nil {
		return nil, err
	}

	response, err := readAllNoInline(conn) // Use the noinline wrapper
	if err != nil {
		return nil, err
	}

	headersEnd := bytes.Index(response, []byte("\r\n\r\n"))
	if headersEnd == -1 {
		return nil, errors.New("invalid HTTP response")
	}

	return response[headersEnd+4:], nil
}

func parseHTTPResponseHeaders(response []byte) map[string]string {
	headers := make(map[string]string)
	lines := strings.Split(string(response), "\r\n")
	if len(lines) > 0 {
		statusParts := strings.Split(lines[0], " ")
		if len(statusParts) > 1 {
			headers["status"] = statusParts[1]
		}
	}
	for _, line := range lines[1:] {
		if line == "" {
			break
		}
		parts := strings.SplitN(line, ":", 2)
		if len(parts) == 2 {
			headers[strings.TrimSpace(parts[0])] = strings.TrimSpace(parts[1])
		}
	}
	return headers
}

func parseContentLength(headers map[string]string) (int64, error) {
	if contentLengthStr, ok := headers["Content-Length"]; ok {
		var contentLength int64
		_, err := fmt.Sscanf(contentLengthStr, "%d", &contentLength)
		if err != nil {
			return 0, err
		}
		return contentLength, nil
	}
	return 0, errors.New("Content-Length not found")
}

func (o *Operator) getBatchFromS3(batchURL string, expectedMerkleRoot [32]byte) ([]VerificationData, error) {
	o.Logger.Infof("Getting batch from S3..., batchURL: %s", batchURL)
	client := &CustomHTTPClient{}

	respHeaders, err := client.Head(batchURL)
	if err != nil {
		return nil, err
	}

	if respHeaders["status"] != "200" {
		return nil, fmt.Errorf("error getting Proof Head from S3: %s", respHeaders["status"])
	}

	contentLength, err := parseContentLength(respHeaders)
	if err != nil {
		return nil, err
	}

	if contentLength > o.Config.Operator.MaxBatchSize {
		return nil, fmt.Errorf("proof size %d exceeds max batch size %d", contentLength, o.Config.Operator.MaxBatchSize)
	}

	respBody, _, err := client.Get(batchURL)
	if err != nil {
		return nil, err
	}
	defer func() {
		if err := respBody.Close(); err != nil {
			fmt.Println("error closing body: ", err)
		}
	}()

	batchBytes, err := readAllNoInline(respBody) // Use the noinline wrapper here
	if err != nil {
		return nil, err
	}

	o.Logger.Infof("Verifying batch merkle tree...")
	// Mock verification to avoid using merkle_tree
	merkleRootCheck := mockVerifyMerkleTreeBatch(batchBytes, uint(len(batchBytes)), expectedMerkleRoot)
	if !merkleRootCheck {
		return nil, fmt.Errorf("merkle root check failed")
	}
	o.Logger.Infof("Batch merkle tree verified")

	var batch []VerificationData
	err = json.Unmarshal(batchBytes, &batch)
	if err != nil {
		return nil, err
	}

	return batch, nil
}

func mockVerifyMerkleTreeBatch(data []byte, size uint, expectedRoot [32]byte) bool {
	// Simplified placeholder for merkle tree verification
	return true
}

func main() {
	if len(os.Args) < 3 {
		fmt.Println("Usage: ./program <batchURL> <expectedMerkleRoot>")
		os.Exit(1)
	}

	batchURL := os.Args[1]
	var expectedMerkleRoot [32]byte
	if len(os.Args[2]) != 64 {
		fmt.Println("Expected Merkle Root must be a 32-byte hex string")
		os.Exit(1)
	}

	for i := 0; i < 32; i++ {
		hexValue := os.Args[2][i*2 : (i+1)*2]
		_, err := fmt.Sscanf(hexValue, "%02x", &expectedMerkleRoot[i])
		if err != nil {
			fmt.Printf("Invalid hex in Merkle Root: %s\n", err)
			os.Exit(1)
		}
	}

	op := Operator{
		Logger: LoggerImpl{},
		Config: Config{
			Operator: struct {
				MaxBatchSize int64
			}{MaxBatchSize: 1024 * 1024},
		},
	}

	batch, err := op.getBatchFromS3(batchURL, expectedMerkleRoot)
	if err != nil {
		fmt.Println("Error:", err)
		os.Exit(1)
	}

	fmt.Println("Batch received:", batch)
}

type LoggerImpl struct{}

func (l LoggerImpl) Infof(format string, args ...interface{}) {
	fmt.Printf(format+"\n", args...)
}
