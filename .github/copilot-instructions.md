# Copilot instructions for dufs

## Project map (big picture)
- CLI entrypoint is in [src/main.rs](src/main.rs). It parses config/CLI into `Args`, initializes logging, creates a `Server`, and starts Hyper listeners.
- Core request handling lives in [src/server.rs](src/server.rs): `Server::call()` → `Server::handle()` → many `handle_*` methods for file ops, WebDAV, and HTML/JSON listings.
- Access control is implemented in [src/auth.rs](src/auth.rs) via `AccessControl`/`AccessPaths` (basic/digest auth + signed token flow).
- HTML UI assets are embedded at compile time from assets/ and injected with `__ASSETS_PREFIX__` + `__INDEX_DATA__` in [src/server.rs](src/server.rs).
- Directory listing data model is `IndexData`/`PathItem` in [src/server.rs](src/server.rs); the no‑JS fallback HTML is generated in [src/noscript.rs](src/noscript.rs).

## Request/feature flow conventions
- Path handling must go through `resolve_path()` + `join_path()` in [src/server.rs](src/server.rs) to enforce path prefixing and Windows drive-letter safety.
- Symlink safety is guarded by `allow_symlink` and `is_root_contained()` in [src/server.rs](src/server.rs); do not bypass these checks.
- Query flags used by the UI/API are parsed in `handle()` (e.g., `?simple`, `?json`, `?noscript`, `?zip`, `?hash`, `?tokengen`). Match existing behavior when adding flags.
- WebDAV methods (PROPFIND, MKCOL, COPY, MOVE, LOCK, etc.) are implemented as `handle_*` methods in [src/server.rs](src/server.rs). Keep new WebDAV handling within that file.

## Configuration and logging
- Config file is YAML with kebab-case keys mapped to `Args` in [src/args.rs](src/args.rs). CLI args override config values.
- HTTP logging format is parsed in [src/http_logger.rs](src/http_logger.rs); application logging uses the simple logger in [src/logger.rs](src/logger.rs).

## Build/test workflows
- Build: `cargo build` (TLS is enabled by default).
- Disable TLS feature: `cargo build --no-default-features` (TLS is the only default feature; see [Cargo.toml](Cargo.toml)).
- Tests are mostly integration tests under tests/ (run with `cargo test`).

## Commit conventions
- Use Conventional Commits prefixes: `feat:`, `fix:`, `chore:`.

## Patterns to follow when changing behavior
- Use `IndexData`/`PathItem` for any directory listing or UI data changes; keep JSON and HTML outputs consistent.
- When touching auth logic, update both token flow in [src/auth.rs](src/auth.rs) and call sites in [src/server.rs](src/server.rs).
- Asset overrides (`--assets`) must contain index.html; keep injection placeholders intact in [src/server.rs](src/server.rs).
