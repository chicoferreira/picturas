from sqlalchemy import create_engine
from sqlalchemy.engine.url import URL
from sqlalchemy.orm import sessionmaker
from sqlalchemy.ext.declarative import declarative_base
from dotenv import load_dotenv

load_dotenv()

url_obj = URL.create(
    drivername='postgresql',
    username='${PG_USER}',
    password='${PG_PASSWORD}',
    host='${PG_HOST}',
    port='${PG_PORT}',
    database='${PG_DATABASE}'
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