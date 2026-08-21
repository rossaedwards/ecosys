import os
from flask import Flask, jsonify, send_file
import json

app = Flask(__name__)

with open("index.json") as f:
    SHARDS = json.load(f)

@app.route("/shards", methods=["GET"])
def list_shards():
    return jsonify(SHARDS)

@app.route("/shards/<shard_id>", methods=["GET"])
def get_shard(shard_id):
    shard = SHARDS.get(shard_id)
    if not shard:
        return jsonify({"error": "Shard not found"}), 404
    path = shard["path"]
    if not os.path.exists(path):
        return jsonify({"error": "File missing"}), 500
    return send_file(path, as_attachment=True)

if __name__ == "__main__":
    app.run(host="0.0.0.0", port=5000)