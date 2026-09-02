import { type ModelDescription } from "#/database/db_utility_types/agent";
import { rspc } from "@/app/rspc_client";

export const useBothProviderModels = (): {
    data: (ModelDescription & { source: "local" | "remote" })[];
    loading: boolean;
} => {
    const { data: localModels, isLoading: localLoading } = rspc.useQuery([
        "ollama.list_models",
        null,
    ]);
    // const {data: remoteModels, isLoading: remoteLoading} = rspc.useQuery(["ollama.list_models", null])

    return {
        data: (localModels ?? []).map((m) => {
            return {
                ...m,
                source: "local",
            };
        }),
        loading: localLoading,
    };
};
