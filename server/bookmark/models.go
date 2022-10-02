package bookmark

import "github.com/google/uuid"


type BookMark struct {
	Url string `json:"url"`
	Domain string `json:"domain"`
	ID uuid.UUID `json:"id"`
	Tag uuid.NullUUID `json:"tag"`
	Collection uuid.NullUUID `json:"collection"`
	Description string `json:"description"`
	CreatedOn string `json:"created_on"`
	ModifiedOn string `json:"modified_on"`
	CustomerID uuid.UUID `json:"customer_id"` // user_id
}