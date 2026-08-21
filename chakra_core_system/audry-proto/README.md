markdown
# Audry UI

## Setup
```bash
npm install
npm start
Runs the React + D3 interface at http://localhost:3000.

Code

---

# 📂 Root `README.md`
```markdown
# audry-proto

Prototype system for model slicing, shard serving, and UI visualization.

## Quickstart
1. Build and run the shard server:
   ```bash
   docker build -t shard-server ./docker
   docker run -p 5000:5000 shard-server
Start the UI:

bash
cd ui
npm install
npm start
Visit http://localhost:3000 to interact with shards.

Code

---

✨ With this in place, you’ll have a **working prototype**:  
- Flask server serves shard metadata + audio files  
- React UI fetches and plays shards, visualizes them with D3  
- Dockerfile packages the server