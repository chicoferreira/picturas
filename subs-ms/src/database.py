from sqlalchemy import create_engine
from sqlalchemy.engine.url import URL
from sqlalchemy.orm import sessionmaker
from sqlalchemy.ext.declarative import declarative_base

url_obj = URL.create(
    drivername='postgresql',
    username='postgres',
    password='postgres',
    host='db',
    port='5432',
    database='subscriptions'
)

engine = create_engine(url_obj)
Session = sessionmaker(bind=engine)
Base = declarative_base()

def get_db():
    db = Session()
    try:
        yield db
    finally:
        db.close()