export interface CustomProcessEnv {
    APP_PORT?: number;
}
export interface CustomRequiredProcessEnv {
    MESSAGE_SERVICE_URL: string;
}

declare global {
    namespace NodeJS {
        interface ProcessEnv
            extends CustomProcessEnv, CustomRequiredProcessEnv {}
    }
}
export {};
