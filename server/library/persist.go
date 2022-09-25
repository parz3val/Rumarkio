package library

import (
	"database/sql"
	"log"

	_ "github.com/lib/pq" //
)

func CreatePGLibrary(db *sql.DB, lib *Library) error {
	stmt := `insert into libraries(name,customer_id) values ($1, $2);`
	_, err := db.Exec(stmt, lib.Name,lib.UserID)
	return err
}

func GetUserLibraries(db *sql.DB, user int) ([]Library, error) {
	var libs []Library

	stmt := `select * from libraries where customer_id=$1;`

	log.Println(user)

	rows, err := db.Query(stmt, user)

	if err != nil {
		return []Library{}, err
	}

	for rows.Next() {
		var lib Library
		err := rows.Scan(&lib.ID, &lib.Name, &lib.UserID)
		if err != nil {
			return []Library{}, err
		}
		libs = append(libs, lib)
	}
	log.Println("Libraries:: ->")
	log.Println(libs)
	return libs, err
}

func CreatePGCollection(db *sql.DB, col *Collection) error {
	stmt := `insert into collections(name, library) values($1, $2);`
	_, err := db.Exec(stmt, col.Name, col.Library)
	return err

}

// func GetCollectionsByLibrary() ([]Collection, error){}

func CreatePGTag(db *sql.DB, tag *Tag) error {
	stmt := `insert into tags(name, customer_id) values($1, $2);`
	_, err := db.Exec(stmt, tag.Name, tag.UserId)
	return err
}

// func GetTagsByUser() ([]Tag, error) {}

func CreatePGBookMark(db *sql.DB, mark *BookMark) error {
	stmt := `insert into bookmarks(url, domain, tag, collection, description) values($1, $2, $3, $4, $5);`
	_, err := db.Exec(stmt, mark.Str, mark.Domain, mark.Tag, mark.Collection, mark.Description)
	return err
}

// func GetAllBookMarkByCollection() ([]BookMark, error){}

// func GetAllBookMarkByTag() ([]BookMark, error){}