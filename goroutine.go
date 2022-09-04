package main

import (
	"fmt"
	"os"
	"runtime"
	"sync"
	"time"
)

func compute() {
	wg := sync.WaitGroup{}
	wg.Add(1000)
	for i := 0; i < 1000; i++ {
		go func() {
			defer wg.Done()
			buffer := make([]byte, 10)
			devUrandom, err := os.Open("/dev/urandom")
			if err != nil {
				panic(err)
			}
			if _, err = devUrandom.Read(buffer); err != nil {
				panic(err)
			}
			if err = devUrandom.Close(); err != nil {
				panic(err)
			}
			devNull, err := os.Create("/dev/null")
			if err != nil {
				panic(err)
			}
			if _, err = devNull.Write(buffer); err != nil {
				panic(err)
			}
			if err = devNull.Close(); err != nil {
				panic(err)
			}
		}()
	}
	wg.Wait()
	runtime.GC()
}

func main() {
	// warmup
	compute()

	before := time.Now()
	for i := 0; i < 1000; i++ {
		compute()
	}
	elapsed := time.Now().Sub(before)
	fmt.Println(elapsed, "total,", elapsed/1000, "avg per iteration")
}