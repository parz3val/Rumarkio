package library

import "github.com/google/uuid"

type Library struct {
	ID uuid.UUID `json:"id"`
	Name string `json:"name"`
	CreatedOn string `json:"created_on"`
	ModifiedOn string `json:"modified_on"`
	CustomerID uuid.UUID `json:"customer_id"`
}
