package main

import (
	"fmt"
	"math/rand"
	"time"
)

func quicksort(slice []int) {

	if len(slice) <= 1 {
		return
	}

	pivot := partition(slice, choose_pivot(slice))
	left := slice[0:pivot]
	right := slice[pivot+1:]

	quicksort(left)
	quicksort(right)
}

func par_quicksort(slice []int) {

	if len(slice) <= 1 {
		return
	}

	pivot := partition(slice, choose_pivot(slice))
	left := slice[0:pivot]
	right := slice[pivot+1:]

	go quicksort(left)
	go quicksort(right)
}

func partition(slice []int, pivot int) int {
	if len(slice) == 1 {
		return 0
	}

	mid := len(slice) - 1
	swap(slice, pivot, mid)
	left := 0
	right := mid - 1

	for left < right {
		if slice[left] <= slice[mid] {
			left = left + 1
		} else if slice[right] >= slice[mid] {
			right = right - 1
		} else {
			swap(slice, left, right)
			left = left + 1
			right = right + 1
		}
	}

	if left > right {
		swap(slice, left, mid)
		return left
	}

	if slice[left] >= slice[mid] {
		swap(slice, left, mid)
		return left
	} else if slice[left] <= slice[mid] {
		swap(slice, left+1, mid)
		return left + 1
	}

	panic("partition falied.")
}

func choose_pivot(slice []int) int {
	left := 0
	mid := len(slice) / 2
	right := len(slice) - 1

	if slice[right] < slice[left] {
		left, right = right, left
	}

	if slice[mid] <= slice[left] {
		return left
	} else if slice[right] <= slice[mid] {
		return right

	} else {
		return mid
	}

}

func swap(slice []int, a int, b int) {
	if a <= len(slice) && b <= len(slice) {
		slice[a], slice[b] = slice[b], slice[a]
	}
}

func main() {

	sortSize := 20000000
	// MAXGOROUTINES := 1
	unsorted := make([]int, 0, sortSize)
	unsorted = rand.Perm(sortSize)

	start := time.Now()
	quicksort(unsorted)
	duration := time.Since(start)
	fmt.Println("single ", duration)

	unsorted = rand.Perm(sortSize)
	start = time.Now()
	par_quicksort(unsorted)
	duration = time.Since(start)

	fmt.Println(duration)

}
