import { createReactQueryHooks } from "@rspc/react";
import { FetchTransport, createClient } from "@rspc/client";
// Import the generated TypeScript definitions exported from Rust
import type { ProceduresLegacy } from "../codegen/bindings.ts";

// Export typed hooks for React
export const rspc = createReactQueryHooks<ProceduresLegacy>();

// TODO: FIx this type issue. It works, but it's definitely broken.
export const client = createClient<ProceduresLegacy>({
    transport: new FetchTransport(
        `http://localhost:${import.meta.env.CDRM_SERVER_PORT ?? "3005"}/rpc`,
    ),
});
