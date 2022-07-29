package main

import "github.com/gin-gonic/gin"

func index(c *gin.Context) {
    c.String(200, "Hello World")
}


func main() {
    r := gin.Default()
    r.GET("/", index)
    r.Run()
}
