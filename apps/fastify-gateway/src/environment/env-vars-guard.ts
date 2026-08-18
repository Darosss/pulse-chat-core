import type { CustomRequiredProcessEnv } from "./environment";
import dotenv from "dotenv";
dotenv.config({ quiet: false, path: "../../.env" });
const REQUIRED_ENVS: (keyof CustomRequiredProcessEnv)[] = [
    "MESSAGE_SERVICE_URL",
];

(() => {
    const notProvidedEnvs: typeof REQUIRED_ENVS = [];
    for (const requiredEnv of REQUIRED_ENVS) {
        const envVar = process.env[requiredEnv];
        if (envVar == null || envVar == undefined)
            notProvidedEnvs.push(requiredEnv);
    }

    if (notProvidedEnvs.length > 0) {
        throw new Error(
            `Please, provide environment variables [${notProvidedEnvs.join(",")}]`
        );
    }
})();
