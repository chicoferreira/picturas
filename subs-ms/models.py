from database import Base
from sqlalchemy import Column, Integer, String, Boolean, DateTime
import datetime

class Subscription(Base):
    __tablename__ = 'subscriptions'
    id = Column(Integer, primary_key=True)
    user_id = Column(Integer, nullable=False)
    price = Column(Integer, nullable=False)
    stripe_subscription_id = Column(String, nullable=False)
    start_date = Column(DateTime, nullable=False)
    end_date = Column(DateTime, nullable=False)
    status = Column(String(20), nullable=False, default='inactive')