package webapi
func Server() {
	// setup db
	Setup()// listen and server localhost:8080

	SetupRoutes()
	Router.Run(":8080")


}