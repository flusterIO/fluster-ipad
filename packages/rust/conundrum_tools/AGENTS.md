# Conundrum MCP tools contributor notes

## Crate role

`conundrum_tools` is the dedicated library for MCP tool definitions and integrations. It contains reusable tool behavior and MCP-facing dependencies; it is not the server transport or database persistence layer. `conundrum_server_rs` owns MCP server setup, registration, dispatch, and application orchestration. `conundrum_db` owns structured models and persistence.

## Where to look

- `src/lib.rs` is the library's public API root.
- The crate manifest lists the MCP (`rmcp`) and AI/database integration dependencies; keep dependencies scoped to actual tool requirements.
- `conundrum_server_rs/src/mcp/` contains the server-side MCP transport and registration flow that consumes tool functionality.

## Change guidance

- Keep tool implementations reusable and separate from server startup, transport wiring, and request dispatch.
- Align tool input/output schemas with the models and parameter types owned by `conundrum_db`; avoid introducing duplicate representations without a clear boundary need.
- Follow MCP schema, error, and async conventions used by the selected `rmcp` APIs and keep generated/public schemas consistent with runtime behavior.
- Avoid logging credentials, prompts, or private document contents.
- Keep provider- or database-specific behavior behind the existing integrations where practical, and avoid pulling server-only dependencies into the library.

## Useful commands

Run from the workspace root:

```sh
cargo check -p conundrum_tools
cargo test -p conundrum_tools
```
