package main

import (
	"gausss/aoc2021/utils"
	"testing"
)

func TestComputePolymers(t *testing.T) {
	var lines = utils.ReadFile("input/part14_test_input.txt")
	var result = computePolymers(lines, map[string]int{"NN": 1, "NC": 1, "CB": 1})
	println(result)
	if result != 2188189693529 {
		t.Error("Wrong number of possible paths")
	}
}
func TestSolve14(t *testing.T) {
	var lines = utils.ReadFile("input/part14_input.txt")
	var result = computePolymers(lines, map[string]int{
		"SF": 1, "FB": 1, "BB": 1, "BN": 1, "NK": 1, "KK": 1, "KO": 1, "OH": 1, "HH": 2, "HP": 1, "PF": 2,
		"FO": 1, "OF": 1, "FF": 1, "FS": 1, "SP": 1, "FV": 1})
	println(result)
}
