package country

import (
	"encoding/json"
	"log"
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

func errNotModified(err error, c *fiber.Ctx) error {
	if err != nil {
		log.Printf("Ressource Not Modfied: %s", err)
	}
	return c.SendStatus(304)
}


func GetCountries(c *fiber.Ctx) error {
	// read file
	data, _ := ioutil.ReadFile("./data.txt")
	// json data
	var Countries2 []Country
	// unmarshall it
	json.Unmarshal(data, &Countries2)

	return c.JSON(Countries2)
}

func GetCountry(c *fiber.Ctx) error {
	// read file
	data, _ := ioutil.ReadFile("./dta.txt")
	// json data
	var Countries2 []Country
	// unmarshall it
	json.Unmarshal(data, &Countries2)

	i, err := strconv.Atoi(c.Params("id"))

	if i < len(Countries2) && err == nil {
		return c.JSON(Countries2[i])
	}

	return c.JSON(DefaultCountry())
}

func PostCountry(c *fiber.Ctx) error {
	p := new(Country)
        c.BodyParser(p)
	
	//read the file and add the new country
	data, err := ioutil.ReadFile("./data.txt")
	return errNotModified(err, c)

	// json data
	var Countries2 []Country

	// unmarshall it
	err = json.Unmarshal(data, &Countries2)
	return errNotModified(err, c)
	p.Id = len(Countries2)
	
	Countries2 = append(Countries2, *p)
	
        // marshall it
	data, err = json.Marshal(Countries2)
	return errNotModified(err, c)

	ioutil.WriteFile("./data.txt", data, 0644) 

	// Data has been posted
	return c.SendStatus(201)
}
