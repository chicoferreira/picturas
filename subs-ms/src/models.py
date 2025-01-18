from .database import Base
from sqlalchemy import Column, Integer, String, Boolean, DateTime
import datetime
from uuid import UUID

class Subscription(Base):
    __tablename__ = 'subscriptions'
    id = Column(Integer, primary_key=True)
    user_id = Column(UUID, nullable=False)
    price = Column(String, nullable=False)
    start_date = Column(DateTime, nullable=False)
    end_date = Column(DateTime, nullable=False)
    status = Column(String(20), nullable=False, default='inactive')
    session_id = Column(String, nullable=False)
    stripe_subscription_id = Column(String)
