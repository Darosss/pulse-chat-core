from sqlalchemy.ext.asyncio import create_async_engine, async_sessionmaker, AsyncSession
from sqlalchemy.orm import DeclarativeBase, Mapped, mapped_column
import os

DATABASE_URL = os.getenv("ACCOUNTS_SQL_CONNECTION_STRING", "sqlite+aiosqlite:///./identity.db")
engine = create_async_engine(DATABASE_URL, echo=False)

AsyncSessionLocal = async_sessionmaker(engine, expire_on_commit=False, class_=AsyncSession)

class Base(DeclarativeBase):
    pass

class UserModel(Base):
    __tablename__ = "users"

    id: Mapped[int] = mapped_column(primary_key=True)
    email: Mapped[str] = mapped_column(unique=True, index=True)
    password_hash: Mapped[str]
    username: Mapped[str]

async def init_db():
    pass
    async with engine.begin() as conn:
        await conn.run_sync(Base.metadata.create_all)