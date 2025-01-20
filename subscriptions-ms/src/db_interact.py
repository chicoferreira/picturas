from sqlalchemy.orm import Session
from .models import Subscription
from .database import get_db

async def new_sub(sub: Subscription, db: Session):
    db.add(sub)
    db.commit()
    return sub  