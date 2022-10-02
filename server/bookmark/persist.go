package bookmark

import (
	"database/sql"

	"fmt"

	"github.com/google/uuid"
)


func CreatePGBookMark(db *sql.DB, mark *BookMark) error {
	var  stmt string
	var err error
	stmt = `insert into bookmarks(id, url, domain, tag, collection, description, customer_id) values($1, $2, $3, $4, $5, $6, $7);`
	_, err = db.Exec(stmt, mark.ID, mark.Url, mark.Domain, mark.Tag, mark.Collection, mark.Description, mark.CustomerID)
	return err
}


func GetAllPGBookmarks(userId uuid.UUID, db *sql.DB) ([]BookMark,  error) {
	var marks []BookMark

	stmt := `select * from bookmarks where customer_id = $1`
	rows, err := db.Query(stmt, userId)

	if err != nil {
		return []BookMark{}, err
	}

	defer rows.Close()
	for rows.Next() {
		var mark BookMark
		err := rows.Scan(&mark.Url,  &mark.Domain, &mark.ID, &mark.CreatedOn, &mark.ModifiedOn, &mark.Description, &mark.Collection, &mark.Tag, &mark.CustomerID)

		if err != nil {
			fmt.Println(err)
			fmt.Println("Error happened here")
			return []BookMark{}, err
		}
		marks = append(marks, mark)

		if err := rows.Err(); err != nil {
			return marks, err
		}
	} 
	return marks, nil

}

func GetBookmarksByDomain(domain string, user uuid.UUID, db *sql.DB) ([]BookMark, error) {

	var marks []BookMark

	stmt := `select * from bookmarks where domain = $1 and customer_id=$2;`
	rows, err := db.Query(stmt, domain, user)

	if err != nil {
		return []BookMark{}, err
	}

	defer rows.Close()
	for rows.Next() {
		var mark BookMark
		err := rows.Scan(&mark.Url,  &mark.Domain, &mark.ID, &mark.CreatedOn, &mark.ModifiedOn, &mark.Description, &mark.Collection, &mark.Tag, &mark.CustomerID)

		if err != nil {
			fmt.Println(err)
			fmt.Println("Error happened here")
			return []BookMark{}, err
		}
		marks = append(marks, mark)

		if err := rows.Err(); err != nil {
			return marks, err
		}
	} 
	return marks, nil
}

func GetBookmarksByTag(tag uuid.UUID, user uuid.UUID,  db *sql.DB) ([]BookMark, error) {

	var marks []BookMark

	stmt := `select * from bookmarks where tag = $1 and customer_id=$2;`
	rows, err := db.Query(stmt, tag, user)

	if err != nil {
		return []BookMark{}, err
	}

	defer rows.Close()
	for rows.Next() {
		var mark BookMark
		err := rows.Scan(&mark.Url,  &mark.Domain, &mark.ID, &mark.CreatedOn, &mark.ModifiedOn, &mark.Description, &mark.Collection, &mark.Tag, &mark.CustomerID)

		if err != nil {
			fmt.Println(err)
			fmt.Println("Error happened here")
			return []BookMark{}, err
		}
		marks = append(marks, mark)

		if err := rows.Err(); err != nil {
			return marks, err
		}
	} 
	return marks, nil
}


func GetBookmarksByCollection(collection uuid.UUID, user uuid.UUID, db *sql.DB) ([]BookMark, error) {

	var marks []BookMark

	stmt := `select * from bookmarks where collection = $1 and customer_id=$2;`
	rows, err := db.Query(stmt, collection, user)

	if err != nil {
		return []BookMark{}, err
	}

	defer rows.Close()
	for rows.Next() {
		var mark BookMark
		err := rows.Scan(&mark.Url,  &mark.Domain, &mark.ID, &mark.CreatedOn, &mark.ModifiedOn, &mark.Description, &mark.Collection, &mark.Tag, &mark.CustomerID)

		if err != nil {
			fmt.Println(err)
			fmt.Println("Error happened here")
			return []BookMark{}, err
		}
		marks = append(marks, mark)

		if err := rows.Err(); err != nil {
			return marks, err
		}
	} 
	return marks, nil
}


