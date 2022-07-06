package main

import (
	"fmt"
	"log"
	"os"

	"github.com/google/uuid"
	"gorm.io/driver/postgres"
	"gorm.io/gorm"
	"gorm.io/gorm/logger"
)

var db *gorm.DB

func InitDB(dataSourceName string) {
	var err error
	db, err = gorm.Open(postgres.Open(
		fmt.Sprintf("host=%s user=%s password=%s dbname=postgres sslmode=disable",
			os.Getenv("psqla"),
			os.Getenv("psqlu"),
			os.Getenv("psqlp"),
		)), &gorm.Config{
		Logger: logger.Default.LogMode(logger.Info),
	})
	if err != nil {
		log.Panic(err)
	}

	if _, err = db.DB(); err != nil {
		log.Panic(err)
	}
}

type UserData struct {
	gorm.Model
	Name        string `json:"name" binding:"required" gorm:"unique"`
	DisplayName string `json:"display_name" binding:"required"`
	Activities  []ActivityData
	Devices     []DeviceData
	ApiKey      uuid.UUID `gorm:"type:uuid;default:uuid_generate_v4()"`
	Role        string    `json:"role" gorm:"default:'user'"`
}
type ActivityData struct {
	gorm.Model
	UserDataID uint
	Name       string           `json:"name" binding:"required"`
	Devices    []DeviceActivity `json:"devices"`
}
type DeviceData struct {
	gorm.Model
	UserDataID uint
	Name       string    `json:"name" binding:"required"`
	DeviceId   uuid.UUID `json:"deviceId" binding:"required" gorm:"type:uuid;default:uuid_generate_v4();primaryKey"`
}
type DeviceActivity struct {
	gorm.Model
	ActivityDataID uint
	DeviceDataID   uuid.UUID
	MinsTotal      uint64 `json:"minsTotal" binding:"required"`
	Active         bool   `json:"active" binding:"required"`
}
