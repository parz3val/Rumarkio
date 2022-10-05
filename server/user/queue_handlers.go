package user

import "fmt"

func SendVerificationEmail(user UserData) {
	fmt.Println("***********************")
	fmt.Printf("Sent email to {}", user.Email)
	fmt.Println("***********************")
}