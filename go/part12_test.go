package main

import (
	"testing"
)

func TestSum(t *testing.T) {
	var lines = readFile("input/part12_test_input.txt")
	var result = computePossiblePaths(lines)
	println(result)
	if result != 36 {
		t.Error("Wrong number of possible paths")
	}
}
