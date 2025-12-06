# Fluster Development Environment

## Requirements

- `pnpm` is assumed as the package manager. It's written directly into many build scripts and an important part of the architecture.
- `cargo-swift` for building the `fluster_rust` package
- `cargo-nextest` for running rust tests.
- `typeshare-cli` for generating cross-language types... mostly enums.

### Environment

- Set `FLUSTER_IOS_ROOT` to the root directory of the ipad application on your system.
- Set `FLUSTER_PROD_BUILD="true"` to build local packages (not the actual xcode project) for production. You can creat a `.env.public` file the root of the monorepo's workspace to set this local to the project.
