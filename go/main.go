package main

import (
	"github.com/gofiber/fiber/v2"
	"rest_go/country"
)

func setupRoutes(app *fiber.App) {
	app.Post("/country", country.PostCountry)
	app.Get("/countries", country.GetCountries)
	app.Get("/country/:id", country.GetCountry)
}

func main() {

	app := fiber.New(fiber.Config{
		Prefork:       true,
		StrictRouting: true,
		ServerHeader:  "Fiber",
	})
	setupRoutes(app)
	app.Listen(":5000")
}
