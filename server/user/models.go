package user

import "time"

type User struct {
	ID uint64 `json:"id"`
	Name string `json:"name"`
	Email string `json:"email"`
	Password string `json:"password"`
	CreatedOn time.Time `json:"created_on"`
	ModifiedOn time.Time `json:"modified_on"`
	ProfilePicture string `json:"profile_picture"`
}

type LoginDeets struct {	
	Email string `json:"email"`
	Password string `json:"password"`
}
