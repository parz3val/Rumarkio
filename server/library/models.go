package library

type Library struct {
	Name string `json:"name"`
	ID uint64 `json:"id"`
	UserID uint64 `json:"user_id"`
}

type Collection struct {
	Name string `json:"name"`
	ID uint64 `json:"id"`
	Library uint64 `json:"library"`
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

type BookMark struct {
	Str string `json:"url_str"`
	Domain string `json:"domain"`
	Tag uint64 `json:"tag"`
	Collection uint64 `json:"collection"`
	Description string `json:"description"`

}