# Installation
On suppose que cargo est déjà installé sur la machine.
Lancer alors la commande (long la première fois):
```bash
cargo run
```

# Développement
Pour le développement, on veut que l'appli se charge à chaque changement:
```bash
cargo install cargo-watch
cargo watch -x 'run'
```

# Requêtes
``` bash
# post
curl  -d '{"name": "France", "capital":"Paris", "area": 10}' -H 'Content-Type: application/json'  http://127.0.0.1:5000/country
```

Pour le get, on peut consulter l'adresse http://127.0.0.1:5000/countries dans un navigateur.
Pour obtenir un seul pays: http://127.0.0.1:5000/country/1

# Déploiement