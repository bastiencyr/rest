# Installation

We assume that cargo is already installed on the machine. Then run the command (long the first time):

```bash
export RUST_LOG="debug" # Turn on debug info
cargo run
```

# Development

For the development, we want the application to load at each change:

```bash
export RUST_LOG="debug" # Turn on debug info
cargo install cargo-watch
cargo watch -x 'run'
```

# Requests

``` bash
# post
curl  -d '{"name": "France", "capital":"Paris", "area": 10}' -H 'Content-Type: application/json'  http://127.0.0.1:5000/country
```

To get all countries, you can consult the address http://127.0.0.1:5001/countries in a browser. To get a single
country: http://127.0.0.1:5001/country/1

# Deployment

Use for example systemd with Nginx. An example of configuration is available in "deployment/systemd". Your server will
run on 127.0.0.1:8000 and the service will listen on /run/rest.sock socket. Nginx redirects connections 
to /run/rest.sock. 
You have to create a specific user for this service before. For example, rest-api user. This user doesn't need any root 
access. 
