package webapi

import (
	"log"

	"github.com/gin-gonic/gin"
	jwtauth "poggybitz.com/ruserver/jwtAuth"
)

func Cors() gin.HandlerFunc {
	return func(c *gin.Context) {
		c.Writer.Header().Add("Access-Control-Allow-Origin", "*")
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
		context.Next()
	}

}