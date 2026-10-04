package main

import (
	"fmt"
	"net/http"
)

func main() {
	http.HandleFunc("/", webHandler)

	fmt.Println("Web server started on port: 8080")
	http.ListenAndServe(":8080", nil)
}

func webHandler(w http.ResponseWriter, r *http.Request) {
	fmt.Fprintln(w, "Welcome to web GO server")
}
