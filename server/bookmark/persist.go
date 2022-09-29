package bookmark

import "database/sql"


func CreatePGBookMark(db *sql.DB, mark *BookMark) error {
	var  stmt string
	var err error
	stmt = `insert into bookmarks(id, url, domain, tag, collection, description, customer_id) values($1, $2, $3, $4, $5, $6, $7);`
	_, err = db.Exec(stmt, mark.ID, mark.UrlStr, mark.Domain, mark.Tag, mark.Collection, mark.Description, mark.CustomerID)
	return err
}