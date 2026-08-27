import os
import redis.asyncio as redis

_redis_pool: redis.Redis | None = None

def get_redis_url() -> str:
    return os.getenv("ACCOUNTS_REDIS_URL", "redis://localhost:6379/0")

fn_init = None

async def init_redis() -> redis.Redis:
    global _redis_pool
    if _redis_pool is None:
        _redis_pool = redis.from_url(
            get_redis_url(),
            encoding="utf-8",
            decode_responses=True,
            max_connections=20, 
        )
    return _redis_pool

def get_redis() -> redis.Redis:
    if _redis_pool is None:
        raise RuntimeError("Redis client is not initialized. Call `init_redis()` at startup.")
    return _redis_pool

async def close_redis() -> None:
    global _redis_pool
    if _redis_pool is not None:
        await _redis_pool.close()
        _redis_pool = None

def get_blacklist_access_token_key(jti: str) -> str:
    return f"auth:blacklist:jti:{jti}"

def get_refresh_token_key(user_id:int, jti: str) -> str:
    return f"auth:refresh_token:{user_id}:{jti}"