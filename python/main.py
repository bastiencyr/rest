from flask import Flask, request, jsonify, json

app = Flask(__name__)


# define our route with a decorator

@app.route("/", methods=["GET"])
def home():
    # return as json
    country = [{"id": 0, "area": 0, "name": "Hello", "capital": "World!"}]
    return jsonify(country)


@app.route("/countries", methods=["GET"])
def get_countries():
    with open("data.txt") as file:
        file_json = json.load(file)
    # return as json
    return jsonify(file_json)


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
