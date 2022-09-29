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
type CollectionInput struct {
	Name string `json:"name"`
	Library uint64 `json:"library"`
}
func CreateCollection(c *gin.Context) {
	var input CollectionInput
	c.Bind(&input)

	db, _ := c.Keys["db"].(*sql.DB)
    col := Collection{
		Name: input.Name,
		Library: input.Library,
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
type TagInput struct {
	Name string `json:"name"`
	User uint64 `json:"user"`
}
func CreateTag(c *gin.Context) {
	var input TagInput
	c.Bind(&input)

	tag := Tag{
		Name: input.Name,
		UserId: input.User,
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


// func wreateBookMark(c *gin.Context) {
// 	var input BookMarkInput
// 	c.Bind(&input)

// 	db, _ := c.Keys["db"].(*sql.DB)

// 	mark := newBookMark(&input)
// 	err := CreatePGBookMark(db, &mark)
// 	if err!= nil {
// 		log.Println(err)
// 		c.JSON(500, gin.H{
// 			"msg": err,
// 		})
// 		return
// 	}

// 	c.JSON(200, gin.H{
// 		"msg": "Success",
// 		"data": mark,
// 	})
// }

