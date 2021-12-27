# app.py
from markupsafe import escape

from flask import Flask, request, jsonify

app = Flask(__name__)

# on définit en dur nos données
countries = [
    {"id": 1, "name": "Thailand", "capital": "Bangkok", "area": 513120},
    {"id": 2, "name": "Australia", "capital": "Canberra", "area": 7617930},
    {"id": 3, "name": "Egypt", "capital": "Cairo", "area": 1010408},
]


def _find_next_id():
    return max(country["id"] for country in countries) + 1


@app.route("/countries", methods=["GET"])
def get_countries():
    return jsonify(countries)


@app.route("/country/<int:id>", methods=["GET"])
def get_country(id):
    ret = countries[id-1] if id < len(countries) else None
    return jsonify(ret)


@app.route("/countries", methods=["POST"])
def add_country():
    if request.is_json:
        country = request.get_json()
        country["id"] = _find_next_id()
        countries.append(country)
        return country, 201
    return {"error": "Request must be JSON"}, 415
