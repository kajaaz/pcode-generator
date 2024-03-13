package main

import (
    "github.com/stretchr/testify/assert"
)

func main() {
    var a string = "Hello"
    var b string = "Hello"

    assert.Equal(nil, a, b, "The two words should be the same.") 
}
