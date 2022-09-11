package library

import (
	"database/sql"

	_ "github.com/lib/pq" //
)

var db *sql.DB
func CreatePGLibrary(lib *Library) error {
	stmt := `insert into library(name, id, user_id) values ($1, $2, $3);`
	_, err := db.Exec(stmt, lib.Name, lib.ID, lib.UserID)
	return err
}