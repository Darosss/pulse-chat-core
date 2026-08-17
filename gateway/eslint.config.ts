// @ts-check
import { defineConfig } from "eslint/config";
import js from "@eslint/js";

export default defineConfig({
    files: ["**/*.{js,ts}"],
    extends: ["eslint:recommended", js.configs.recommended],
});
