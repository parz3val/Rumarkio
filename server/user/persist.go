package user

import (
	"database/sql"
	"fmt"

	_ "github.com/lib/pq" //
)

var db *sql.DB

func setupPOSTGRES() {
	dsn := "host=0.0.0.0 port=5432 user=postgres password=rumarkio dbname=rumarkio_test sslmode=disable"
	var err error
	db, err = sql.Open("postgres", dsn)

	if err != nil {
		panic(err)
	}
	err = db.Ping()

	if err != nil {
		fmt.Println(err)
	}
}

func Setup() {
	setupPOSTGRES()
}