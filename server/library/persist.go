package library

import (
	"database/sql"
	"fmt"

	"github.com/google/uuid"
	_ "github.com/lib/pq" //
)

func CreatePGLibrary(db *sql.DB, lib *Library) error {
	stmt := `insert into libraries(name, id, customer_id) values ($1, $2, $3);`
	_, err := db.Exec(stmt, lib.Name, lib.ID, lib.CustomerID)
	return err
}

func GetAllPGLibraries(user uuid.UUID, db *sql.DB) ([]Library, error) {

	var libs []Library

	stmt := `select * from libraries where customer_id = $1`
	rows, err := db.Query(stmt, user)

	if err != nil {
		return []Library{}, err
	}

	defer rows.Close()
	for rows.Next() {
		var lib Library
		err := rows.Scan(&lib.Name, &lib.ID, &lib.CreatedOn, &lib.ModifiedOn, &lib.CustomerID)
		if err != nil {
			fmt.Println(err)
			fmt.Println("Error happened here")
			return []Library{}, err
		}
		libs = append(libs, lib)

		if err := rows.Err(); err != nil {
			return libs, err
		}
	} 
	return libs, nil

}