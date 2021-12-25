package main

import (
	"gausss/aoc2021/utils"
	"testing"
)

func TestComputeVisibleDots(t *testing.T) {
	var lines = utils.ReadFile("input/part13_test_input.txt")
	var paper = makePaper(lines)
	paper.foldY(7)

	var result = len(paper.distinct())
	println(result)
	if result != 17 {
		t.Error("Wrong number of possible paths")
	}
}

func TestSolve13(t *testing.T) {
	var lines = utils.ReadFile("input/part13_input.txt")
	var paper = makePaper(lines)
	paper.foldX(655)
	paper.foldY(447)
	paper.foldX(327)
	paper.foldY(223)
	paper.foldX(163)
	paper.foldY(111)
	paper.foldX(81)
	paper.foldY(55)
	paper.foldX(40)
	paper.foldY(27)
	paper.foldY(13)
	paper.foldY(6)

	dotMap := paper.distinct()
	var display []string
	for y := 0; y < 20; y++ {
		display = append(display, "")
		for x := 0; x < 20; x++ {
			if dotMap[Dot{x, y}] == true {
				display[y] += "#"
			} else {
				display[y] += "."
			}
		}
	}

	for _, value := range display {
		println(value)
	}
}
