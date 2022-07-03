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
	Name       string `json:"name" binding:"required" gorm:"unique"`
	Activities []ActivityData
	ApiKey     uuid.UUID `gorm:"type:uuid;default:uuid_generate_v4()"`
}
type ActivityData struct {
	gorm.Model
	UserDataID uint
	Name       string `json:"name" binding:"required"`
	MinsTotal  uint64 `json:"minsTotal" binding:"required"`
	Active     bool   `json:"active" binding:"required"`
}
