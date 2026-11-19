from carbotrack import CarboTrack
from fastapi import FastAPI

tracker = CarboTrack(cloud="aws", region="us-east-1", budget=1000)
app = FastAPI()

@app.middleware("http")
async def carbon_middleware(request, call_next):
    response = await call_next(request)
    tracker.track({"type": "compute", "operation": request.url.path, "duration_ms": 10, "cores": 1})
    return response

@app.get("/")
def root():
    return {"status": "ok", "emissions": tracker.get_emissions()}
