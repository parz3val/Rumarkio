package user

import (
	"context"
	"database/sql"
	"log"
	"time"

	"github.com/gin-gonic/gin"
	"github.com/google/uuid"
	"github.com/procyon-projects/chrono"
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
	validate := validateUser(user);

	if validate == 0 {
		c.JSON(400, gin.H{
			"msg": "Username or password missing",
		})
		return
	}

	// generate uuid for user id
	user.ID = uuid.New()
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
		c.JSON(400, gin.H{"Failed to create user": (user.Name)})
		return
	}

	taskScheduler := chrono.NewDefaultTaskScheduler()
	go func() {
		now := time.Now()
		startTime := now.Add(time.Second * 30)
		_, err := taskScheduler.Schedule(func(ctx context.Context) {
			SendVerificationEmail(existing_user)
	}, chrono.WithTime(startTime))

	if err == nil {
		log.Print("Email task scheduled successfully!")
	}

	}()
	taskScheduler.Shutdown() //
	// The Shutdown() method doesn't cause immediate shut down of the Scheduler and returns a channel. 
	// It will make the Scheduler stop accepting new tasks and shut down after all running tasks finish their current work.
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
		return
	}

	db, val := c.Keys["db"].(*sql.DB)
	if !val  {
		log.Println(db)
		log.Println(val)
	}
	existing_user, err := GetUserByEmail(db, details.Email)


	if err != nil {
		c.JSON(500, gin.H{"msg": err})
		return
	}
	err = bcrypt.CompareHashAndPassword([]byte(existing_user.Password), []byte(details.Password))
	if err == nil {
		log.Println(existing_user)
		existing_ID, _ := uuid.Parse(existing_user.ID)
		token, err := jwtauth.GenerateJWT(existing_user.Email, existing_user.Name, (existing_ID) )
		log.Println("The token is", token)
		if err != nil {
			c.JSON(500, gin.H{"msg": err})
			return
		}
		c.JSON(200, gin.H{"accessToken": token})
		return
	} else {
		c.JSON(400, gin.H{"msg": "Username or password not correct"})
		return
	}
}

func UserInfo(c *gin.Context) {
	c.JSON(200, gin.H{ "msg": "Secret API"})
}