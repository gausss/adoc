package main

import (
	"gausss/aoc2021/utils"
	"testing"
)

func TestComputePossiblePaths(t *testing.T) {
	var lines = utils.ReadFile("input/part12_test_input.txt")
	var result = computePossiblePaths(lines)
	println(result)
	if result != 36 {
		t.Error("Wrong number of possible paths")
	}
}
