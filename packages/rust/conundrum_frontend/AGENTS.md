# Conundrum frontend contributor notes

## Project role

`conundrum_frontend` is the React and TypeScript web client for Conundrum. It owns the browser UI, client-side state, navigation, and presentation of language, database, workspace, and AI capabilities. It consumes shared functionality and types from `@conundrum/main` and `@conundrum/ts`, and communicates with backend services through the established API clients. Keep server transport and orchestration in `conundrum_server_rs`, persistence in `conundrum_db`, and portable language behavior in `conundrum`.

## Where to look

- `src/main.tsx` initializes the web client and mounts the app.
- `src/core/` contains app-wide setup, shared components, hooks, state, utilities, and generated bindings.
- `src/features/` contains product areas such as AI, database, file system, navigation, settings, and workspace management.
- `src/components/` contains shared UI components, including AI-oriented building blocks.
- `src/index.css`, `vite.config.ts`, and `tsconfig.json` define global styling, bundling, aliases, and TypeScript behavior.
- `package.json` declares the Vite scripts and workspace package dependencies.

## Change guidance

- Keep reusable app infrastructure in `src/core/` and feature-specific behavior in the corresponding `src/features/<area>/` module. Follow nearby folder and naming patterns before introducing new structure.
- Treat the frontend as a client of backend APIs. Keep request/response contracts aligned with server routes and RPC definitions, and use shared generated types where available instead of creating parallel shapes.
- Keep language semantics in `@conundrum/main` / `conundrum`, shared cross-client types in their established shared package, and persistence behavior in `conundrum_db`.
- Preserve state conventions and component boundaries. Check existing hooks, state slices, and feature components before adding new global state or cross-feature dependencies.
- Keep accessibility, keyboard behavior, responsive layouts, and loading/error states consistent with neighboring UI.
- Do not expose credentials, private workspace paths, prompts, or document contents in browser logs or fixtures.

## Useful commands

Run from this project directory:

```sh
pnpm dev
```

The package currently defines no test, type-check, or build script. Use the workspace's configured validation commands when they are added; do not treat the placeholder `test` script as a passing test suite.
