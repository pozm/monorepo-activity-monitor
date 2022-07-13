package main

import (
	"fmt"
	"mime/multipart"
	"net/http"
	"os"
	"strconv"
	"strings"
	"time"

	"github.com/gin-contrib/cors"
	"github.com/gin-contrib/static"
	"github.com/gin-gonic/gin"
	"github.com/gin-gonic/gin/binding"
	"github.com/google/uuid"
	"github.com/gorilla/websocket"
	_ "github.com/joho/godotenv/autoload"
	lop "github.com/samber/lo/parallel"
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

	// gin.SetMode(gin.ReleaseMode)

	// gin.DisableConsoleColor()
	r := gin.Default()

	r.Use(cors.Default())
	// r.Use(func(ctx *gin.Context) {
	// 	ctx.Header("Access-Control-Allow-Origin", "*")
	// })

	InitDB("test.db")

	db.AutoMigrate(&UserData{}, &ActivityData{}, &DeviceData{}, &DeviceActivity{})

	// var tempUser UserData
	// db.FirstOrCreate(&tempUser, UserData{Name: "test"})
	var tempUser2 UserData
	db.FirstOrCreate(&tempUser2, UserData{Name: "Luna"})
	var tempUser3 UserData
	db.FirstOrCreate(&tempUser3, UserData{Name: "MacBookAirUser12"})
	fmt.Printf("luna key: %s\n", tempUser2.ApiKey)

	// db.Save(tempUser)
	r.MaxMultipartMemory = 8 << 20 // 8 MiB // stop allowing larger images for etc
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

	api := r.Group("/api")

	user := api.Group("/user")

	app := api.Group("/app", Authmw())

	dvc := api.Group("/device", Authmw())

	// get many users
	user.GET("/", func(c *gin.Context) {
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
	// user data
	user.GET("/:name", func(c *gin.Context) {

		name := c.Param("name")

		type ScanRes struct {
			Act      uint
			DaID     int
			Ddi      uuid.UUID
			Mins     int
			UserName string
			ActName  string
			Dau      time.Time
			Dac      time.Time
			active   bool
		}
		var resultScan []ScanRes

		db.Raw(`select ad.id as act, coalesce(da.id, -1) as da_id, da.device_data_id ddi, coalesce(da.mins_total, 0) as mins, ud.name as user_name, ad.name as act_name, da.updated_at as dau, da.created_at as dac, da.active active from user_data ud
        inner join activity_data ad on ud.id = ad.user_data_id
        left outer join device_activities da on ad.id = da.activity_data_id
where lower(ud.name) = ?`, name).Scan(&resultScan)

		if len(resultScan) == 0 {

			c.JSON(http.StatusNotFound, gin.H{"error": "user not found"})
			return
		}

		var deviceData []DeviceData

		db.Where("device_id in ?", lop.Map(resultScan, func(res ScanRes, _ int) uuid.UUID {
			return res.Ddi
		})).Find(&deviceData)

		// db.Preload("Activities").Preload("Activities.Devices").Joins("Device_Data","DeviceId = ?",).Model(&UserData{}).First(&user, "UPPER(\"user_data\".\"name\") = UPPER(?)", name)

		type PublicActData struct {
			ActivityId uint              `json:"activity_id"`
			Devices    map[uuid.UUID]int `json:"devices"`
			MinsTotal  int               `json:"mins_total"`
			UpdatedAt  time.Time         `json:"updated_at"`
			CreatedAt  time.Time         `json:"created_at"`
			Active     bool              `json:"active"`
		}
		type PublicData struct {
			Name       string                   `json:"name"`
			Activities map[string]PublicActData `json:"activities"`
			Devices    []DeviceData             `json:"devices"`
		}

		var publicData PublicData = PublicData{Name: resultScan[0].UserName, Devices: deviceData, Activities: make(map[string]PublicActData)}
		for _, act := range resultScan {
			existing, ok := publicData.Activities[act.ActName]
			if ok {

				existing.Devices[act.Ddi] = act.Mins
				existing.MinsTotal += act.Mins
				if existing.UpdatedAt.Before(act.Dau) {
					existing.UpdatedAt = act.Dau
				}
				publicData.Activities[act.ActName] = existing
				continue
			}
			if act.DaID == -1 {
				publicData.Activities[act.ActName] = PublicActData{ActivityId: act.Act, MinsTotal: act.Mins, UpdatedAt: act.Dac, CreatedAt: act.Dau, Active: act.active, Devices: make(map[uuid.UUID]int)}
				continue
			}
			publicData.Activities[act.ActName] = PublicActData{ActivityId: act.Act, MinsTotal: act.Mins, UpdatedAt: act.Dac, CreatedAt: act.Dau, Active: act.active, Devices: map[uuid.UUID]int{act.Ddi: act.Mins}}
		}

		c.JSON(http.StatusOK, publicData)

	})

	/* select mins_total, dd.name as device_name, ad.name as activity_name, ud.name as user_name, ud.id as user_id from device_activities
	    inner join activity_data ad on ad.id = device_activities.activity_data_id
	    inner join device_data dd on dd.device_id = device_activities.device_data_id::uuid
	    inner join user_data ud on ud.id = dd.user_data_id
	where lower(dd.name) = lower(?) and lower(ad.name) = lower(?) and ud.id = ? */

	// user image
	user.GET("/:name/img/", func(ctx *gin.Context) {
		name := ctx.Param("name")
		ctx.File(fmt.Sprintf("./icons/users/%s.png", name))
	})
	// activity image
	user.GET("/:name/img/:activity", func(ctx *gin.Context) {
		act := ctx.Param("activity")
		name := ctx.Param("name")
		loc := fmt.Sprintf("./icons/%s%s.png", name, act)
		stripped := strings.ReplaceAll(loc, " ", "")
		lower := strings.ToLower(stripped)
		fmt.Printf("searching for %s\n", lower)
		ctx.File(lower)
	})

	// update activity
	app.PATCH("/", func(ctx *gin.Context) {
		var ud UserData = ctx.MustGet("user").(UserData)
		type IncomingData struct {
			Active     bool   `json:"active"`
			Activity   string `json:"activity"`
			DeviceName string `json:"dName"`
			Time       uint64 `json:"time"`
		}
		var data IncomingData
		if err := ctx.Bind(&data); err != nil || data.Activity == "" {
			ctx.JSON(http.StatusBadRequest, gin.H{"error": "bad request"})
			return
		}
		type ScanRes struct {
			Act uint
			Dev uuid.UUID
			Da  int
		}
		var resultScan ScanRes

		db.Raw(`select ad.id as act, dd.device_id as dev, coalesce(da.id, -1) as da from user_data ud
		inner join activity_data ad on ud.id = ad.user_data_id
		inner join device_data dd on ud.id = dd.user_data_id
		left outer join device_activities da on ad.id = da.activity_data_id and da.device_data_id = dd.device_id::text
	where lower(dd.name) = lower(?) and lower(ad.name) = lower(?) and ud.id = ?`, data.DeviceName, data.Activity, ud.ID).Scan(&resultScan)

		// db.Model(&ActivityData{}).Where("User_Data_ID = ? AND name  = ?", ud.ID, data.Activity).First(&act)
		fmt.Printf("%+v\n", resultScan)
		if resultScan.Act == 0 {
			ctx.JSON(http.StatusBadRequest, gin.H{"error": "activity not found"})
			return
		}
		if resultScan.Da == -1 {
			db.Model(&DeviceActivity{}).Create(&DeviceActivity{ActivityDataID: resultScan.Act, DeviceDataID: resultScan.Dev, Active: data.Active, MinsTotal: data.Time})
		} else {
			if data.Active {
				db.Model(&DeviceActivity{}).Where("device_data_id = ? AND active = false and activity_data_id = ?", resultScan.Dev, resultScan.Act).Updates(map[string]interface{}{
					"mins_total": gorm.Expr("device_activities.mins_total + ?", data.Time),
					"active":     data.Active,
				})

			} else {
				db.Model(&DeviceActivity{}).Where("device_data_id = ? AND activity_data_id = ?", resultScan.Dev, resultScan.Act).Updates(map[string]interface{}{
					"mins_total": gorm.Expr("mins_total + floor(extract(EPOCH FROM now() - updated_at)/60)"),
					"active":     data.Active,
				})

			}
		}

	})
	// edit activity
	app.PUT("/", func(ctx *gin.Context) {
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
	// delete activity
	app.DELETE("/:activity", func(c *gin.Context) {
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
	// create activity
	app.POST("/new", func(c *gin.Context) {

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
		out := db.Model(&u).Association("Activities").Append(&ActivityData{Name: newApp.Name})
		fmt.Printf("%+v\n save to %s\n", out, lower)
	})
	// get all devices
	dvc.GET("/", func(c *gin.Context) {
		var ud UserData = c.MustGet("user").(UserData)
		var d []DeviceData
		db.Model(&DeviceData{}).Where("User_Data_ID = ?", ud.ID).Find(&d)
		c.JSON(http.StatusOK, d)
	})
	// create device
	dvc.POST("/new", func(c *gin.Context) {
		var ud UserData = c.MustGet("user").(UserData)
		deviceName := c.DefaultQuery("name", "Device")
		d := DeviceData{UserDataID: ud.ID, Name: deviceName}
		db.Create(&d)
		c.JSON(http.StatusOK, d)
	})

	r.Use(static.Serve("/", static.LocalFile("./html", false)))
	r.NoRoute(func(ctx *gin.Context) {
		ctx.Header("xurl", ctx.Request.URL.Path)
		println(ctx.Request.URL.Path)
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
