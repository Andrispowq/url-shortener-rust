# AI-Generated Suggestions

## Code Quality Rating
Overall score: **6/10**. The project now exposes working HTTP endpoints and auto-generated docs, and the layering is cleanly separated. Remaining technical debt centers on ergonomics (error types, storage abstractions), runtime performance (coarse mutex locking), and test coverage for the HTTP surface.

## Targeted Improvements
1. **Stabilize error handling** (`src/service/link_service.rs`, `src/persistence/*`). Methods still return raw `String` errors that the HTTP handlers interpret via string matching. Introduce a `LinkError` enum (implementing `Display` + `StdError`) so Axum routes can match on typed variants and automatically map to status codes.
2. **Avoid cloning on store** (`src/persistence/in_memory_store.rs:20-34`). `Storage::store` clones the entire `Link` just to keep an owned copy for the HashMap while also returning the original `Link`. Consider taking `&Link` (or returning `()`), or storing via `self.links.insert(id, value)` without cloning to cut allocations.
3. **Revisit unique-target semantics** (same file). Enforcing global uniqueness of `target_url` through `unique_targets: HashSet<_>` is surprising for a URL shortener and blocks legitimate duplicates. Either remove the guard or scope it to features like "custom alias" instead of all links.
4. **Replace global `Arc<Mutex<LinkService>>` with finer-grained locks** (`src/main.rs`, `src/api/links.rs`). Every HTTP call acquires a mutex guarding the whole service, limiting concurrency. Prefer an `Arc<LinkService>` where only the storage backend uses interior mutability (e.g., `RwLock` per map) or move to async-friendly store traits that operate on `&self` + interior locks.
5. **Retire deprecated RNG usage** (`src/service/code_generator.rs`). `thread_rng()` is deprecated in `rand 0.10`. Switch to `rand::rng()` or hold a reusable RNG to silence warnings and avoid repeated initialization costs.
6. **Exercise the HTTP surface with tests** (`tests/` missing). Add Axum integration tests (spawning the router with `tower::ServiceExt`) covering `POST /links` success/failure paths and redirect responses from `GET /r/:code`.
7. **Expose observability on redirects** (`src/api/links.rs`). Currently only 404 vs. 500 is logged via status codes. Emit tracing spans/structured logs when a redirect occurs (include code + target) to aid debugging.
8. **Document timestamp & redirect semantics in OpenAPI** (`src/api.rs`, DTO schemas). The generated schema lacks format hints (e.g., `format = "date-time"` for `created_at`) and does not describe that `GET /r/{code}` responds with 307 redirects. Annotate schemas/responses to make the Swagger UI self-explanatory.
