package main

import (
	"fmt"
	"io"
	"math/rand"
	"os"
	"time"
)

func Cqsort(s []int, MAXGOROUTINES int) {
	if len(s) <= 1 {
		return
	}
	workers := make(chan int, MAXGOROUTINES-1)
	for i := 0; i < (MAXGOROUTINES - 1); i++ {
		workers <- 1
	}
	cqsort(s, nil, workers)
}

func cqsort(s []int, done chan int, workers chan int) {
	// report to caller that we're finished
	if done != nil {
		defer func() { done <- 1 }()
	}

	if len(s) <= 1 {
		return
	}
	// since we may use the doneChannel synchronously
	// we need to buffer it so the synchronous code will
	// continue executing and not block waiting for a read
	doneChannel := make(chan int, 1)

	pivotIdx := partition(s)

	select {
	case <-workers:
		// if we have spare workers, use a goroutine
		// for parallelization
		go cqsort(s[:pivotIdx+1], doneChannel, workers)
	default:
		// if no spare workers, sort synchronously
		cqsort(s[:pivotIdx+1], nil, workers)
		// calling this here as opposed to using the defer
		doneChannel <- 1
	}
	// use the existing goroutine to sort above the pivot
	cqsort(s[pivotIdx+1:], nil, workers)
	// if we used a goroutine we'll need to wait for
	// the async signal on this channel, if not there
	// will already be a value in the channel and it shouldn't block
	<-doneChannel
	return
}

func partition(s []int) (swapIdx int) {
	pivotIdx, pivot := pickPivot(s)
	// swap right-most element and pivot
	s[len(s)-1], s[pivotIdx] = s[pivotIdx], s[len(s)-1]
	// sort elements keeping track of pivot's idx
	for i := 0; i < len(s)-1; i++ {
		if s[i] < pivot {
			s[i], s[swapIdx] = s[swapIdx], s[i]
			swapIdx++
		}
	}
	// swap pivot back to its place and return
	s[swapIdx], s[len(s)-1] = s[len(s)-1], s[swapIdx]
	return
}

func pickPivot(s []int) (pivotIdx int, pivot int) {
	pivotIdx = rand.Intn(len(s))
	pivot = s[pivotIdx]
	return
}

func main() {

	file, err := os.Open("data/perm50e6.txt")
	if err != nil {
		fmt.Println(err)
		os.Exit(1)
	}

	var perline int
	var nums1 []int
	var nums2 []int
	fmt.Println("Reading File:...")
	for {
		_, err := fmt.Fscanf(file, "%d\n", &perline) // give a patter to scan

		if err != nil {

			if err == io.EOF {
				break // stop reading the file
			}
			fmt.Println(err)
			os.Exit(1)
		}
		nums1 = append(nums1, perline)
		nums2 = append(nums2, perline)
	}
	fmt.Println("Sorting!")

	start := time.Now()
	Cqsort(nums1, 16)
	duration := time.Since(start)

	fmt.Println("goroutines", duration)

	start = time.Now()
	Cqsort(nums2, 1)
	duration = time.Since(start)

	fmt.Println(duration)

}
