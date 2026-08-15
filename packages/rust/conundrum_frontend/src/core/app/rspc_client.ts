import { createReactQueryHooks } from "@rspc/react";
import { FetchTransport, createClient } from "@rspc/client";
// Import the generated TypeScript definitions exported from Rust
import type { ProceduresLegacy } from "../codegen/bindings.ts";

// Export typed hooks for React
export const rspc = createReactQueryHooks<ProceduresLegacy>();

export const getServerPort = (): string | number => {
    return import.meta.env.CDRM_SERVER_PORT ?? "3005"
}

export const client = createClient<ProceduresLegacy>({
    transport: new FetchTransport(
        `http://localhost:${getServerPort()}/api/rpc`,
    ),
});
