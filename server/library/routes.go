package library

import (
	"database/sql"
	"log"

	"github.com/gin-gonic/gin"
)



func CreateLibrary(c *gin.Context) {
	var lib Library
	c.Bind(&lib)
	
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