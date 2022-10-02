package tag

import (
	"database/sql"
	"fmt"

	"github.com/google/uuid"
)


func CreatePGTag(db *sql.DB, tag *Tag) error {
	// stmt := `insert into tags(name, customer_id) values($1, $2);`
	// _, err := db.Exec(stmt, tag.Name, tag.UserId)
	stmt := `insert into tags(name, id, customer_id) values ($1, $2, $3);`
	_, err := db.Exec(stmt, tag.Name, tag.ID, tag.CustomerID)
	return err
}

func GetAllPGTags(user uuid.UUID, db *sql.DB) ([]Tag, error) {

	var tags []Tag

	stmt := `select * from tags where customer_id = $1`
	rows, err := db.Query(stmt, user)

	if err != nil {
		return []Tag{}, err
	}

	defer rows.Close()
	for rows.Next() {
		var tag Tag
		err := rows.Scan(&tag.Name, &tag.ID, &tag.CreatedOn, &tag.ModifiedOn, &tag.CustomerID)
		if err != nil {
			fmt.Println(err)
			fmt.Println("Error happened here")
			return []Tag{}, err
		}
		tags = append(tags, tag)

		if err := rows.Err(); err != nil {
			return tags, err
		}
	} 
	return tags, nil

}


func TagExistsForUser(db *sql.DB, tag_ *Tag) (bool, error) {
	var tags []Tag
	stmt := `select * from tags where customer_id = $1 and name = $2;`
	rows, err := db.Query(stmt, tag_.CustomerID, tag_.Name)

	if err != nil {
		return false, err
	}

	defer rows.Close()
	for rows.Next() {
		var tag Tag
		err := rows.Scan(&tag.Name, &tag.ID, &tag.CreatedOn, &tag.ModifiedOn, &tag.CustomerID)
		if err != nil {
			fmt.Println(err)
			fmt.Println("Error happened here")
			return false, err
		}
		tags = append(tags, tag)

		if err := rows.Err(); err != nil {
			return false, err
		}
	} 
	if len(tags) < 1 {
		return false, nil
	}
	return true, nil

}