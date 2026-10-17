# Conundrum database contributor notes

## Crate role

`conundrum_db` owns the structured data models, LanceDB-backed persistence, schema setup, query parameters, seed data, and database utilities. The public crate surface is rooted in `src/lib.rs` and exposed primarily through `vector::{models, parameters, database, tools}`.

The dependency direction is intentional: `conundrum_db` depends on `conundrum` (with its `db` feature), while the core must not depend on this crate. Some shared primitives and AI-related types live in `conundrum::lifted_models` or other core modules; check there before adding a duplicate model.

The workspace also has a dedicated `conundrum_macros` crate for procedural derives and a `conundrum_tools` crate reserved for MCP tool implementations. Keep persistence and data models here, put reusable derive generation in the macros crate, and put MCP tool behavior in the tools crate. The server owns MCP transport, registration, and orchestration.

## Where to look

- `src/vector/models/`: serializable domain models and their module exports.
- `src/vector/parameters/`: request/query input types consumed by callers and the server.
- `src/vector/database/`: schema, initialization, and persistence helpers.
- `src/vector/seed/`: initial/example data and documentation seeding.
- `src/vector/tools/` and `src/vector/ai_utils/`: database-side tool definitions and helpers.
- `src/test_utils/`: test database and CRUD helpers.

## Change guidance

- Keep model serialization, schema fields, identifiers, and CRUD behavior aligned. A model edit may require related changes to schema generation, seed data, query parameters, and the server-facing API.
- Check `#[derive]` attributes and generated consumers (including Serde, Schemars, Specta, and Typeshare) before changing public model shapes.
- Prefer existing database helpers and model patterns; inspect similar neighboring models and their tests before introducing a new abstraction.
- Keep database-specific behavior here. If a type must be shared with the language core or WASM consumers, consider whether it belongs in `conundrum` instead.
- Treat schema evolution as a compatibility concern. Inspect schema versioning/initialization paths before changing persisted fields.
- Do not expose credentials, local paths, or user content in logs or fixtures.

## Useful commands

Run from this crate directory:

```sh
cargo check -p conundrum_db
cargo test -p conundrum_db
```

Database and document-processing dependencies can make builds resource-intensive. Run focused tests while iterating, then the relevant crate-level checks when practical.
