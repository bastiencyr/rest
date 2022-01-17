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

## Compilation
You need first to compile the project with `cargo build --release`. The executable will be available in 
"/path_to_the_project/rest/rust/target/release/rust".

## Unix socket
Use for example systemd with Nginx. An example of configuration for nginx and systemd is available in 
"deployment/systemd_unix". You must set WorkingDirectory, User and Group in the service systemd file 
to get the service working.

Nginx will listen to 127.0.0.1:8000 and redirect connections to /run/rest.sock. You can change 
the address and the port in the nginx configuration.
You have to create a specific user for this service before. For example, rest-api user. This user doesn't need any root 
access. For a testing purpose, you can use an existing user ($USER for example).

## Tcp socket
Use for example systemd with Nginx. An example of configuration for nginx and systemd is available in 
"deployment/systemd_tcp". You must set WorkingDirectory, User and Group in the service systemd file 
to get the service working.

The rust server will listen to 127.0.0.1:5000. 
Nginx will listen to 127.0.0.1:8000 and redirect connections to the internal address 127.0.0.1:5000. You can change 
the address and the port of the systemd service and nginx in the configuration.
You have to create a specific user for this service before. For example, rest-api user. This user doesn't need any root 
access. For a testing purpose, you can use an existing user ($USER for example).