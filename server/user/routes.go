package user

import (
	"log"

	"github.com/gin-gonic/gin"
	"golang.org/x/crypto/bcrypt"
)
func validateUser(user User) int{
	if user.Email != "" && user.Name != "" && user.Password != ""{
		return 1
	}
	return 0
}

func Register(c *gin.Context) {
	// get apop
	var user User
	c.Bind(&user)
	log.Println(user)
	validate := validateUser(user);

	if validate == 0 {
		c.JSON(400, gin.H{
			"msg": "Username or password missing",
		})
		return
	}

	password, bcErr := bcrypt.GenerateFromPassword([]byte(user.Password), 14)
	existing_user, _  := GetUserByEmail(user.Email)
	if existing_user.Email != "" {	
		c.JSON(400, gin.H{
			"msg": "User already exists",
		})

	}
	if bcErr != nil {
		c.JSON(400, gin.H{"Encryption Error": "Failed"})
	}
	user.Password = string(password)
	err := CreatePGUser(&user)
	if err != nil {
		log.Println(err)
		c.JSON(400, gin.H{"Encryption Error": "Failed"})
		return
	}

	// return the created user
	c.JSON(200, gin.H{
		"msg": "Registration Successful!",
	})
	
}

func Login(c *gin.Context) {
	// get req
	var details LoginDeets
	c.Bind(&details)
	if details.Email == "" || details.Password == ""{
		c.JSON(400, gin.H{"msg": "Email or password missing"})
	}

	existing_user, err := GetUserByEmail(details.Email)

	if err != nil {
		c.JSON(500, gin.H{"msg": err})
	}
	err = bcrypt.CompareHashAndPassword([]byte(existing_user.Password), []byte(details.Password))
	if err == nil {
		c.JSON(200, existing_user)
	} else {
		c.JSON(400, gin.H{"msg": "Username or password not correct"})
	}
}