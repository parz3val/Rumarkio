package user

import (
	"github.com/google/uuid"
)
type User struct {
	ID uuid.UUID `json:"id"`
	Name string `json:"name"`
	Email string `json:"email"`
	Password string `json:"password"`
}

type UserData struct {
	ID string `json:"id"`
	Name string `json:"name"`
	Email string `json:"email"`
	Password string `json:"password"`
	CreatedOn string `json:"created_on"`
}

type LoginDeets struct {	
	Email string `json:"email"`
	Password string `json:"password"`
}
