# AI-Generated Code Overview

## Runtime & HTTP Stack
`src/main.rs` bootstraps a Tokio runtime, builds a shared `LinkService` inside an `Arc<Mutex<_>>`, and wires an Axum `Router` with two routes: `POST /links` and `GET /r/:code`. The router is augmented with `utoipa_swagger_ui`, so visiting `/docs` serves Swagger UI backed by the OpenAPI document at `/api-doc/openapi.json`. Tracing is initialized via `tracing-subscriber`, and the app listens on `0.0.0.0:3000`.

## Domain & Conversion Layer
`src/models/link.rs` defines the `Link` aggregate with UUID, short code, target URL, creation timestamp, and click counter. Factory helpers supply IDs/timestamps, while `increase_clicks` mutates the counter in place. DTOs in `src/dto` (`CreateLinkDto`, `LinkDto`) derive `Serialize`, `Deserialize`, and `ToSchema` so they feed both Axum extractors and the OpenAPI generator. The tiny `Dto<T>` trait keeps conversion logic colocated with the entities.

## Persistence & State
`src/persistence/in_memory_store.rs` implements the `Storage<Link>` trait with HashMaps keyed by UUID and code plus a `HashSet` to enforce unique targets. The trait in `src/persistence/storage.rs` now returns `Result` from `store` and exposes `get_by_code_mut`, preparing the ground for future database adapters. The in-memory store remains the default engine but can be swapped once `LinkService` is genericized.

## Service & HTTP Flow
`LinkService` (`src/service/link_service.rs`) owns the store and a `CodeGenerator`. `create_link` calls `CodeGenerator::generate`, persists the link, and returns a `LinkDto`. `visit_link` fetches by code, increments clicks, and hands the DTO back. `src/api/links.rs` exposes thin Axum handlers that lock the shared service, invoke those methods, map errors to HTTP status codes, and use `Redirect::temporary` for visit responses. Utoipa annotations (`#[utoipa::path]` and `#[derive(OpenApi)]`) make the handlers discoverable in Swagger UI.

## Control Flow Summary
1. Browser or Swagger UI sends `POST /links` with a JSON `CreateLinkDto`.
2. Axum extracts the DTO, locks the service, and calls `create_link`.
3. The service persists a `Link`, converts it to `LinkDto`, and returns it with HTTP 201.
4. Clients resolve short links via `GET /r/{code}`; the handler calls `visit_link` and responds with a 307 redirect to `LinkDto.target_url` or 404 if the code is unknown.
This separation keeps transport, domain, and persistence concerns isolated for future rewrites.
