package webapi

import (
	"github.com/gin-gonic/gin"
	"poggybitz.com/ruserver/bookmark"
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
			libs.POST("/get", library.GetLibraries)
		}

		cols := v1.Group("/collection").Use(AuthMiddleWare())
		{
			cols.POST("/new", library.CreateCollection)
		}

		tags := v1.Group("/tag").Use(AuthMiddleWare())
		{
			tags.POST("/new", library.CreateTag)
		}
		
		marks := v1.Group("/marks").Use(AuthMiddleWare())
		{
			marks.POST("/new", bookmark.CreateBookmark)
			marks.POST("/all", bookmark.GetAll)
		}
	}

}