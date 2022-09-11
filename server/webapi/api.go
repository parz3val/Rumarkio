package webapi

import (
	"poggybitz.com/ruserver/user"
)
func Server() {
	// setup db
	user.Setup()// listen and server localhost:8080

	SetupRoutes()
	Router.Run(":8080")


}