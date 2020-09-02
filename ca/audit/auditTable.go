
package audit

import (
	"fmt"
	"github.com/joho/godotenv"
	"os"
	"github.com/jinzhu/gorm"
	_ "github.com/jinzhu/gorm/dialects/sqlite"
)

//const mysql string = "risk:1qaz@WSX@tcp(39.96.40.177:3306)/credit?charset=utf8&loc=Local&parseTime=True"
var AuditDb *gorm.DB

func init() {
	godotenv.Load()
	dbinfo := os.Getenv("DB")
	AuditDb, err := gorm.Open("sqlite3", dbinfo)
	if err != nil {
		fmt.Println(err)
		return
	}
}

type Contract struct {
	Contract    string `json:"contract"`
	Url         string `json:"url"`
	Consistency bool   `json:"consistency"`
	Hash        bool   `json:"hash"`
	Sign        bool   `json:"sign"`
	Args        string `json:"args"`
	Res         string `json:"res"`
	Voucher     string `json:"voucher"`
	Status      string `json:"status"`
}

func (Contract) TableName() string {
	return "contract"
}
