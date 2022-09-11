package webapi

import (
	"database/sql"
	"fmt"
	"log"

	"github.com/gin-gonic/gin"
	_ "github.com/lib/pq" //
	jwtauth "poggybitz.com/ruserver/jwtAuth"
)

var DB *sql.DB
func Cors() gin.HandlerFunc {
	return func(c *gin.Context) {
		c.Writer.Header().Add("Access-Control-Allow-Origin", "*")
		c.Set("db", DB)
		c.Next()
	}
}


func AuthMiddleWare() gin.HandlerFunc {
	return func(context *gin.Context) {
		tokenStr := context.GetHeader("Authorization")
		if tokenStr == ""{
			context.JSON(500, gin.H{"msg": "Missing JWT"})
			context.Abort()
			return
		}
		log.Println(tokenStr)
		err := jwtauth.ValidateToken(tokenStr)

		if err != nil {
			context.JSON(401, gin.H{"msg": "Unauthorized"})
			context.Abort()
			return
		}
		claim, err := jwtauth.ExtractClaims(tokenStr)
		if err != nil {
			log.Println(err)
		}
		log.Println(claim)

		context.Next()
	}

}


func setupPOSTGRES() {
	dsn := "host=0.0.0.0 port=5432 user=postgres password=rumarkio dbname=rumarkio_test sslmode=disable"
	var err error
	DB, err = sql.Open("postgres", dsn)

	if err != nil {
		panic(err)
	}
	err = DB.Ping()

	if err != nil {
		fmt.Println(err)
	}
}

func Setup() {
	setupPOSTGRES()
}
