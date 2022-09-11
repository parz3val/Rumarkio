package library

type Library struct {
	Name string `json:"name"`
	ID uint64 `json:"id"`
	UserID uint64 `json:"user_id"`
}