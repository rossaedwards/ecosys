**audry-proto: Two-Shard Fractal-Lattice Voice Demo**

Kick off your prototype repository for Audry’s distributed “shard” inference. This repo delivers a minimal two-shard split of a tiny TTS model, publishes them to IPFS + Tor, serves via a Flask shard-server, and visualizes the fractal graph in a D3.js web UI.

**📁 Repository Structure**

Code

audry-proto/

├── docker/

│ ├── Dockerfile \# Builds the shard-server container

│ └── shard-server-entry.sh \# Entrypoint script

│

├── model-slice/

│ ├── slice_model.py \# Split a PyTorch model into two shards

│ └── requirements.txt \# PyTorch + utilities

│

├── shard-server/

│ ├── shard_server.py \# Flask microservice (serves FLAC shards)

│ └── index.json \# PoC node metadata for two shards

│

├── ui/

│ ├── public/

│ │ └── index.html \# HTML shell

│ ├── src/

│ │ ├── App.js \# Fetch shards + play audio

│ │ └── Graph.js \# D3 fractal-lattice visualizer

│ ├── package.json \# React + D3 dependencies

│ └── README.md \# UI setup & run instructions

│

└── README.md \# This overview & quickstart

**⚙️ Prerequisites**

- Git

- Docker (or Podman)

- Python 3.9+

- Node.js 16+ & npm

- IPFS daemon (go-ipfs)

- Tor daemon (for onion service)

**🚀 Quickstart**

1.  Clone & enter the repo

bash

git clone https://github.com/yourorg/audry-proto.git

cd audry-proto

2.  Launch IPFS & Tor (in separate terminals)

bash

ipfs daemon

tor

3.  Build shards

bash

cd model-slice

pip install -r requirements.txt

python slice_model.py \\

--input-model path/to/tiny-tts.pt \\

--shard-count 2 \\

--output-dir ../shard-server/shards

This produces:

Code

shard-server/shards/

shard_0.pt

shard_1.pt

And updates shard-server/index.json with paths & neighbor links.

4.  Publish shards to IPFS

bash

cd shard-server/shards

ipfs add -r .

Note the returned CIDs and paste them into index.json under each node’s cid field.

5.  Build & run shard-server container

bash

cd ../..

docker build -t audry/shard-server ./docker

docker run -d --name shard-server \\

-p 8080:8080 \\

audry/shard-server

6.  Run the Web UI

bash

cd ui

npm install

npm start

Open http://localhost:3000 to see the fractal-lattice graph. Click a node to fetch its FLAC via the shard-server and play it.

**📦 Components Deep Dive**

**1. model-slice/slice_model.py**

Splits a small PyTorch model into N shards by chunking its state_dict.

- Flags: --input-model, --shard-count, --output-dir

**2. shard-server/shard_server.py**

A Flask app that:

- Loads index.json (nodes with path, neighbors, and cid for IPFS).

- GET /shard/\<id\> returns FLAC bytes (local or via IPFS gateway).

- GET /neighbors/\<id\> returns JSON array of neighbor node IDs.

**3. ui/src/Graph.js**

Renders the fractal-lattice using D3:

- Nodes are phrases/shard IDs.

- Links follow neighbors relations.

- Click handler fetches /shard/{id} and streams audio via HTML5.

**🔮 Next Steps**

- Extend slice_model.py to support real TTS weights (e.g., distilled LLaMA-7B audio model).

- Harden shard-server: integrate onion routing via Tor’s SOCKS proxy and IPFS HTTP API.

- Enhance UI:

  - Pre-fetch neighbor shards for sub-10 ms loads.

  - Display real-time latency & QPS metrics.

- Containerize the full stack with Kubernetes YAML for node-pooling between lightweight & heavy shards.
