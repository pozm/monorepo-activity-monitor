package main

import (
	"fmt"
	"mime/multipart"
	"net/http"
	"os"
	"strconv"
	"strings"

	"github.com/gin-contrib/cors"
	"github.com/gin-contrib/static"
	"github.com/gin-gonic/gin"
	"github.com/gin-gonic/gin/binding"
	"github.com/gorilla/websocket"
	_ "github.com/joho/godotenv/autoload"
	"gorm.io/gorm"
)

func Authmw() gin.HandlerFunc {
	return func(c *gin.Context) {
		var foundUser UserData
		tx := db.First(&foundUser, "Api_Key = ?", c.GetHeader("ApiKey"))
		if tx.Error != nil {
			c.AbortWithStatus(400)
		}
		c.Set("user", foundUser)
		// c.Next()
	}
}

var upgrader = websocket.Upgrader{}

func setupRouter() *gin.Engine {
	// Disable Console Color

	gin.SetMode(gin.ReleaseMode)

	// gin.DisableConsoleColor()
	r := gin.Default()

	r.Use(cors.Default())
	// r.Use(func(ctx *gin.Context) {
	// 	ctx.Header("Access-Control-Allow-Origin", "*")
	// })

	InitDB("test.db")

	db.AutoMigrate(&UserData{}, &ActivityData{})

	// var tempUser UserData
	// db.FirstOrCreate(&tempUser, UserData{Name: "test"})
	var tempUser2 UserData
	db.FirstOrCreate(&tempUser2, UserData{Name: "Luna"})
	var tempUser3 UserData
	db.FirstOrCreate(&tempUser3, UserData{Name: "MacBookAirUser12"})
	fmt.Printf("luna key: %s\n", tempUser2.ApiKey)

	// db.Save(tempUser)
	r.MaxMultipartMemory = 8 << 20 // 8 MiB
	type PublicUserData struct {
		gorm.Model
		// ID         uint           `json:"id"`
		Name       string         `json:"name"`
		Activities []ActivityData `json:"activities" gorm:"foreignKey:UserDataID"`
	}

	type PublicUserSmallData struct {
		gorm.Model
		// ID         uint           `json:"id"`
		Name       string         `json:"name"`
		Activities []ActivityData `json:"activities" gorm:"foreignKey:UserDataID"`
	}

	r.GET("/ws", func(c *gin.Context) {

		ws, err := upgrader.Upgrade(c.Writer, c.Request, nil)
		if err != nil {
			println(err.Error())
		}
		defer ws.Close()
		for {
			type JsonWebsocketData struct {
				Type string `json:"type"`
				Data string `json:"data"`
			}
			var data JsonWebsocketData
			err = ws.ReadJSON(&data)
			if err != nil {
				println(err.Error())
				break
			}
		}

	})

	r.GET("/api/users", func(c *gin.Context) {
		last := c.DefaultQuery("last", "0")
		var users []PublicUserSmallData

		lasti, err := strconv.Atoi(last)
		if err != nil {
			c.AbortWithStatusJSON(400, gin.H{"error": "last must be an integer"})
			return
		}

		db.Preload("Activities").Model(&UserData{}).Limit(10).Where("user_data.id > (? - 1)", lasti).Find(&users)

		c.JSON(http.StatusOK, users)
	})

	user := r.Group("/api/user/:name")

	user.GET("/img/:activity", func(ctx *gin.Context) {
		act := ctx.Param("activity")
		name := ctx.Param("name")
		loc := fmt.Sprintf("./icons/%s%s.png", name, act)
		stripped := strings.ReplaceAll(loc, " ", "")
		lower := strings.ToLower(stripped)
		fmt.Printf("searching for %s\n", lower)
		ctx.File(lower)
	})

	user.GET("/", func(c *gin.Context) {

		name := c.Param("name")

		var user PublicUserData

		db.Preload("Activities").Model(&UserData{}).First(&user, "UPPER(\"user_data\".\"name\") = UPPER(?)", name)

		if user.Name == "" {

			c.JSON(http.StatusNotFound, gin.H{"error": "user not found"})
			return
		}

		c.JSON(http.StatusOK, user)

	})

	r.PATCH("/api/app", Authmw(), func(ctx *gin.Context) {
		var ud UserData = ctx.MustGet("user").(UserData)
		type IncomingData struct {
			Active   bool   `json:"active"`
			Activity string `json:"activity"`
			Time     uint64 `json:"time"`
		}
		var data IncomingData
		if err := ctx.Bind(&data); err != nil || data.Activity == "" {
			ctx.JSON(http.StatusBadRequest, gin.H{"error": "bad request"})
			return
		}
		fmt.Printf("%+v\n", data)
		if data.Active {
			db.Model(&ActivityData{}).Where("User_Data_ID = ? AND name = ?", ud.ID, data.Activity).Updates(map[string]interface{}{"mins_total": gorm.Expr("mins_total + ?", data.Time), "active": data.Active})
		} else {
			db.Model(&ActivityData{}).Where("User_Data_ID = ? AND name = ?", ud.ID, data.Activity).Updates(map[string]interface{}{"mins_total": gorm.Expr("mins_total + floor(extract(EPOCH FROM now() - updated_at)/60)"), "active": data.Active})

		}

	})
	r.PUT("/api/app", Authmw(), func(ctx *gin.Context) {
		var ud UserData = ctx.MustGet("user").(UserData)
		type AppForm struct {
			Name    string                `form:"name" binding:"required"`
			NewName string                `form:"name" binding:"required"`
			File    *multipart.FileHeader `form:"file" binding:"-"`
		}
		var data AppForm
		if err := ctx.Bind(&data); err != nil || data.Name == "" {
			ctx.JSON(http.StatusBadRequest, gin.H{"error": "bad request"})
			return
		}
		locOld := fmt.Sprintf("./icons/%s%s.png", ud.Name, data.Name)
		stripped := strings.ReplaceAll(locOld, " ", "")
		lower := strings.ToLower(stripped)
		locNew := fmt.Sprintf("./icons/%s%s.png", ud.Name, data.NewName)
		stripped2 := strings.ReplaceAll(locNew, " ", "")
		lower2 := strings.ToLower(stripped2)
		os.Rename(lower, lower2)
		db.Model(&ActivityData{}).Where("User_Data_ID = ? AND name = ?", ud.ID, data.Name).Updates(
			map[string]interface{}{
				"mins_total": gorm.Expr("mins_total + floor(extract(EPOCH FROM now() - updated_at)/60)"),
				"active":     false,
				"name":       data.NewName,
			},
		)
		if data.File != nil {
			ctx.SaveUploadedFile(data.File, lower2)
		}
		ctx.JSON(http.StatusOK, gin.H{"success": "updated"})
	})

	r.DELETE("/api/app/:activity", Authmw(), func(c *gin.Context) {
		var ud UserData = c.MustGet("user").(UserData)
		act := c.Param("activity")
		if tx := db.Delete(&ActivityData{}, "User_Data_ID = ? AND name = ?", ud.ID, act); tx.Error != nil {
			c.JSON(http.StatusBadRequest, gin.H{"error": "bad request"})
			return
		}
		loc := fmt.Sprintf("./icons/%s%s.png", ud.Name, act)
		os.Remove(loc)
		c.JSON(http.StatusOK, gin.H{"success": "deleted"})

	})

	r.POST("/api/newApp", Authmw(), func(c *gin.Context) {

		type AppForm struct {
			Name string                `form:"name" binding:"required"`
			File *multipart.FileHeader `form:"file" binding:"required"`
		}

		var newApp AppForm

		if err := c.MustBindWith(&newApp, binding.FormMultipart); err != nil {
			fmt.Printf("bbbruiuuh %+v\n", err)
			c.JSON(http.StatusBadRequest, gin.H{"error": "bad request"})
			return
		}

		os.MkdirAll("./icons", os.ModePerm)

		var g ActivityData
		u := c.MustGet("user").(UserData)
		db.Model(&u).Association("Activities").Find(&g, "name = ?", newApp.Name)
		if g.ID != 0 {
			c.JSON(http.StatusBadRequest, gin.H{"error": "already exist"})
			return
		}
		loc := fmt.Sprintf("./icons/%s%s.png", u.Name, newApp.Name)
		stripped := strings.ReplaceAll(loc, " ", "")
		lower := strings.ToLower(stripped)
		c.SaveUploadedFile(newApp.File, lower)
		out := db.Model(&u).Association("Activities").Append(&ActivityData{Name: newApp.Name, MinsTotal: 0})
		fmt.Printf("%+v\n save to %s\n", out, lower)
	})

	r.Use(static.Serve("/", static.LocalFile("./html", false)))
	r.NoRoute(func(ctx *gin.Context) {
		ctx.File("./html/index.html")
	})
	return r
}

func main() {
	r := setupRouter()
	r.SetTrustedProxies([]string{"localhost", "127.0.0.1"})
	// Listen and Server in 0.0.0.0:8080
	r.Run(":9174")
}
