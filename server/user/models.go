package user

type User struct {
	ID uint64 `json:"id"`
	Name string `json:"name"`
	Email string `json:"email"`
	Password string `json:"password"`
}

type LoginDeets struct {	
	Email string `json:"email"`
	Password string `json:"password"`
}

func CreatePGUser(user *User) error {
	stmt := `insert into users(name, email, password) values ($1, $2, $3);`

	_, err := db.Exec(stmt, user.Name, user.Email, user.Password)

	return err
}

func GetUser(id string) (User, error) {
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

func GetUserByEmail(email string) (User, error) {
	var user User

	stmt := `select * from users where email=$1;`

	rows, err := db.Query(stmt, email)

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