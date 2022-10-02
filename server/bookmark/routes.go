package bookmark

import (
	"database/sql"
	"fmt"
	"log"
	"net/http"
	"net/url"

	"github.com/PuerkitoBio/goquery"
	"github.com/dgrijalva/jwt-go"
	"github.com/gin-gonic/gin"

	"github.com/google/uuid"
)

func newBookMark(input *BookMarkInput, customer uuid.UUID) (BookMark, error){
	link, _ := url.Parse(input.Url)
	resp, err := http.Get(input.Url)
	if err != nil {
		return BookMark{}, err
	}
	defer resp.Body.Close()
	body, err := goquery.NewDocumentFromReader(resp.Body) 
	if err != nil {
		return BookMark{}, err
	}
	title_text := body.Find("title").Text()
	tid_, err := uuid.Parse(input.Tag) 
	tbool := true
	cbool := true
	if err !=nil {
		tbool = false
	}
	col_, err := uuid.Parse(input.Collection)

	if err != nil {
		cbool = false
	}

	tag_id := &uuid.NullUUID{
			UUID: tid_,
			Valid: tbool,
	}
	col_id := &uuid.NullUUID{
		UUID: col_,
		Valid: cbool,
	}


	return BookMark{
		ID: uuid.New(),
		Tag: *tag_id,
		Collection: *col_id,
		CustomerID: customer,
		Url: input.Url,
		Domain: link.Hostname(),
		Description: title_text,
	}, nil
}

type BookMarkInput struct {
	Url string `json:"url"`
	Collection string `json:"collection"`
	Tag string `json:"tag"`
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
	mark, err := newBookMark(&input, user_id)
	if err != nil {
		log.Println(err)
		c.JSON(500, gin.H{
			"msg": err,
		})
		return
	}
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

func GetAllBookmarks(c *gin.Context) {

	defer c.Abort()
	db, _ := c.Keys["db"].(*sql.DB)
	user := c.Keys["user"].(jwt.MapClaims)
	userId, _ := uuid.Parse(user["id"].(string))
	fmt.Println(userId)
	bookmarks, err := GetAllPGBookmarks(userId, db)
	fmt.Println(bookmarks)
	if err != nil {
		c.JSON(500, gin.H{
			"msg": "Not found!",
		})
		return
	}

	c.JSON(200, gin.H{
		"msg": "Success", 
		"data": bookmarks,
		
	})
	
}
func GetAllMarksByDomain(c *gin.Context) {
	defer c.Abort()

	db, _ := c.Keys["db"].(*sql.DB)
	user := c.Keys["user"].(jwt.MapClaims)
	userId, _ := uuid.Parse(user["id"].(string))
	domain := c.Query("domain")
	bookmarks, err := GetBookmarksByDomain(domain, userId, db)

	if err != nil {
		c.JSON(500, gin.H{
			"msg": "Not found!",
		})
		return
	}

	c.JSON(200, gin.H{
		"msg": "Success", 
		"data": bookmarks,
		
	})
}

func GetAllMarksByTag(c *gin.Context) {
	defer c.Abort()

	db, _ := c.Keys["db"].(*sql.DB)
	user := c.Keys["user"].(jwt.MapClaims)
	userId, _ := uuid.Parse(user["id"].(string))
	tag_ := c.Query("tag")
	tagId := uuid.MustParse(tag_)
	bookmarks, err := GetBookmarksByTag(tagId, userId, db)

	if err != nil {
		c.JSON(500, gin.H{
			"msg": "Not found!",
		})
		return
	}

	c.JSON(200, gin.H{
		"msg": "Success", 
		"data": bookmarks,
		
	})
}


func GetAllMarksByCollection(c *gin.Context) {
	defer c.Abort()

	db, _ := c.Keys["db"].(*sql.DB)
	user := c.Keys["user"].(jwt.MapClaims)
	userId, _ := uuid.Parse(user["id"].(string))
	col := c.Query("collection")
	colId := uuid.MustParse(col)
	bookmarks, err := GetBookmarksByCollection(colId, userId, db)

	if err != nil {
		c.JSON(500, gin.H{
			"msg": "Not found!",
		})
		return
	}

	c.JSON(200, gin.H{
		"msg": "Success", 
		"data": bookmarks,
		
	})
}