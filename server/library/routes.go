package library

import (
	"database/sql"
	"log"

	"github.com/dgrijalva/jwt-go"
	"github.com/gin-gonic/gin"
	"github.com/google/uuid"
)

type Lib struct {
	Name string `json:"name"`
}


func CreateLibrary(c *gin.Context) {
	defer c.Abort()
	var input Lib
	user := c.Keys["user"].(jwt.MapClaims)
	userId, _ := uuid.Parse(user["id"].(string))
	lib := Library{
		Name: input.Name,
		ID: uuid.New(),
		CustomerID: userId,
	}

	db, _ := c.Keys["db"].(*sql.DB)

	err := CreatePGLibrary(db, &lib)
	if err!= nil {
		log.Println(err)
		c.JSON(500, gin.H{
			"msg": err,
		})
		return
	}
	c.JSON(200, gin.H{
		"msg": "Sucess",
		"data": lib ,
	})
}

func GetAllLibs(c *gin.Context) {
	defer c.Abort()
	user := c.Keys["user"].(jwt.MapClaims)
	userId, _ := uuid.Parse(user["id"].(string))
	db, _ := c.Keys["db"].(*sql.DB)
	libs, err := GetAllPGLibraries(userId, db)
	if err != nil {
		c.JSON(500, gin.H{
			"msg": err,
		})
		return
	}
	c.JSON(200, gin.H{
		"msg": "Success",
		"data": libs,
	})
}