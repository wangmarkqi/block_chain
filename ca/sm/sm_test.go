package sm

import (
	"fmt"
	"testing"
)

func TestAccount_TableName(t *testing.T) {
	sk := sm.Get_key_path("org1/sk", false)
	pk := sm.Get_key_path("org1/pk", false)

	pkpath := sm.Get_key_path("/child/org1/pk.pem")
	pkstr := sm.Read_file(pkpath)
	rootskpath := sm.Get_key_path("/root/sk.pem")
	s := sm.Sign(rootskpath, pkstr)
	fmt.Println(s)
	cerp := sm.Get_key_path("/child/org1/cert")
	sm.Write_file(cerp, s)

	enc := sm.Enc(pk, "af")
	dec := sm.Dec(sk, enc)
	s := sm.Sign(sk, "adsf")
	sm.Verify(pk, "adsf", s)
}
