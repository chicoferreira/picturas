import stripe
import os
import httpx
import logging
import json
import jwt
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
WEBHOOK_SECRET = os.getenv('WEBHOOK_SECRET')
USERS_ENDPOINT = os.getenv('USERS_ENDPOINT')
PRICE_OBJECT = os.getenv('PRICE_OBJECT')
SUCCESS_URL = os.getenv('SUCCESS_URL')
CANCEL_URL = os.getenv('CANCEL_URL')

with open("/keys/access.key.pub", "r") as file:
    pub_key = file.read().strip()

# POST to the users endpoint with premium role
async def notif_users(sub: Subscription):
    user_id = sub.user_id
    end_date = sub.end_date.isoformat()
    if sub.status == 'active':
        role = 'premium'
        expires_on = end_date
    else:
        role = 'default'
        expires_on = None
    data = {
        'user_id': user_id,
        'role': role,
        'expires_on': end_date
    }
    logging.info(f'Notifying users service with data: {data}')
    
    try:
        async with httpx.AsyncClient() as client:
            response = await client.post(USERS_ENDPOINT, json=data) # POST to the users endpoint
            response.raise_for_status()
            logging.info(f'Users service response: {response.json()}')

    except httpx.HTTPStatusError as e:
        logging.error(f'HTTP error notifying users service: {e}')

    except Exception as e:
        logging.error(f'Error notifying users service: {e}')

# Create a stripe checkout session
@router.post('/create-checkout-session')
async def create_checkout_session(req: Request, db: Session = Depends(get_db)):
    try:
        # Get user_id from access token
        data = await req.cookies()
        access_token = data['access_token']

        if not access_token:
            return JSONResponse(
                status_code=400,
                content={'error': 'Unauthorized'}
            )
        
        access_token_decoded = jwt.decode(access_token, pub_key, algorithms='RS256')

        now = datetime.utcnow()
        if now > datetime.fromtimestamp(access_token_decoded['exp']):
            return JSONResponse(
                status_code=400,
                content={'error': 'Unauthorized'}
            )

        user_id = access_token_decoded['sub']

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
                'price': PRICE_OBJECT,
                'quantity': 1,
            }],
            success_url= SUCCESS_URL,
            cancel_url= CANCEL_URL,
        )

        sub_exists = db.query(Subscription).filter_by(user_id = user_id).first()

        # Create a new subscription record and insert it into the db, if it didn't exist yet
        if not sub_exists:
            sub = Subscription(
                session_id = session.id,
                user_id = user_id,
                price = 9.99,
                stripe_subscription_id = session.subscription,
                start_date = datetime.utcnow(),
                end_date = datetime.utcnow() + timedelta(days=30),
                status = 'inactive'
            )

            new_subscription = await new_sub(sub, db)
        
        # If it exists, update the record with the new session_id and new dates
        else:
            sub_exists.session_id = session.id
            sub_exists.start_date = datetime.utcnow()
            sub_exists.end_date = datetime.utcnow() + timedelta(days=30)
            db.commit()

        return JSONResponse(
            status_code=200,
            content={'url': session['url']}
        )

    except Exception as e:
        return JSONResponse(
            status_code=500,
            content={'error': str(e)}
        )

# Handle stripe webhook events
@router.post('/webhook')
async def handle_webhook(req: Request, res: Response, db: Session = Depends(get_db)):
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
            raw_payload, sig_header, WEBHOOK_SECRET
        )
    
        match event['type']:
            case 'invoice.payment_succeeded':
                invoice = event['data']['object']
                stripe_sub_id = invoice['subscription']
                sub = db.query(Subscription).filter_by(stripe_subscription_id = stripe_sub_id).first()
                if sub:
                    if invoice['billing_reason'] == 'subscription_create':
                        sub.status = 'active'
                        db.commit()

                    elif invoice['billing_reason'] == 'subscription_update':
                        sub.end_date = datetime.fromtimestamp(invoice['period']['end'])
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
    
@router.get('/subscription-details')
async def get_subscription_details(req: Request, db: Session = Depends(get_db)):
    try:
        data = await req.cookies()
        access_token = data['access_token']

        if not access_token:
            return JSONResponse(
                status_code=400,
                content={'error': 'Unauthorized'}
            )
        
        access_token_decoded = jwt.decode(access_token, pub_key, algorithms='RS256')

        now = datetime.utcnow()
        if now > datetime.fromtimestamp(access_token_decoded['exp']):
            return JSONResponse(
                status_code=400,
                content={'error': 'Unauthorized'}
            )

        user_id = access_token_decoded['sub']

        if not user_id:
            return JSONResponse(
                status_code=400,
                content={'error': 'Missing required fields'}
            )

        sub = db.query(Subscription).filter_by(user_id = user_id).first()

        if not sub:
            return JSONResponse(
                status_code=404,
                content={'error': 'Subscription not found'}
            )

        return JSONResponse(
            status_code=200,
            content={
                'sub_id': sub.id,
                'sub_price': sub.price,
                'start_date': sub.start_date,
                'end_date': sub.end_date,
                'status': sub.status.
                'user_role': 'premium' if sub.status == 'active' else 'default'
            }
        )

    except Exception as e:
        return JSONResponse(
            status_code=500,
            content={'error': str(e)}
        )

