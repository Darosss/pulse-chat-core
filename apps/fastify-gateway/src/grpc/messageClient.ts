import * as grpc from "@grpc/grpc-js";
import { MessageServiceClient } from "../pb/message.js";

export const messageClient = new MessageServiceClient(
    process.env.MESSAGE_SERVICE_URL,
    grpc.credentials.createInsecure()
);
