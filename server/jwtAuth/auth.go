package jwtauth

import (
	"errors"
	"log"
	"time"

	"github.com/dgrijalva/jwt-go"
	"github.com/google/uuid"
)
var jwtKey = []byte("10196137")

type JWTClaim struct {
	User string `json:"user"`
	Email string `json:"email"`
	ID uuid.UUID `json:"id"`
	jwt.StandardClaims
}

func GenerateJWT(email string, username string, id uuid.UUID) (tokenString string, err error) {
	expirationTime := time.Now().Add(1 * time.Hour)
	claims:= &JWTClaim{
		Email: email,
		User: username,
		ID: id,
		StandardClaims: jwt.StandardClaims{
			ExpiresAt: expirationTime.Unix(),
		},
	}
	token := jwt.NewWithClaims(jwt.SigningMethodHS256, claims)
	tokenString, err = token.SignedString(jwtKey)
	return
}

func ValidateToken(signedToken string) (err error) {
	token, err := jwt.ParseWithClaims(
		signedToken,
		&JWTClaim{},
		func(token *jwt.Token) (interface{}, error) {
			return []byte(jwtKey), nil
		},
	)
	if err != nil {
		return
	}
	claims, ok := token.Claims.(*JWTClaim)
	if !ok {
		err = errors.New("couldn't parse claims")
		return
	}
	if claims.ExpiresAt < time.Now().Local().Unix() {
		err = errors.New("token expired")
		return
	}
	return
}

func ExtractClaims(tokenStr string) (jwt.MapClaims, error) {
	hmacSecret := jwtKey 
	token, err := jwt.Parse(tokenStr, func(token *jwt.Token) (interface{}, error) {
			// check token signing method etc
			return hmacSecret, nil
	})

	if err != nil {
		return nil, err
	}

	if claims, ok := token.Claims.(jwt.MapClaims); ok && token.Valid {
		return claims, nil
	} else {
		log.Printf("Invalid JWT Token")
		return nil, err
	}
}