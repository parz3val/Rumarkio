package library

import (
	"database/sql"
	"log"

	"github.com/dgrijalva/jwt-go"
	"github.com/gin-gonic/gin"
)



func CreateLibrary(c *gin.Context) {
	// parse the user info
	user := c.Keys["user"].(jwt.MapClaims)
	var lib Library
	c.Bind(&lib)
	log.Println("The user is :: ##")
	log.Println(user)
	// set a default name for the library if no name is provided

	if userId, found := user["id"]; found {
		userF := userId.(float64)
		lib.UserID = uint64(userF)
	} 
	
	if lib.Name == "" {
		lib.Name = "defaultlib"
	}

	log.Println(lib)

	// TO DO : Validate the library
	db, _ := c.Keys["db"].(*sql.DB)
	err := CreatePGLibrary(db, &lib)
	if err != nil {
		log.Println(err)
		c.JSON(400, gin.H {
			"msg": "Couldn't create library",
		})
	}
}

type GetUserLibsInput struct {
	User int `json:"user"`
}
func GetLibraries(c *gin.Context) {
	var user GetUserLibsInput
	c.Bind(&user)
	
	db, _ := c.Keys["db"].(*sql.DB)

	data, err := GetUserLibraries(db, user.User)

	if err != nil {
		log.Println(err)
		c.JSON(500, gin.H{
			"msg": err,
		})
		c.Abort()
		return
	}

	c.JSON(200, gin.H{
		"msg": data,
	})

}



