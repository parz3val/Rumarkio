package user

import (
	"database/sql"
	"log"

	_ "github.com/lib/pq" //
)

func CreatePGUser(db *sql.DB, user *User) error {
	log.Println("The user is ", user)
	stmt := `insert into users(id, name, email, password) values ($1, $2, $3, $4);`

	_, err := db.Exec(stmt, user.ID, user.Name, user.Email, user.Password)

	return err
}

func GetUser(db *sql.DB, id string) (User, error) {
	var user User

	stmt := `select * from users where id=$1;`

	rows, err := db.Query(stmt, id)

	if err != nil {
		return User{}, err
	}

	for rows.Next() {
		err = rows.Scan(&user.ID, &user.Name, &user.Email, &user.Password)
		if err != nil {
			return User {}, err
		}
	}
	return user, nil
}

func GetUserByEmail(db *sql.DB, email string) (UserData, error) {
	var user UserData

	stmt := `select * from users where email=$1;`

	rows, err := db.Query(stmt, email)

	if err != nil {
		return UserData{}, err
	}

	log.Println(rows)
	for rows.Next() {
		err = rows.Scan(&user.ID, &user.Name, &user.Email, &user.Password)

		if err != nil {
			return UserData{}, err
		}
	}
	return user, nil
}