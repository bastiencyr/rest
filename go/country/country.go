package country

import (
	"encoding/json"
	"fmt"
	"github.com/gofiber/fiber/v2"
	"io/ioutil"
	"strconv"
)

// Article ...
type Country struct {
	Id      int    `json:"id"`
	Name    string `json:"name"`
	Capital string `json:"capital"`
	Area    int    `json:"area"`
}

func DefaultCountry() *Country {
	return &Country{
		Id:      -1,
		Name:    "Not found",
		Capital: "Not found",
		Area:    -1,
	}
}

func GetCountries(c *fiber.Ctx) error {
	// read file
	data, err := ioutil.ReadFile("./data.txt")
	if err != nil {
		fmt.Print(err)
	}

	// json data
	var Countries2 []Country

	// unmarshall it
	err = json.Unmarshal(data, &Countries2)
	if err != nil {
		fmt.Println("error:", err)
	}

	return c.JSON(Countries2)
}

func GetCountry(c *fiber.Ctx) error {
	// read file
	data, err := ioutil.ReadFile("./data.txt")
	if err != nil {
		fmt.Print(err)
	}

	// json data
	var Countries2 []Country

	// unmarshall it
	err = json.Unmarshal(data, &Countries2)
	if err != nil {
		fmt.Println("error:", err)
	}

	i, err := strconv.Atoi(c.Params("id"))

	if i < len(Countries2) && err == nil {
		return c.JSON(Countries2[i])
	}

	return c.JSON(DefaultCountry())
}
