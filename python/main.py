from flask import Flask, request, jsonify, json

app = Flask(__name__)


# define our route with a decorator
@app.route("/countries", methods=["GET"])
def get_countries():
    f = json.load(open("data.txt"))
    # return as json
    return jsonify(f)


@app.route("/country/<int:id>", methods=["GET"])
def get_country(id):
    f = json.load(open("data.txt"))
    ret = f[id - 1] if id < len(f) else None
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
