import stripe
import os
import httpx
import logging
import json
from datetime import datetime, timedelta
from uuid import UUID

from .models import Subscription
from fastapi import APIRouter, Request, Response, Depends
from fastapi.responses import JSONResponse
from .database import Session, get_db
from .db_interact import new_sub
from dotenv import load_dotenv

router = APIRouter()

load_dotenv()
stripe.api_key = os.getenv('STRIPE_SECRET_KEY')
USERS_ENDPOINT = os.getenv('USERS_ENDPOINT')

async def notif_users(sub: Subscription):
    user_id = sub.user_id
    end_date = sub.end_date.isoformat()
    data = {
        'user_id': user_id,
        'role': 'premium',
        'expires_on': end_date
    }
    logging.info(f'Notifying users service with data: {data}')
    
    try:
        async with httpx.AsyncClient() as client:
            response = await client.post(USERS_ENDPOINT, json=data)
            response.raise_for_status()
            logging.info(f'Users service response: {response.json()}')

    except httpx.HTTPStatusError as e:
        logging.error(f'HTTP error notifying users service: {e}')

    except Exception as e:
        logging.error(f'Error notifying users service: {e}')

@router.post('/users')
async def check(req: Request):
    data = await req.json()
    print(data)

@router.post('/create-checkout-session')
async def create_checkout_session(req: Request, db: Session = Depends(get_db)):
    try:
        data = await req.json()
        user_id = data['user_id']

        if not user_id:
            return JSONResponse(
                status_code=400,
                content={'error': 'Missing required fields'}
            )

        # Call Stripe API to create a checkout session
        session = stripe.checkout.Session.create(
            payment_method_types=['card'],
            mode='subscription',
            line_items=[{
                'price': 'price_1QhdkQIu8PvkBpDWPolasFE4',
                'quantity': 1,
            }],
            success_url='https://example.com/success',
            cancel_url='https://example.com/cancel',
        )

        sub = Subscription(
            session_id = session.id,
            user_id = user_id,
            price = 10,
            stripe_subscription_id = session.subscription,
            start_date = datetime.utcnow(),
            end_date = datetime.utcnow() + timedelta(days=30),
            status = 'inactive'
        )

        new_subscription = await new_sub(sub, db)

        return JSONResponse(
            status_code=200,
            content={'url': session['url']}
        )

    except Exception as e:
        return JSONResponse(
            status_code=500,
            content={'error': str(e)}
        )

@router.post('/webhook')
async def handle_webhook(req: Request, db: Session = Depends(get_db)):
    raw_payload = await req.body()
    sig_header = req.headers.get('Stripe-Signature')

    payload = json.loads(raw_payload.decode('utf-8'))
    s_id = payload['data']['object']['id']

    record = db.query(Subscription).filter_by(session_id = s_id).first()
    retrieved_session = stripe.checkout.Session.retrieve(s_id)
    record.stripe_subscription_id = retrieved_session.subscription
    db.commit()

    try:
        event = stripe.Webhook.construct_event(
            raw_payload, sig_header, os.getenv('WEBHOOK_SECRET')
        )
    
        if event['type'] == 'checkout.session.completed':
            session = event['data']['object']
            stripe_sub_id = session['subscription']
            sub = db.query(Subscription).filter_by(stripe_subscription_id = stripe_sub_id).first()
            if sub:
                sub.status = 'active'
                db.commit()
                await notif_users(sub)
        
        return JSONResponse(
            status_code=200,
            content={'status': 'success'}
        )
    
    except stripe.error.SignatureVerificationError:
        return JSONResponse(
            status_code=400,
            content={'error': 'Invalid Signature'}
        )
    
    except Exception as e:
        return JSONResponse(
            status_code=500,
            content={'error': str(e)}
        )


@router.get('/subscriptions')
async def get_subscriptions(db: Session = Depends(get_db)):
    subs = db.query(Subscription).all()
    return subs


