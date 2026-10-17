# Conundrum macros contributor notes

## Crate role

`conundrum_macros` provides procedural derive macros used across the Conundrum Rust workspace. It generates database schema/entity/CRUD implementations and property helpers. Shared contracts used by generated implementations live in `conundrum_macro_traits`; the macros should generate code against those contracts rather than duplicate them. Keep database persistence and runtime behavior in `conundrum_db` or its consumers.

## Where to look

- `src/lib.rs` declares and exports the procedural macros.
- `src/database/` contains derives for database entities, schemas, partial models, and default CRUD behavior.
- `conundrum_macro_traits` contains support traits and types expected by generated code.
- `README.md` documents the crate's intended use and examples.

## Change guidance

- Treat generated Rust as a public API: inspect representative invocations and downstream consumers before changing generated names, bounds, attributes, or trait requirements.
- Keep parsing and diagnostics robust. Prefer `syn` parsing and useful compile errors over panics or silently empty output for invalid input.
- Preserve compatibility with the workspace's Rust edition and dependency versions; avoid emitting syntax or paths that consumers cannot resolve.
- When changing a derive, update its examples and relevant macro expansion/compile tests where available.
- Keep this crate focused on code generation; MCP tools and runtime database operations belong in `conundrum_tools` and `conundrum_db`, respectively.

## Useful commands

Run from the workspace root:

```sh
cargo check -p conundrum_macros
cargo test -p conundrum_macros
```
