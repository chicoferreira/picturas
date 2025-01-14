import stripe
import os
from uuid import UUID

from models import Subscription
from fastapi import APIRouter, Request, Response, Depends
from fastapi.responses import JSONResponse
from database import Session, get_db
from db_interact import new_sub
from dotenv import load_dotenv

router = APIRouter()

load_dotenv()
stripe.api_key = os.getenv('STRIPE_SECRET_KEY')

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
                'price': 'price_1Qfi9BIu8PvkBpDWSZ3jvFBf',
                'quantity': 1,
            }],
            success_url='https://example.com/success',
            cancel_url='https://example.com/cancel',
        )

        sub = Subscription(
            user_id = user_id,
            price = 10,
            stripe_subscription_id = session.subscription,
            start_date = datetime.utcnow(),
            end_date = datetime.utcnow() + timedelta(days=30),
            status = 'inactive'
        )

        new_sub = new_sub(sub, db)

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
    payload = await req.body()
    sig_header = req.headers.get('Stripe-Signature')

    try:
        event = stripe.Webhook.construct_event(
            payload, sig_header, os.getenv('WEBHOOK_SECRET')
        )

        if event['type'] == 'checkout.session.completed':
            session = event['data']['object']
            stripe_sub_id = session['subscription']
            sub = db.query(Subscription).filter_by(stripe_subscription_id = stripe_sub_id).first()
            if sub:
                sub.status = 'active'
                db.commit()
        
        return 200

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



