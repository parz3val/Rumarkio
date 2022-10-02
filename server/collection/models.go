package collection

import "github.com/google/uuid"

type Collection struct {
	ID uuid.UUID `json:"id"`
	Name string `json:"name"`
	CreatedOn string `json:"created_on"`
	ModifiedOn string `json:"modified_on"`
	Library uuid.NullUUID `json:"library"`
	CustomerID uuid.UUID `json:"customer_id"`
}