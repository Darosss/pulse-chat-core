
from sqlalchemy.ext.asyncio import AsyncSession, create_async_engine
from pb.auth_pb2 import AuthResponse, RegisterRequest, LoginRequest, LogoutResponse, LogoutRequest, RefreshTokenRequest, RefreshTokenResponse, GetPublicJWTKeyRequest, GetPublicJWTKeyResponse
from pb.auth_pb2_grpc import AuthServiceServicer
from sqlalchemy import select
from db import AsyncSessionLocal, UserModel
from grpc import StatusCode, aio
from utils import hash_password, create_jwt_token, verify_password, decode_jwt_token, create_refresh_token
from redis_manager import get_redis, get_blacklist_access_token_key, get_refresh_token_key
from jwt_keys import get_public_key

class AccountsService(AuthServiceServicer):
    async def Register(self, request: RegisterRequest, context: aio.ServicerContext) -> AuthResponse:
        async with AsyncSessionLocal() as session:
            stmt = select(UserModel).where(UserModel.email == request.email)
            result = await session.execute(stmt)
            existing_user = result.scalar_one_or_none()
            if(existing_user):
                await context.abort(StatusCode.ALREADY_EXISTS, "A user with this email already exists")

            hashed_pw = hash_password(request.password)
            new_user = UserModel(
                username=request.username,
                email=request.email,
                password_hash=hashed_pw
            )
            session.add(new_user)
            await session.commit()
            token = create_jwt_token(new_user.id, new_user.username)
            return AuthResponse(
                token=token, user_id=new_user.id, username=new_user.username
            )
    async def Login(self, request: LoginRequest, context: aio.ServicerContext) -> AuthResponse:
        async with AsyncSessionLocal() as session:
            stmt = select(UserModel).where(UserModel.email == request.email)
            result = await session.execute(stmt)
            user = result.scalar_one_or_none()

            if not user or not verify_password(request.password, user.password_hash) :
                await context.abort(StatusCode.UNAUTHENTICATED, "Invalid email or password")
            token: str = create_jwt_token(user.id, user.username)
            refresh_token = await create_refresh_token(user.id, get_redis())
            return AuthResponse(
                token=token,
                user_id=int(user.id),
                username=user.username
            )
    async def Logout(self, request: LogoutRequest, context: aio.ServiceContext) -> LogoutResponse:
        payload = decode_jwt_token(request.refresh_token) 
        if not payload or payload.get("type") != "refresh":
            context.set_code(StatusCode.UNAUTHENTICATED)
            return LogoutResponse(success=False)

        user_id = payload.get("user_id")
        jti = payload.get("jti")
        
        r = get_redis()
        stored_token = await r.get(get_refresh_token_key(user_id, jti))
        if not stored_token or stored_token != request.refresh_token: 
            context.set_code(StatusCode.UNAUTHENTICATED)
            return LogoutResponse(success=False)
        await r.delete(get_refresh_token_key(user_id, jti))
        return LogoutResponse(success=True)

    async def RefreshToken(self, request: RefreshTokenRequest, context: aio.ServicerContext) -> RefreshTokenResponse:
        payload = decode_jwt_token(request.refresh_token) 
        if not payload or payload.get("type") != "refresh":
            context.set_code(StatusCode.UNAUTHENTICATED)
            return RefreshTokenResponse()

        user_id = payload.get("user_id")
        jti = payload.get("jti")
        
        r = get_redis()
        stored_token = await r.get(get_refresh_token_key(user_id, jti))
        
        if not stored_token or stored_token != request.refresh_token: 
            context.set_code(StatusCode.UNAUTHENTICATED)
            return RefreshTokenResponse()

        await r.delete(get_refresh_token_key(user_id, jti))
        
        new_access_token = create_access_token(user_id)
        new_refresh_token, new_jti = await create_refresh_token(user_id)
        
        await r.set(get_refresh_token_key(user_id, new_jti), new_refresh_token, ex=7*24*3600)

        return RefreshTokenResponse(
            access_token=new_access_token,
            refresh_token=new_refresh_token
        )

    async def GetPublicJWTKey(self, request: GetPublicJWTKeyRequest, context: aio.ServicerContext) -> GetPublicJWTKeyResponse:
        
        public_key_str = get_public_key().decode("utf-8")

        return GetPublicJWTKeyResponse(key=public_key_str)  

    async def blacklist_access_token(self, jti: str, ttl_seconds: int):
        r = get_redis()
        key = get_blacklist_access_token_key(jti)
        
        await r.set(key, "revoked", ex=ttl_seconds)

    async def is_token_blacklisted(self, jti: str) -> bool:
        r = get_redis()
        key = get_blacklist_access_token_key(jti)
        
        return await r.exists(key) > 0
