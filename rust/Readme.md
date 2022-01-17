# Installation

We assume that cargo is already installed on the machine. Then run the command (long the first time):

```bash
export RUST_LOG="debug" # Turn on debug info
cargo run
```

The server will run at http://127.0.0.1:5000. You should see a single record:

``` 
{
    "id":	"0",
    "name":	"Hello",
    "capital":	"World!",
    "area":	"0",
}
```

You can change the listening port with:
```
cargo run -- --addr 127.0.0.1:5003
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

To get all countries, you can consult the address http://127.0.0.1:5000/countries in a browser. To get a single
country: http://127.0.0.1:5000/country/1

# Deployment

## Unix socket
Use for example systemd with Nginx. An example of configuration for nginx and systemd is available in 
"deployment/systemd_unix". Nginx listens to 127.0.0.1:8000 and redirects connections to /run/rest.sock. You can change 
the address and the port in the nginx configuration.
You have to create a specific user for this service before. For example, rest-api user. This user doesn't need any root 
access. For a testing purpose, you can use an existing user ($USER for example).

## Tcp socket
Use for example systemd with Nginx. An example of configuration for nginx and systemd is available in 
"deployment/systemd_tcp". The rust server listens to 127.0.0.1:5000. 
Nginx listens to 127.0.0.1:8000 and redirects connections to the internal address 127.0.0.1:5000. You can change 
the address and the port of the systemd service and nginx in the configuration.
You have to create a specific user for this service before. For example, rest-api user. This user doesn't need any root 
access. For a testing purpose, you can use an existing user ($USER for example).