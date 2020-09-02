package audit

import (
	"fmt"
	"os"
	"testing"

	_ "github.com/jinzhu/gorm/dialects/sqlite"
	"github.com/joho/godotenv"
)

func TestAccount_TableName(t *testing.T) {
	godotenv.Load()
	dbinfo := os.Getenv("DB")
	fmt.Println(dbinfo)
	// AuditDb, _ := gorm.Open("sqlite3", dbinfo)
	// AuditDb.CreateTable(Contract{})
	// AuditDb.DropTable(Contract{})
}
