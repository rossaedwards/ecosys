from fastapi import FastAPI
from app.hardware_api.interface import get_orb_status

app = FastAPI(title="Datacore Orb Simulator", version="0.1.0")

@app.get("/")
async def root():
    return {"message": "Datacore Orb Simulator Running"}

@app.get("/orb/status")
async def orb_status():
    status = get_orb_status()
    return {"orb_status": status}
