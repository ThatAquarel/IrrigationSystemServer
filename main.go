package main

import (
	"bufio"
	"encoding/json"
	"fmt"
	"net"
	"strings"
	"time"
)

var gpioMap = map[string]int{"front": 0, "back_pool": 1, "back_shed": 2}
var running = false
var timer = time.NewTimer(0)

func decodeJson(netData string) (map[string]interface{}, []string) {
	var jsonMap map[string]interface{}

	err := json.Unmarshal([]byte(netData), &jsonMap)
	handleError(err)

	keys := make([]string, 0, len(jsonMap))
	for i := range jsonMap {
		keys = append(keys, i)
	}

	return jsonMap, keys
}

func digitalWrite(key string, state bool) {
	if state {
		fmt.Printf("HIGH %v\n", gpioMap[key])
		return
	}
	fmt.Printf("LOW %v\n", gpioMap[key])
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

func handleRoutines(jsonMap map[string]interface{}, keys []string) {
	running = true
	for i := 0; i < 3; i++ {
		allOff(keys)
		digitalWrite(keys[i], true)

		timerCurrent := time.NewTimer(time.Duration(int(jsonMap[keys[i]].(float64))) * time.Second)
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

	jsonMap, keys := decodeJson(netData)

	if !running {
		go handleRoutines(jsonMap, keys)
	} else {
		netData += " Stop Routine"
		handleStopRoutine(keys)
	}

	response := fmt.Sprintf("%v %v %s\n", time.Now().Format(time.RFC3339), c.RemoteAddr(), netData)
	fmt.Print(response)
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
