package main

import (
	"fmt"
	"net/http"
)

func main() {
	mux := http.NewServeMux()
	mux.HandleFunc("/", webHandler)
	fmt.Println("Server listening on port: 8080")
	http.ListenAndServe(":8080", mux)

}

func webHandler(w http.ResponseWriter, r *http.Request) {
	fmt.Fprintln(w, "Welcome to GO Web server")
}
