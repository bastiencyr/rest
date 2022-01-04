# Installation
We suppose that cargo is already installed on the machine.
Then run the command (long the first time):
```bash
cargo run
```

# Development
For the development, we want the application to load at each change:
```bash
cargo install cargo-watch
cargo watch -x 'run'
```

# Requests
``` bash
# post
curl  -d '{"name": "France", "capital":"Paris", "area": 10}' -H 'Content-Type: application/json'  http://127.0.0.1:5000/country
```

To get all countries, you can consult the address http://127.0.0.1:5001/countries in a browser.
To get a single country: http://127.0.0.1:5001/country/1

# Deployment
Use systemd with Nginx. No need to take a specific web server.