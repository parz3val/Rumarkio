package bookmark

import "github.com/google/uuid"


type BookMark struct {
	UrlStr string `json:"url_str"`
	Domain string `json:"domain"`
	ID uuid.UUID `json:"id"`
	Tag uuid.NullUUID `json:"tag"`
	Collection uuid.NullUUID `json:"collection"`
	Description string `json:"description"`
	CreatedOn string `json:created_on`
	modifiedOn string `json:modified_on`
	CustomerID uuid.UUID `json:customer_id` // user_id
}