package library

type Library struct {
	Name string `json:"name"`
	ID uint64 `json:"id"`
	UserID uint64 `json:"user_id"`
}


type Tag struct {
	Name string `json:"name"`
	ID uint64 `json:"id"`
	UserId uint64 `json:"user_id"`
}

type URL struct {
	Str string `json:"url_str"`
	Domain string `json:"domain"`
}
