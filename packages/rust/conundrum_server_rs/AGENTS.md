# Conundrum server contributor notes

## Crate role

`conundrum_server_rs` exposes Conundrum capabilities to applications and AI clients. It depends on both `conundrum` and `conundrum_db`; keep domain models and persistence in those crates and put transport/orchestration concerns here. The package builds the `cdrm_server` binary and the `cdrm_server_lib` library.

The server has three API surfaces: REST (`src/rest/` and `src/routes/`), RPC (`src/rpc/`), and MCP (`src/mcp/`). The `src/rig/` tree holds AI integration/features, and `src/run_server.rs` coordinates startup. Public library exports are intentionally limited in `src/lib.rs`.

The workspace includes `conundrum_macros` for procedural derives and `conundrum_tools` as the dedicated library for MCP tool implementations. Keep MCP transport, server setup, tool discovery/registration, and dispatch in this crate; put reusable MCP tool definitions and integrations in `conundrum_tools`. Database persistence remains in `conundrum_db`.

## Adding a tool or operation

First determine which clients need the capability and inspect the closest existing implementation in each relevant surface. MCP, RPC, and REST have different request/response and registration patterns; do not assume adding a function in one surface exposes it in the others. For MCP, trace tool registration and dispatch through `src/mcp/` and the tool index. For RPC, follow the router composition under `src/rpc/routers/`. Reuse database models and parameter types where appropriate instead of defining transport-specific copies.

## Change guidance

- Keep transport handlers thin: validate/translate input, call shared crate functionality, and map results/errors to that transport's established response shape.
- Preserve async/runtime conventions and avoid blocking work on async executor threads; follow neighboring code for file watching, database, and AI operations.
- Review serialization/schema derives and generated client types when changing public API shapes.
- Treat filesystem paths, workspace selection, and user data as trust boundaries. Reuse existing validation and scope checks.
- Avoid logging secrets, prompts, or private document contents.
- Keep provider-specific AI behavior behind existing abstractions where possible so local/offline and remote-provider paths remain viable.

## Useful commands

Run from this crate directory:

```sh
cargo check -p conundrum_server_rs
cargo test -p conundrum_server_rs
```

The binary is named `cdrm_server`; follow the README or existing launch configuration for runtime environment and storage setup.
