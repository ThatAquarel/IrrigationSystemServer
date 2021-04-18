package main

import (
	"bufio"
	"fmt"
	"github.com/stianeikeland/go-rpio"
	"net"
	"strconv"
	"strings"
	"time"
)

var (
	gpioMap = map[string]int{"front": 14, "back_pool": 15, "back_shed": 18}
	running = false
	timer   = time.NewTimer(0)
)

func decode(netData string) ([]string, []int) {
	keys := make([]string, 3, 3)
	values := make([]int, 3, 3)

	elements := strings.Split(netData, ",")

	for i := 0; i < 3; i++ {
		element := strings.Split(elements[i], "=")
		keys[i] = element[0]
		values[i], _ = strconv.Atoi(element[1])
	}

	return keys, values
}

func digitalWrite(key string, state bool) {
	pin := rpio.Pin(gpioMap[key])

	err := rpio.Open()
	handleError(err)
	defer rpio.Close()

	pin.Output()

	if state {
		fmt.Printf("HIGH %v\n", gpioMap[key])
		pin.High()
	} else {
		fmt.Printf("LOW %v\n", gpioMap[key])
		pin.Low()
	}
}

func allOff(keys []string) {
	for _, key := range keys {
		digitalWrite(key, false)
	}
}

func handleError(err error) {
	if err != nil {
		return
	}
}

func handleStopRoutine(keys []string) {
	allOff(keys)
	timer.Stop()
	running = false
}

func handleRoutines(keys []string, values []int) {
	running = true
	for i := 0; i < 3; i++ {
		allOff(keys)
		digitalWrite(keys[i], true)

		timerCurrent := time.NewTimer(time.Duration(values[i]) * time.Second)
		timer = timerCurrent
		<-timerCurrent.C

		if !running {
			break
		}
	}
	allOff(keys)
	running = false
}

func handleConnection(c net.Conn) {
	netData, _ := bufio.NewReader(c).ReadString('\n')
	netData = strings.ReplaceAll(netData, "\n", "")
	if netData == "" {
		return
	}

	keys, values := decode(netData)

	response := fmt.Sprintf("%v %v %s\n", time.Now().Format(time.RFC3339), c.RemoteAddr(), netData)
	fmt.Print(response)

	if !running {
		go handleRoutines(keys, values)
	} else {
		netData += " Stop Routine"
		handleStopRoutine(keys)
	}

	_, err := c.Write([]byte(response))
	handleError(err)

	err = c.Close()
	handleError(err)
}

func main() {
	l, err := net.Listen("tcp4", ":6969")
	handleError(err)

	defer func(l net.Listener) {
		err := l.Close()
		handleError(err)
	}(l)

	for {
		c, err := l.Accept()
		handleError(err)
		go handleConnection(c)
	}
}
