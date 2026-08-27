

from dotenv import load_dotenv
load_dotenv()
import os
import asyncio
from grpc import aio
from db import init_db
from services import accounts_service
from pb import auth_pb2_grpc
from redis_manager import init_redis, close_redis
from jwt_keys import load_keys
SERVICE_URL = os.getenv("ACCOUNTS_SERVICE_URL","[::]:50051")
async def serve():
    await init_db()
    await init_redis()
    load_keys()
    server = aio.server()
    auth_pb2_grpc.add_AuthServiceServicer_to_server(accounts_service.AccountsService(), server)
    
    server.add_insecure_port(SERVICE_URL)
    
    try:
        await server.start()
        await server.wait_for_termination()
    finally:
        await close_redis()
    

if __name__ == "__main__":
    asyncio.run(serve())