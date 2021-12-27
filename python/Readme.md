# Déveloeppment
On utilisera le serveur web intégré avec la commandes uivante : 
```
export FLASK_ENV=development
export FLASK_APP=main.py
flask run 
```

L'API est alors consultable à l'adresse http://127.0.0.1:5000/countries pour un simple GET. 

On peut également consulter un pays en particulier :  http://127.0.0.1:5000/country/1

Et ajouter un pays (bash) : 
``` 
curl -i http://127.0.0.1:5000/countries \
-X POST \
-H 'Content-Type: application/json' \
-d '{"name":"Germany", "capital": "Berlin", "area": 357022}'
```

# Déploiement 
Pour le déploiement, on utilisera un vraie serveur web tel que gunicorn. Il est alors conseillé de le 
faire fonctionner avec Nginx.
```
gunicorn -w 4 -b 127.0.0.1:4000 main:app
```