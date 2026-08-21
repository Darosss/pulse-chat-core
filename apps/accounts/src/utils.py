from datetime import datetime, timedelta, timezone
import os
import jwt
import bcrypt 

JWT_SECRET: str = os.getenv("ACCOUNTS_JWT_SECRET", "super_secret_dev_key_123!")
JWT_ALGORITHM: str = "HS256"
TOKEN_EXPIRATION_HOURS: int = 24


def hash_password(password: str) -> str:
    return bcrypt.hashpw(password.encode("utf-8"), bcrypt.gensalt()).decode("utf-8")


def verify_password(plain_password: str, hashed_password: str) -> bool:
    return bcrypt.checkpw(
        plain_password.encode("utf-8"),
        hashed_password.encode("utf-8"),
    )


def create_jwt_token(user_id: str, username: str) -> str:
    expiration = datetime.now(timezone.utc) + timedelta(hours=TOKEN_EXPIRATION_HOURS)

    payload: dict[str, str | int] = {
        "username": username,
        "user_id": user_id
    }

    token: str = jwt.encode(payload, JWT_SECRET, algorithm=JWT_ALGORITHM)
    return token


def decode_jwt_token(token: str) -> dict[str, str | int] | None:
    try:
        payload: dict[str, str | int] = jwt.decode(
            token, JWT_SECRET, algorithms=[JWT_ALGORITHM]
        )
        return payload
    except jwt.PyJWTError:
        return None