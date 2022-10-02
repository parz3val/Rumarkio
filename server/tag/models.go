package tag

import "github.com/google/uuid"

type Tag struct {
	Name string `json:"name"`
	ID uuid.UUID `json:"id"`
	CreatedOn string `json:"created_on"`
	ModifiedOn string `json:"modified_on"`
	CustomerID uuid.UUID `json:"customer_id"`
}