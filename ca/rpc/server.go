package rpc

import (
	"ca/sm"
	"encoding/json"
	"fmt"
	"github.com/joho/godotenv"
	"io/ioutil"
	"net/http"
	"os"
)

func caHandler(w http.ResponseWriter, r *http.Request) {
	body, _ := ioutil.ReadAll(r.Body)
	var ca sm.CA
	err := json.Unmarshal(body, &ca)
	if err != nil {
		fmt.Println(err)
	}
	res := sm.Dispatch(ca)
	js, _ := json.Marshal(&res)

	fmt.Fprint(w, string(js))
}
func auditHandler(w http.ResponseWriter, r *http.Request) {
	body, _ := ioutil.ReadAll(r.Body)
	fmt.Println(string(body))
	fmt.Fprint(w, "ok")
}
func Start() {
	godotenv.Load()
	rpc := os.Getenv("RPC")
	fmt.Println(rpc)

	http.HandleFunc("/ca", caHandler)
	http.HandleFunc("/audit", auditHandler)
	http.ListenAndServe(rpc, nil)
}
