package webapi

import (
	"github.com/gin-gonic/gin"
	"poggybitz.com/ruserver/library"
	"poggybitz.com/ruserver/user"
)


var Router *gin.Engine

func SetupRoutes() {
	Router = gin.Default()

	Router.Use(Cors())

	v1 := Router.Group("/v1")
	{
		v1.POST("/register", user.Register)
		v1.POST("/login", user.Login)

		secured := v1.Group("/secured").Use(AuthMiddleWare())
		{
			secured.POST("/userinfo", user.UserInfo )
		}

		libs := v1.Group("/lib").Use(AuthMiddleWare())
		{
			libs.POST("/new", library.CreateLibrary)
		}
	}

}