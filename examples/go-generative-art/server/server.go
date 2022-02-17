package main

import (
    "fmt"
    "flag"
    "log"
    "net/http"
)

func main() {
    port := flag.String("p", "8080", "port to serve on")
    directory := flag.String("d", ".", "the directory where our assets are hosted")
    flag.Parse()

    http.Handle("/", http.FileServer(http.Dir(*directory)))

    log.Printf("Serving %s on HTTP port: %s\n", *directory, *port)
    log.Fatal(http.ListenAndServe(fmt.Sprintf(":%s", *port), nil))
}
