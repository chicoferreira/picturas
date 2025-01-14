from uuid import UUID
from datetime import datetime
from pydantic import BaseModel

class ReqSubscription(BaseModel):
    user_id: UUID    

class ResSubscription(BaseModel):
    id: UUID
    user_id: UUID
    plan: str
    stripe_subscription_id: str
    start_date: datetime
    end_date: datetime
    status: str

