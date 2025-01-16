import json
from . import stripe_service 
from fastapi import FastAPI
from .database import engine, Base

app = FastAPI()

app.include_router(stripe_service.router)

@app.on_event('startup')
async def startup():
    Base.metadata.create_all(bind=engine)

@app.get('/')
async def main():
    return {'message': 'Microservice is running'}
          