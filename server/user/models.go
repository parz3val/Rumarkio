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
