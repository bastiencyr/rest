# Deployment

In this Readme, you will learn how to deploy a simple rest API coded with actix on
Nginx proxy with systemd integration.

Nginx will listen to 127.0.0.1:8000 and redirect connections to internal address 127.0.0.1:5000. You can change
the external address and the port in the nginx configuration and internal address in rest-rs.socket.

## Compiling
First, you need to compile this project. You need to install cargo and then run `cargo build --release`.
Cargo will generate target/release/rest executable files.

## Configure systemd

Put rest-rs.service and rest-rs.socket in /etc/systemd/system.

Now, you have to create a new user:
```bash
sudo adduser --system rest-api --group --no-create-home
```

So, the service will run as rest-api user who has no root privileges. It's better for
security reasons because you can control the permissions of your services.

Don't forget to set "WorkingDirectory" and "ExecStart" as an absolute path in rest-py.service.
"ExecStart" must be the path to your executable file.

Then:
```bash
# Enable the services
sudo systemctl enable rest-rs.socket
sudo systemctl enable rest-rs.service

# start the services
sudo systemctl start rest-rs.socket
sudo systemctl start rest-rs.service

# Check status
sudo systemctl status rest-rs.socket
sudo systemctl status rest-rs.service
```

## Install Nginx

`sudo apt install nginx`

Put nginx.conf in /etc/nginx/nginx.conf (Backup and remove the default one).
Put rest-rs-server.conf in /etc/nginx/sites-available

Make a symbolic link and start nginx:
```bash
sudo ln -s /etc/nginx/sites-available/rest-rs-server.conf /etc/nginx/sites-enabled/rest-rs-server.conf
sudo systemctl restart nginx

# Check status
sudo systemctl status nginx
```

If you have some troubles:
```bash
# Check the status of nginx
sudo systemctl status nginx

# Check the syntax of your configuration files
sudo nginx -t

# Check the log of nginx
sudo tail /var/log/nginx/error.log
```

Now you should see the server running at http://localhost:8000.

## Permissions
The data are stored in a file called "data.txt" in the WorkingDirectory. If the post curl request doesn't work, you should
probably change the permission of this file. The service is run as rest-api user.

```bash
sudo chown rest-api:rest-api data.txt
```

# Request/API endpoints

To fetch all countries, you can consult the address http://127.0.0.1:8000/countries in a browser. We can also fetch
only one country at  http://127.0.0.1:8000/country/1

Notice that you also can fetch countries on local address at http://127.0.0.1:5000/countries.


Or add a country in bash:
``` bash
# post
curl
-d '{"name": "France", "capital":"Paris", "area": 10}'
-H 'Content-Type: application/json'  http://127.0.0.1:8000/country
```