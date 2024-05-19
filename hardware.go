package main

var gpioMap = map[string]int{"front": 14, "back_pool": 15, "back_shed": 18}

func allOff(gpios []int) error {
	var err error

	for _, pin := range gpios {
		err = setPin(pin, false)
	}

	return err
}
