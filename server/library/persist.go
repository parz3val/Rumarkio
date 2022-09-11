package library

import (
	"database/sql"

	_ "github.com/lib/pq" //
)

func CreatePGLibrary(db *sql.DB, lib *Library) error {
	stmt := `insert into libraries(name, id,customer_id) values ($1, $2, $3);`
	_, err := db.Exec(stmt, lib.Name, lib.ID, lib.UserID)
	return err
}