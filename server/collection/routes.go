package collection

import (
	"database/sql"
	"log"

	"github.com/dgrijalva/jwt-go"
	"github.com/gin-gonic/gin"
	"github.com/google/uuid"
)



type CollectionInput struct {
	Name string `json:"name"`
	Library uuid.NullUUID `json:"library"`
}


func CreateCollection(c *gin.Context) {
	defer c.Abort()
	var input CollectionInput
	c.Bind(&input)
	user := c.Keys["user"].(jwt.MapClaims)
	userId, _ := uuid.Parse(user["id"].(string))
	db, _ := c.Keys["db"].(*sql.DB)
    col := Collection{
		ID: uuid.New(),
		Name: input.Name,
		Library: input.Library,
		CustomerID: userId ,
	}
	err := CreatePGCollection(db, &col);
	if err!= nil {
		log.Println(err)
		c.JSON(500, gin.H{
			"msg": err,
		})
		return
	}
	c.JSON(200, gin.H{
		"msg": "Sucess",
		"data": col,
	})

}