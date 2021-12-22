package main

import (
	"gausss/aoc2021/utils"
	"testing"
)

func TestComputeFlashes(t *testing.T) {
	var lines = utils.ReadFile("input/part11_test_input.txt")
	result := modelSteps(100, lines)
	println(result)
	if result != 195 {
		t.Error("Wrong number of flashes")
	}
}
