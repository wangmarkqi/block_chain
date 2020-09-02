package sm

import (
	"encoding/hex"
	"fmt"
	"github.com/joho/godotenv"
	"github.com/tjfoc/gmsm/sm2"
	"io/ioutil"
	"log"
	"math/big"

	"os"
	"strings"
)

const key = "wq"
const split = "&&&&"

func Get_key_path(name string) string {
	godotenv.Load()
	p := os.Getenv("DATA")
	path := fmt.Sprintf("%s/%s", p, name)
	return path

}
func GenKeyPair(skpath, pkpath string) {
	priv, _ := sm2.GenerateKey() // 生成密钥对
	pub := &priv.PublicKey
	sm2.WritePublicKeytoPem(pkpath, pub, []byte(key))
	sm2.WritePrivateKeytoPem(skpath, priv, []byte(key))
}
func Enc(pkpath string, message string) string {
	pub, err := sm2.ReadPublicKeyFromPem(pkpath, []byte(key))
	fmt.Println(333,err)
	fmt.Println(message)
	if err!=nil{
		fmt.Println(err)
	}
	msg := []byte(message)
	ciphertxt, err := pub.Encrypt(msg)
	if err != nil {
		log.Fatal(err)
	}
	str := hex.EncodeToString(ciphertxt)
	return str
}
func Dec(skpath string, ciphertxt string) string {
	priv, _ := sm2.ReadPrivateKeyFromPem(skpath, []byte(key))
	b, _ := hex.DecodeString(ciphertxt)
	plaintxt, _ := priv.Decrypt(b)
	return string(plaintxt)
}

func Sign(skpath string, message string) string {
	priv, _ := sm2.ReadPrivateKeyFromPem(skpath, []byte(key))
	msg := []byte(message)
	r, s, _ := sm2.Sign(priv, msg)
	res := fmt.Sprintf("%x%s%x", r, split, s)
	return res

}
func Verify(pkpath string, message string, sign string) bool {
	l := strings.Split(sign, split)

	rrr := new(big.Int)
	rrr.SetString(l[0], 16)
	sss := new(big.Int)
	sss.SetString(l[1], 16)

	pub, _ := sm2.ReadPublicKeyFromPem(pkpath, []byte(key))
	msg := []byte(message)
	isok := sm2.Verify(pub, msg, rrr, sss)
	fmt.Printf("Verified: %v\n", isok)
	return isok

}

func Read_file(name string) string {
	b, err := ioutil.ReadFile(name)
	if err != nil {
		fmt.Print(err)
	}
	str := string(b)
	return str
}

func Write_file(name string, content string) {
	d1 := []byte(content)
	ioutil.WriteFile(name, d1, 0644)
	return
}
func Write_temp_file(content string) string {
	tmpfile, _ := ioutil.TempFile("", "temp*")
	tmpfile.Write([]byte(content))
	res := Read_file(tmpfile.Name())
	fmt.Println(res)
	return res
}
