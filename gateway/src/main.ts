import Fastify from "fastify";
const fastify = Fastify({
    logger: true,
});

fastify.get("/", async function handler(_request, _reply) {
    return { gateway: "home" };
});

try {
    await fastify.listen({ port: 3000 });
} catch (err) {
    fastify.log.error(err);
    process.exit(1);
}
