from datetime import datetime, timedelta, timezone
import os
import jwt
import bcrypt 
from jwt_keys import get_private_key, get_public_key
import uuid

TOKEN_EXPIRATION_MINUTES: int = os.getenv("ACCOUNTS_JWT_TOKEN_EXPIRATION_MINUTES", 15)
REFRESH_TOKEN_EXPIRE_MINUTES: int =  os.getenv("ACCOUNTS_JWT_REFRESH_TOKEN_EXPIRATION_MINUTES", 60*7)
JWT_ALGORITHM: str = "RS256"


def hash_password(password: str) -> str:
    return bcrypt.hashpw(password.encode("utf-8"), bcrypt.gensalt()).decode("utf-8")


def verify_password(plain_password: str, hashed_password: str) -> bool:
    return bcrypt.checkpw(
        plain_password.encode("utf-8"),
        hashed_password.encode("utf-8"),
    )


def create_jwt_token(user_id: int, username: str) -> str:
    private_key = get_private_key()
    now = datetime.now(timezone.utc)
    expiration = now + timedelta(minutes=TOKEN_EXPIRATION_MINUTES)
    payload: dict[str, str | int] = {
        "sub": str(user_id),
        "username": username,
        "user_id": user_id,
        "jti": str(uuid.uuid4()),
        "type": "access",
        "iat": now,
        "exp": int(expiration.timestamp())
    }

    token: str = jwt.encode(payload, private_key, algorithm=JWT_ALGORITHM)
    return token

async def create_refresh_token(user_id: int, redis_client: redis.Redis) -> str:
    jti = str(uuid.uuid4())
    private_key = get_private_key()
    now = datetime.now(timezone.utc)
    expiration = now + timedelta(minutes=REFRESH_TOKEN_EXPIRE_MINUTES)
    ttl_seconds = int(timedelta(minutes=REFRESH_TOKEN_EXPIRE_MINUTES).total_seconds())

    payload = {
        "sub": str(user_id),
        "user_id": user_id,
        "jti": jti,
        "type": "refresh",
        "iat": now,
        "exp": int(expiration.timestamp()),
    }
    
    token = jwt.encode(payload, private_key, algorithm=JWT_ALGORITHM)

    redis_key = f"refresh_token:{user_id}:{jti}"
    await redis_client.set(redis_key, "active", ex=ttl_seconds)

    return token

def decode_jwt_token(token: str) -> dict[str, str | int] | None:
    public_key = get_public_key()
    try:
        payload: dict[str, str | int] = jwt.decode(
            token, public_key, algorithms=[JWT_ALGORITHM]
        )
        return payload
    except jwt.ExpiredSignatureError:
        print("token has expired")
        return None
    except jwt.InvalidTokenError as e:
        print(f"invalid token: {e}")
        return None