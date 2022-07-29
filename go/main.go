package main

import "github.com/gin-gonic/gin"

type Message struct {
    Text string `json:"text"`
}

func index(c *gin.Context) {
    c.JSON(200, Message { Text: "Hello World" })
}


func main() {
    r := gin.Default()
    r.GET("/", index)
    r.Run()
}
