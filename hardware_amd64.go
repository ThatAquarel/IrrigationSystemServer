//go:build amd64

package main

import "fmt"

func setPin(gpio int, state bool) error {
	if state {
		fmt.Printf("HIGH %v\n", gpio)
	} else {
		fmt.Printf("LOW %v\n", gpio)
	}

	return nil
}
