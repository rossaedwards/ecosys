"""
Aurphyx_Memoree — Main Daemon Entry Point
Run: python daemon/memoree_service.py
f0rg3d in l0v3 by Ross Edwards
"""

import sys
import os
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

import asyncio
import logging
import uvicorn
from fastapi import FastAPI
from api.routes import router
from daemon.heartbeat import HeartbeatLoop

logging.basicConfig(
    level=logging.INFO,
    format="%(asctime)s [%(levelname)s] %(name)s: %(message)s",
    handlers=[
        logging.FileHandler("daemon/memoree_service.log"),
        logging.StreamHandler(),
    ]
)
log = logging.getLogger("memoree")

app = FastAPI(
    title="Aurphyx_Memoree Daemon",
    description="Sovereign Memory, Identity & Continuity Substrate — f0rg3d in l0v3",
    version="0.1.0",
)
app.include_router(router, prefix="")


@app.on_event("startup")
async def startup():
    log.info("❤️‍🔥 Aurphyx_Memoree Daemon starting on 127.0.0.1:7042")
    loop = asyncio.get_event_loop()
    hb = HeartbeatLoop()
    loop.create_task(hb.run())


if __name__ == "__main__":
    uvicorn.run(
        "daemon.memoree_service:app",
        host="127.0.0.1",
        port=7042,
        reload=False,
        log_level="info",
    )
