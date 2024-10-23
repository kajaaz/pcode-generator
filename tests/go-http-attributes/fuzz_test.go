package main

import (
	"net/http"
	"net/http/httptest"
	"testing"
)

func FuzzHandler2(f *testing.F) {
	// Fuzzing function
	f.Fuzz(func(t *testing.T, input string) {
		// Create a new HTTP request with the fuzzed input
		req := httptest.NewRequest("GET", "/?input="+input, nil)

		// Create a ResponseRecorder to capture the response
		rr := httptest.NewRecorder()

		// Call the handler function with the request and ResponseRecorder
		handler(rr, req)

		// Optionally, check the response to ensure it has the expected properties
		if rr.Code != http.StatusOK {
			t.Errorf("Unexpected status code: got %v, want %v", rr.Code, http.StatusOK)
		}

		// Optionally, check for other potential issues in the response body
		body := rr.Body.String()
		if body == "" {
			t.Errorf("Response body is empty for input: %v", input)
		}
	})
}
