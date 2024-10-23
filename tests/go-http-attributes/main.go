package main

import (
	"html/template"
	"log"
	"net/http"
	"time"
)

var tmpl = `
<!DOCTYPE html>
<html>
<head>
    <title>Vulnerable Page</title>
</head>
<body>
    <!-- Vulnerable unquoted attribute -->
    <a href="/" attr={{.}}>Click here</a>
</body>
</html>
`

func handler(w http.ResponseWriter, r *http.Request) {
	// Parse and execute the vulnerable template with user input
	t, err := template.New("webpage").Parse(tmpl)
	if err != nil {
		http.Error(w, "Error parsing template", http.StatusInternalServerError)
		return
	}

	// Get user input from the URL query parameter 'input'
	userInput := r.URL.Query().Get("input")

	// Execute the template, passing the user input directly
	if err := t.Execute(w, userInput); err != nil {
		log.Printf("Error executing template: %v", err)
		http.Error(w, "Error rendering template", http.StatusInternalServerError)
	}
}

func main() {
	// Set up the HTTP server with timeouts to prevent slowloris-style DoS attacks
	srv := &http.Server{
		Addr:         ":8080",
		ReadTimeout:  5 * time.Second,  // Set a read timeout
		WriteTimeout: 10 * time.Second, // Set a write timeout
		IdleTimeout:  120 * time.Second, // Set an idle timeout
		Handler:      http.DefaultServeMux, // Use the default handler
	}

	http.HandleFunc("/", handler)

	// Start the server, handling errors if they occur
	log.Println("Starting server on :8080")
	if err := srv.ListenAndServe(); err != nil && err != http.ErrServerClosed {
		log.Fatalf("Server failed to start: %v", err)
	}
}
