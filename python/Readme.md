# Development 
We will use the integrated server with the following command:
```
export FLASK_ENV=development
export FLASK_APP=main.py
flask run 
```
The API will be available at http://127.0.0.1:5000/countries to fetch all countries.
We can also fetch only one country with :  http://127.0.0.1:5000/country/1

Or add a country in bash:
``` 
curl -i http://127.0.0.1:5000/countries \
-X POST \
-H 'Content-Type: application/json' \
-d '{"name":"Germany", "capital": "Berlin", "area": 357022}'
```

# Deployment 
For the deployment, we will use a real web server such as gunicorn. It is then advised to 
to run it with Nginx.
```
gunicorn -w 4 -b 127.0.0.1:4000 main:app
```