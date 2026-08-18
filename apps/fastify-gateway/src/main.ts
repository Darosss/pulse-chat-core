import "./environment/env-vars-guard.js";
import Fastify from "fastify";
import fastifyWebsocket from "@fastify/websocket";
import { channelRoutes } from "./routes/channel.js";
import { chatSocket } from "./websocket/chat.js";
const fastify = Fastify({
    logger: true,
});

fastify.get("/", async function handler(_request, _reply) {
    return { gateway: "home" };
});

async function bootstrap() {
    await fastify.register(fastifyWebsocket);
    await fastify.register(channelRoutes);
    await fastify.register(chatSocket);

    try {
        await fastify.listen({ port: Number(process.env.APP_PORT) || 3000 });
    } catch (err) {
        fastify.log.error(err);
        process.exit(1);
    }
}
bootstrap();
