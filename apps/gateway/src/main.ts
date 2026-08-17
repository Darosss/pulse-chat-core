import "./environment/env-vars-guard.js";
import Fastify from "fastify";
import { channelRoutes } from "./routes/channel.js";
const fastify = Fastify({
    logger: true,
});

fastify.get("/", async function handler(_request, _reply) {
    return { gateway: "home" };
});

async function bootstrap() {
    await fastify.register(channelRoutes);
    try {
        await fastify.listen({ port: Number(process.env.APP_PORT) || 3000 });
    } catch (err) {
        fastify.log.error(err);
        process.exit(1);
    }
}
bootstrap();
