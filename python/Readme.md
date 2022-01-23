# Development 
We will use the integrated server with the following command:
```
export FLASK_ENV=development
export FLASK_APP=main.py
flask run 
```
At this point, you can fetch the root api at http://127.0.0.1:5000. Flask run the server on port 5000 by default.
You should see a single record:

``` 
{
    "id":	"0",
    "name":	"Hello",
    "capital":	"World!",
    "area":	"0",
}
```

# Request/API endpoints

To fetch all countries, you can consult the address http://127.0.0.1:5000/countries in a browser. We can also fetch 
only one country at  http://127.0.0.1:5000/country/1


Or add a country in bash:
``` bash
# post
curl  
-d '{"name": "France", "capital":"Paris", "area": 10}' 
-H 'Content-Type: application/json'  http://127.0.0.1:5000/country
```

# Deployment 
For the deployment, we will use a real web server such as gunicorn. It is then advised
to run it with Nginx.
```
gunicorn -w 4 -b 127.0.0.1:4000 main:app
```

## Systemd integration, unix socket
Use for example systemd with Nginx. An example of configuration for nginx and systemd is available in 
"deployment/systemd_unix". You have a Readme to install this project with nginx and systemd.
