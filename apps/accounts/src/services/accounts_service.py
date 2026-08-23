
from sqlalchemy.ext.asyncio import AsyncSession, create_async_engine
from pb.auth_pb2 import AuthResponse, RegisterRequest, LoginRequest, ValidateTokenRequest, ValidateTokenResponse
from pb.auth_pb2_grpc import AuthServiceServicer
from sqlalchemy import select
from db import AsyncSessionLocal, UserModel
from grpc import StatusCode, aio
from utils import hash_password, create_jwt_token, verify_password, decode_jwt_token

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
            token: str = create_jwt_token(str(user.id), user.username)
            return AuthResponse(
                token=token,
                user_id=int(user.id),
                username=user.username
            )

    async def ValidateToken(self, request: ValidateTokenRequest, context: aio.ServicerContext) -> ValidateTokenResponse: 
        payload = decode_jwt_token(request.token)
        if not payload:
            return ValidateTokenResponse(
                is_valid=False,
                user_id=0,
                username=""
            )

        user_id = int(payload.get("user_id", ""))
        async with AsyncSessionLocal() as session:
            stmt = select(UserModel).where(UserModel.id == user_id)
            result = await session.execute(stmt)
            user = result.scalar_one_or_none()

            if not user:
                return ValidateTokenResponse(
                    is_valid=False,
                    user_id=0,
                    username=""
                )

            return ValidateTokenResponse(
                is_valid=True,
                user_id=int(user.id),
                username=user.username
            )
