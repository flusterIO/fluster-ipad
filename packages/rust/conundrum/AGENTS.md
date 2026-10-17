# Conundrum contributor notes

## Crate role

`conundrum` is the Rust core for the Conundrum language and the home for shared types that must be usable independently of the database crate. It supports native and WebAssembly builds. The optional `db` feature deliberately adds database/server-related dependencies and breaks the lightweight WASM boundary; do not enable it casually in code intended for WASM.

## Where to look

- `src/lang/`, `src/parsers/`, and `src/output/` contain the language pipeline.
- `src/lifted_models/` contains primitives and models moved out of `conundrum_db` so they can be shared without making the core depend on the full data layer.
- `src/ecosystem/` and `src/embedded/` contain ecosystem settings and embedded documentation/assets.
- `roadmap.md` describes language direction; `soul.md` records project values. The crate README gives the broader product context.

## Change guidance

- Keep core language and portable shared types independent of database implementation details. Put database-owned models and persistence in `conundrum_db`.
- Preserve native/WASM compatibility. Check target-specific dependencies and feature gates when adding dependencies or APIs.
- Prefer existing parser, serialization, and error-handling conventions in the surrounding module. Inspect call sites before changing public types because this crate is consumed by the database and server crates as well as other targets.
- Embedded documentation and generated/runtime-consumed assets may have consumers beyond Rust. Follow the generation/update path already used in the relevant module rather than editing generated output in isolation.
- Avoid broad refactors across the large crate when a narrow change can meet the requirement.

## Useful commands

Run from this crate directory:

```sh
cargo check -p conundrum
cargo check -p conundrum --target wasm32-unknown-unknown
```

The WASM command requires the target to be installed. For database-feature work, use `cargo check -p conundrum --features db` and account for its native-only dependency implications.
