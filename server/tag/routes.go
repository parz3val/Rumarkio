package tag

import (
	"database/sql"
	"log"

	"github.com/dgrijalva/jwt-go"
	"github.com/gin-gonic/gin"
	"github.com/google/uuid"
)

type TagInput struct {
	Name string `json:"name"`
}


func CreateTag(c *gin.Context) {
	defer c.Abort()
	var input TagInput
	user := c.Keys["user"].(jwt.MapClaims)
	userId, _ := uuid.Parse(user["id"].(string))
	tag := Tag{
		Name: input.Name,
		ID: uuid.New(),
		CustomerID: userId,
	}

	db, _ := c.Keys["db"].(*sql.DB)

	err := CreatePGTag(db, &tag)
	if err!= nil {
		log.Println(err)
		c.JSON(500, gin.H{
			"msg": err,
		})
		return
	}
	c.JSON(200, gin.H{
		"msg": "Sucess",
		"data": tag,
	})
}

func GetAllTags(c *gin.Context) {
	defer c.Abort()
	user := c.Keys["user"].(jwt.MapClaims)
	userId, _ := uuid.Parse(user["id"].(string))
	db, _ := c.Keys["db"].(*sql.DB)
	tags, err := GetAllPGTags(userId, db)
	if err != nil {
		c.JSON(500, gin.H{
			"msg": err,
		})
		return
	}
	c.JSON(200, gin.H{
		"msg": "Success",
		"data": tags,
	})
}