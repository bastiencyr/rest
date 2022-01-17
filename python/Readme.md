# Development 
We will use the integrated server with the following command:
```
export FLASK_ENV=development
export FLASK_APP=main.py
flask run 
```
At this point, you can fetch the root api at http://127.0.0.1:5000. You should see a single record:

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
"deployment/systemd_unix". Nginx will listen to 127.0.0.1:8000 and redirect connections to /run/rest.sock. You can change 
the address and the port in the nginx configuration. 
You must set WorkingDirectory, User and Group in the service systemd file to get the service working.

You have to create a specific user for this service before. For example, rest-api user. This user doesn't need any root 
access. For a testing purpose, you can use an existing user ($USER for example).