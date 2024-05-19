//go:build armv6l

package main

import "github.com/stianeikeland/go-rpio"

func setPin(gpio int, state bool) error {
	pin := rpio.Pin(gpio)

	err := rpio.Open()
	if err != nil {
		return err
	}
	defer rpio.Close()

	pin.Output()

	if state {
		pin.High()
	} else {
		pin.Low()
	}

	return nil
}
