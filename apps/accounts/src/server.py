

from dotenv import load_dotenv
load_dotenv()
import os
import asyncio
import grpc
from db import init_db
from services import accounts_service
from pb import auth_pb2_grpc

SERVICE_URL = os.getenv("ACCOUNTS_SERVICE_URL","[::]:50051")

async def serve():
    await init_db()

    server = grpc.aio.server()
    auth_pb2_grpc.add_AuthServiceServicer_to_server(accounts_service.AccountsService(), server)
    
    server.add_insecure_port(SERVICE_URL)
    
    await server.start()
    await server.wait_for_termination()

if __name__ == "__main__":
    asyncio.run(serve())