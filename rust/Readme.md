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
# Build the doc
To build doc without dependencies doc, run `cargo doc --open --no-deps`. 
It will open doc on your browser. 

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

## Systemd with Unix socket

An example of configuration for nginx and systemd is available in 
[the deployement folder](deployment/systemd_unix).


## Systemd with Tcp socket

An example of configuration for nginx and systemd is available in 
[the deployement folder](deployment/systemd_tcp).
