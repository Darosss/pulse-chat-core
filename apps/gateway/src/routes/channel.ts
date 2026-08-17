import type { FastifyInstance } from "fastify";
import { messageClient } from "../grpc/messageClient.js";

interface MessagesParams {
    id: string;
}
interface MessagesQuery {
    limit?: number;
}

export async function channelRoutes(fastify: FastifyInstance) {
    fastify.get("/api/v1/channels/:id/messages", async (request, reply) => {
        const { id } = request.params as MessagesParams;
        const { limit } = request.query as MessagesQuery;

        return new Promise((resolve, reject) => {
            messageClient.getChannelHistory(
                {
                    channelId: id,
                    limit: limit || 50,
                },
                (error, response) => {
                    if (error) {
                        reply.status(500);
                        return reject({
                            error: `Failed to fetch channel history (${id})`,
                            details: error.message,
                        });
                    }
                    resolve(response);
                }
            );
        });
    });
}
