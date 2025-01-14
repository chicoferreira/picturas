from sqlalchemy import create_engine
from sqlalchemy.engine.url import URL
from sqlalchemy.orm import sessionmaker

url_obj = URL.create(
    drivername='postgresql',
    username='postgres',
    password='postgres',
    host='localhost',
    port='5433',
    database='subscriptions'
)

engine = create_engine(url_obj)
Session = sessionmaker(bind=engine)

class Base:
    __name__: str
    
    def __tablename__(cls):
        return cls.__name__.lower()

def get_db():
    db = Session()
    try:
        yield db
    finally:
        db.close()