import json
import stripe_service
from fastapi import FastAPI
from database import engine, Base

app = FastAPI()

app.include_router(stripe_service.router)

async def startup():
    await engine.begin().run_sync(Base.metadata.create_all)

@app.get('/')
async def main():
    return {'message': 'Microservice is running'}
          