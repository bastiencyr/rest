from flask import Flask, request, jsonify, json, make_response

app = Flask(__name__)


# define our route with a decorator

@app.route("/", methods=["GET"])
def home():
    # return as json
    country = [{"id": 0, "area": 0, "name": "Hello", "capital": "World!"}]
    resp = make_response(jsonify(country))
    # Get the headers from the proxy server
    # This headers are controlled by the proxy_set_header directive in Nginx
    # cache_control = request.headers.get("Cache-Control", default="NoneValue")
    # The root will not be cached on the proxy unless proxy_cache_valid directive is set.
    return resp


@app.route("/countries", methods=["GET"])
def get_countries():
    with open("data.txt") as file:
        file_json = json.load(file)
    # return as json
    resp = make_response(jsonify(file_json))
    # set Cache-Control policy. We inform the proxy that this request can be stored in a public area for
    # up to 50 secondes. The proxy is then forced to respect the Cache-Control policy but
    # this response can go through several proxy's. So the first proxy can edit the outgoing request headers.
    resp.headers['Cache-Control'] = 'public, max-age=50'
    return resp


@app.route("/country/<int:id>", methods=["GET"])
def get_country(id):
    with open("data.txt") as file:
        file_json = json.load(file)
        ret = file_json[id - 1] if id < len(file_json) else None
    return jsonify(ret)


@app.route("/country", methods=["POST"])
def add_country():
    with open("data.txt", mode="r") as file:
        data = json.load(file)

    data_file_w = open("data.txt", mode="w")
    if request.is_json:
        country = request.get_json()
        country["id"] = len(data)
        data.append(country)
        data_file_w.write(json.dumps(data))  # convert to string
        data_file_w.close()
        return country, 201
    return {"error": "Request must be JSON"}, 415
