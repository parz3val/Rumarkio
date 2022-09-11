package library

import (
	"database/sql"
	"log"
	"net/http"
	"net/url"

	"github.com/PuerkitoBio/goquery"
	"github.com/gin-gonic/gin"
)



func CreateLibrary(c *gin.Context) {
	var lib Library
	c.Bind(&lib)
	
	// TO DO : Validate the library
	db, _ := c.Keys["db"].(*sql.DB)
	err := CreatePGLibrary(db, &lib)
	if err != nil {
		log.Println(err)
		c.JSON(400, gin.H {
			"msg": "Couldn't create library",
		})
	}
}

type GetUserLibsInput struct {
	User int `json:"user"`
}
func GetLibraries(c *gin.Context) {
	var user GetUserLibsInput
	c.Bind(&user)
	
	db, _ := c.Keys["db"].(*sql.DB)

	data, err := GetUserLibraries(db, user.User)

	if err != nil {
		log.Println(err)
		c.JSON(500, gin.H{
			"msg": err,
		})
		c.Abort()
		return
	}

	c.JSON(200, gin.H{
		"msg": data,
	})

}
type CollectionInput struct {
	Name string `json:"name"`
	Library uint64 `json:"library"`
}
func CreateCollection(c *gin.Context) {
	var input CollectionInput
	c.Bind(&input)

	db, _ := c.Keys["db"].(*sql.DB)
    col := Collection{
		Name: input.Name,
		Library: input.Library,
	}
	err := CreatePGCollection(db, &col);
	if err!= nil {
		log.Println(err)
		c.JSON(500, gin.H{
			"msg": err,
		})
		return
	}
	c.JSON(200, gin.H{
		"msg": "Sucess",
		"data": col,
	})

}
type TagInput struct {
	Name string `json:"name"`
	User uint64 `json:"user"`
}
func CreateTag(c *gin.Context) {
	var input TagInput
	c.Bind(&input)

	tag := Tag{
		Name: input.Name,
		UserId: input.User,
	}

	db, _ := c.Keys["db"].(*sql.DB)

	err := CreatePGTag(db, &tag)
	if err!= nil {
		log.Println(err)
		c.JSON(500, gin.H{
			"msg": err,
		})
		return
	}
	c.JSON(200, gin.H{
		"msg": "Sucess",
		"data": tag,
	})
}

type BookMarkInput struct {
	Url string `json:"url"`
	Collection uint64 `json:"collection"`
	Tag uint64 `json:"tag"`
}
func CreateBookMark(c *gin.Context) {
	var input BookMarkInput
	c.Bind(&input)

	db, _ := c.Keys["db"].(*sql.DB)

	mark := newBookMark(&input)
	err := CreatePGBookMark(db, &mark)
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

func newBookMark(input *BookMarkInput) BookMark {
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
		Str: input.Url,
		Domain: link.Hostname(),
		Collection: input.Collection,
		Tag: input.Tag,
		Description: title_text,
	}
}

