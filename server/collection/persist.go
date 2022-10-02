package collection

import "database/sql"

func CreatePGCollection(db *sql.DB, col *Collection) error {
	stmt := `insert into collections(id, name, library, customer_id) values($1, $2, $3, $4);`
	_, err := db.Exec(stmt, col.ID, col.Name, col.Library, col.CustomerID)
	return err

}