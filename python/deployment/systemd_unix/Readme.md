# Install python dependencies

```bash
sudo apt install python3-flask
sudo apt install gunicorn 
```
Notice that your flask app must be compatible with your code. There can be a gap between 
the version of the OS and your local version of development. In any case, you should 
always respect the version of the OS.

# Configure systemd 

Put rest-py.service and rest-py.socket in /etc/systemd/system.

Now, you have to create a new user:
```bash
sudo adduser --system rest-api --group --no-create-home
```

So, the service will run as rest-api user who has no root privileges. It's better for 
security reasons because you can control the permissions of your services. 

Don't forget to set "WorkingDirectory" as an absolute path in rest-py.service. 

Then: 
```bash
# Enable the services
sudo systemctl enable rest-py.socket
sudo syemctl enable rest-py.service

# start the services
sudo systemctl start rest-py.socket
sudo systemctl start rest-py.service

# Check status
sudo systemctl status rest-py.socket
sudo systemctl status rest-py.service
```


# Install Nginx

`sudo apt install nginx`

Put nginx.conf in /etc/nginx/nginx.conf (Backup and remove the default one).
Put rest-py-server.conf in /etc/nginx/sites-available

Make a symbolic link and start nginx: 
```bash
sudo ln -s /etc/nginx/sites-available/rest-py-server.conf /etc/nginx/sites-enabled/rest-py-server.conf
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