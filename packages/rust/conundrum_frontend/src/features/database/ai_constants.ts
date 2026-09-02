export const REMOTE_AI_PROVIDER_KEY = "openai";
export const REMOTE_AI_PROVIDER_NAME = "OpenAI";

export interface ProviderData {
    source: "remote" | "local";
    key: string;
    name: string;
}

export const PROVIDERS: ProviderData[] = [
    {
        source: "remote",
        key: REMOTE_AI_PROVIDER_KEY,
        name: REMOTE_AI_PROVIDER_NAME,
    },
    {
        source: "local",
        key: "ollama",
        name: "Ollama",
    },
] as const;
