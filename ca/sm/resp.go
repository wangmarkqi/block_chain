package sm

import (
	"fmt"
	"github.com/joho/godotenv"
	"io/ioutil"
	"os"
)

type CA struct {
	Which   string `json:"which"`
	Caname  string `json:"caname"`
	Caurl   string `json:"caurl"`
	Org     string `json:"org"`
	Sym_key string `json:"sym_key"`
	Pk      string `json:"pk"`
	Err     string `json:"err"`
}

func Verify_pk(pkstr string)bool  {
	//ca1的私钥对org1的公钥签名产生了cert，可以用ca1的公钥+传入的私钥+cert验证。略.这一步可以延伸到上一级ca，把根证书公钥的验证放到上一级，因为根证书私钥在上一级
	return true
}
func Dispatch(ca CA) CA {
	//公钥不对就拒绝，
	if !Verify_pk(ca.Pk){
		return ca
	}
	fmt.Println(ca.Which)
	switch ca.Which {
	case "ca_dec":
		return ca_dec(ca)
	case "ca_enc":
		return ca_enc(ca)
	default:
		return ca
	}

}
func ca_enc(ca CA) CA {
	tmpfile, _ := ioutil.TempFile("", "temp*")
	tmpfile.Write([]byte(ca.Pk))
	enc_key := Enc(tmpfile.Name(), ca.Sym_key)
	ca.Sym_key = enc_key
	return ca

}
func ca_dec(ca CA) CA {
	godotenv.Load()
	org := os.Getenv("ORG")
	datadir := os.Getenv("DATA")
	skpath := fmt.Sprintf("%s/child/%s/sk.pem", datadir, org)
	decstr := Dec(skpath, ca.Sym_key)
	ca.Sym_key = decstr
	return ca
}
