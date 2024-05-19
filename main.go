package main

import (
	"encoding/json"
	"errors"
	"fmt"
	"log"
	"net/http"
	"net/url"
	"os"
	"path/filepath"
	"strconv"
	"strings"
	"time"

	"github.com/gorilla/mux"
)

func fmtNow() string {
	t := time.Now().UTC()
	return t.Format("2006-01-02 15:04:05")
}

func fmtResp(op string, err error) map[string]interface{} {
	resp := make(map[string]interface{})
	resp["op"] = op
	resp["ok"] = !hardware_error
	resp["running"] = running
	resp["time"] = fmtNow()

	if err != nil {
		resp["error"] = err
	}

	return resp
}

func ParseQuery(query string) (Routine, error) {
	m := make(Routine, len(gpioMap), len(gpioMap))
	err := parseQuery(m, query)
	return m, err
}

func parseQuery(m Routine, query string) (err error) {
	i := 0
	zone := true

	for query != "" {
		var key string
		key, query, _ = strings.Cut(query, "&")
		if strings.Contains(key, ";") {
			err = fmt.Errorf("invalid semicolon separator in query")
			continue
		}
		if key == "" {
			continue
		}
		key, value, _ := strings.Cut(key, "=")
		key, err1 := url.QueryUnescape(key)
		if err1 != nil {
			if err == nil {
				err = err1
			}
			continue
		}
		value, err1 = url.QueryUnescape(value)
		if err1 != nil {
			if err == nil {
				err = err1
			}
			continue
		}

		if strings.Contains(key, "zone") && zone {
			m[i] = Zone{zone: value, duration: -1}
			zone = false
		} else {
			if !strings.Contains(key, "duration") {
				err = fmt.Errorf("malformed request")
				continue
			}
			d, err1 := strconv.Atoi(value)
			if err1 != nil || d < 0 {
				err = fmt.Errorf("malformed request, duration not a number")
				continue
			}
			m[i].duration = d

			i += 1
			zone = true
		}
	}
	return err
}

func run(w http.ResponseWriter, r *http.Request) {
	routine, err := ParseQuery(r.URL.RawQuery)
	if err == nil {
		if !running {
			go runRoutine(routine)
		} else {
			err = errors.New("task already started")
		}
	}

	json.NewEncoder(w).Encode(fmtResp("run", err))
}

func stop(w http.ResponseWriter, r *http.Request) {
	stopRoutine()

	json.NewEncoder(w).Encode(fmtResp("stop", nil))
}

func status(w http.ResponseWriter, r *http.Request) {
	json.NewEncoder(w).Encode(fmtResp("status", nil))
}

type spaHandler struct {
	staticPath string
	indexPath  string
}

func (h spaHandler) ServeHTTP(w http.ResponseWriter, r *http.Request) {
	path := filepath.Join(h.staticPath, r.URL.Path)

	fi, err := os.Stat(path)
	if os.IsNotExist(err) || fi.IsDir() {
		http.ServeFile(w, r, filepath.Join(h.staticPath, h.indexPath))
		return
	}

	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	http.FileServer(http.Dir(h.staticPath)).ServeHTTP(w, r)
}

func main() {
	router := mux.NewRouter()

	router.HandleFunc("/routine/run", run)
	router.HandleFunc("/routine/stop", stop)
	router.HandleFunc("/routine/status", status)

	spa := spaHandler{staticPath: "static", indexPath: "index.html"}
	router.PathPrefix("/").Handler(spa)

	srv := &http.Server{
		Handler:      router,
		Addr:         addr,
		WriteTimeout: 15 * time.Second,
		ReadTimeout:  15 * time.Second,
	}

	log.Fatal(srv.ListenAndServe())
}
