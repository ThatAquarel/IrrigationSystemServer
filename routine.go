package main

import "time"

var (
	gpioMap        = map[string]int{"f": 14, "bp": 15, "bs": 18}
	running        = false
	timer          = time.NewTimer(0)
	hardware_error = false
)

type Routine []Zone
type Zone struct {
	zone     string
	duration int
}

func runRoutine(zones Routine) {
	running = true
	defer stopRoutine()

	for _, zone := range zones {
		var err error
		err = allOff()
		err = setPin(gpioMap[zone.zone], true)
		hardware_error = err != nil

		timerCurrent := time.NewTimer(time.Duration(zone.duration) * time.Second)
		timer = timerCurrent
		<-timerCurrent.C

		if !running {
			break
		}
	}
}

func stopRoutine() {
	running = false
	timer.Stop()
	allOff()
}

func allOff() error {
	var err error

	for zone := range gpioMap {
		err = setPin(gpioMap[zone], false)
	}

	return err
}
