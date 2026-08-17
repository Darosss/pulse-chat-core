import type { FastifyInstance } from "fastify";
import { messageClient } from "../grpc/messageClient.js";

interface ChatWSParams {
    channelId: string;
}

export async function chatSocket(fastify: FastifyInstance) {
    fastify.get(
        "/ws/chat/:channelId",
        { websocket: true },
        (connection, req) => {
            const { channelId } = req.params as ChatWSParams;
            fastify.log.info(
                `WebSocket client connected to channel: ${channelId}`
            );
            const grpcStream = messageClient.streamLiveMessages({ channelId });
            grpcStream.on("data", (data) => {
                connection.socket.send(
                    JSON.stringify({ event: "NEW_MESSAGE", payload: data })
                );
            });
            grpcStream.on("error", (err) => {
                fastify.log.error("gRPC chat socket stream error: ", err);
            });
            grpcStream.on("close", () => {
                fastify.log.info(
                    `WebSocket client disconnected from channel: ${channelId}`
                );
                grpcStream.cancel();
            });
        }
    );
}
