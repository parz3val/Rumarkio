package user

import (
	"database/sql"
	"log"

	"github.com/gin-gonic/gin"
	"golang.org/x/crypto/bcrypt"
	jwtauth "poggybitz.com/ruserver/jwtAuth"
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
	db, val := c.Keys["db"].(*sql.DB)
	if val == false {
		log.Println(db)
		log.Println(val)
	}
	existing_user, _  := GetUserByEmail(db, user.Email)
	if existing_user.Email != "" {	
		c.JSON(400, gin.H{
			"msg": "User already exists",
		})

	}
	if bcErr != nil {
		c.JSON(400, gin.H{"Encryption Error": "Failed"})
	}
	user.Password = string(password)
	err := CreatePGUser(db, &user)
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
	log.Println(details)
	if details.Email == "" || details.Password == ""{
		c.JSON(400, gin.H{"msg": "Email or password missing"})
		c.Abort()
		return 
	}

	db, val := c.Keys["db"].(*sql.DB)
	if val == false {
		log.Println(db)
		log.Println(val)
	}
	existing_user, err := GetUserByEmail(db, details.Email)
	log.Println(err)
	if err != nil {
		c.JSON(500, gin.H{"msg": err})
		c.Abort()
		return
	}
	err = bcrypt.CompareHashAndPassword([]byte(existing_user.Password), []byte(details.Password))
	if err == nil {
		log.Println(existing_user)
		token, err := jwtauth.GenerateJWT(existing_user.Email, existing_user.Name, int(existing_user.ID))
		if err != nil {
			c.JSON(500, gin.H{"msg": err})
		}
		c.JSON(200, gin.H{"accessToken": token})
	} else {
		c.JSON(400, gin.H{"msg": "Username or password not correct"})
		c.Abort()
		return
	}
}

func UserInfo(c *gin.Context) {
	c.JSON(200, gin.H{ "msg": "Secret API"})
}