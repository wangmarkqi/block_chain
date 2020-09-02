package main

//go build --buildmode=c-archive  hello.go

import "C"
import "fmt"

func main() {
	Hello("hello")
}

//export Hello
func Hello(name string) {
	fmt.Println("output:",name)
}