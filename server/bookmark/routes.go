package bookmark

import (
	"database/sql"
	"log"
	"net/http"
	"net/url"

	"github.com/PuerkitoBio/goquery"
	"github.com/dgrijalva/jwt-go"
	"github.com/gin-gonic/gin"

	"github.com/google/uuid"
)

func newBookMark(input *BookMarkInput, customer uuid.UUID) BookMark {
	link, _ := url.Parse(input.Url)
	resp, err := http.Get(input.Url)
	if err != nil {
		return BookMark{}
	}
	defer resp.Body.Close()
	body, err := goquery.NewDocumentFromReader(resp.Body) 
	if err != nil {
		return BookMark{}
	}
	title_text := body.Find("title").Text()
	log.Println(title_text)
	return BookMark{
		ID: uuid.New(),
		CustomerID: customer,
		UrlStr: input.Url,
		Domain: link.Hostname(),
		Description: title_text,
	}
}

type BookMarkInput struct {
	Url string `json:"url"`
}


func CreateBookmark(c *gin.Context) {
	var input BookMarkInput
	c.Bind(&input)

	db, _ := c.Keys["db"].(*sql.DB)
	user :=  c.Keys["user"].(jwt.MapClaims)
	user_id, err := uuid.Parse(user["id"].(string))

	if err != nil {
		log.Println(err)
		c.JSON(500, gin.H{
			"msg": "Invalid User Id",
		})
		return
	}
	mark := newBookMark(&input, user_id)

	err = CreatePGBookMark(db, &mark)

	if err!= nil {
		log.Println(err)
		c.JSON(500, gin.H{
			"msg": err,
		})
		return
	}

	c.JSON(200, gin.H{
		"msg": "Success",
		"data": mark,
	})
}

func GetAll(c *gin.Context) {
	
}