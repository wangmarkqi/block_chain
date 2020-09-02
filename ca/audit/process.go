package audit

import (
	"fmt"
	"os"
	"sort"
	"github.com/joho/godotenv"
)

func (ct Contract) SaveContract(data string) error {
	AuditDb.Save(&self)
	return nil
}
func CreatT() {
	fmt.Println("adfa")

	godotenv.Load()

	dbinfo := os.Getenv("DB")
	fmt.Println(dbinfo)
}

func (cm Contract) QueryContract() (interface{}, error) {
	AuditDb.Where("Imei = ?", "adf").Find(&cm)
	res := make([]Contract, 0, 0)
	sort.Slice(res, func(i, j int) bool {
		ti := res[i].CreatedAt.Format(common.TimeFormat)
		tj := res[j].CreatedAt.Format(common.TimeFormat)
		return ti < tj
	})
	return res, nil

}
