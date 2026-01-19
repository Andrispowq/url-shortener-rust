# AI-Generated Code Overview

## Entry Point & Modules
`src/main.rs` declares the core modules (`models`, `persistence`, `api`, `service`, `dto`) and is expected to remain slim. Expand behavior by adding startup wiring (e.g., router construction, storage implementations) inside dedicated modules instead of bloating `main`.

## Domain Model
`Link` in `src/models/link.rs` encapsulates the shortened URL metadata (UUID id, short code, target URL, timestamp, click count). `Link::new` currently generates the UUID and timestamp, while helper methods (like `increase_clicks`, once implemented) should mutate or clone the struct before persistence.

## Persistence Layer
`InMemoryStore` (`src/persistence/in_memory_store.rs`) stores links in two hash maps: `links` (keyed by UUID) and `code_to_id`. The `Storage` trait defines the operations (`store`, `get_by_id`, `get_by_code`) used by services so you can later swap in persistent backends (Postgres, Redis, etc.) without changing business logic.

## Service Layer
`LinkService` (`src/service/link_service.rs`) is the orchestration point. `create_link` builds a `Link`, passes it through the DTO transformer (`Dto<LinkDto>` impl), and writes it to the store. `visit_link` looks up a link by code, increments the click counter, and returns the DTO, making this the place to add validation, auth, or rate limiting.

## DTOs & API Surface
DTO helpers live in `src/dto`. `LinkDto` mirrors the public-facing fields and is derived `Serialize`/`Deserialize` for Axum or any HTTP layer. `CreateLinkDto` holds request payloads, and the blanket `Dto<T>` trait centralizes conversions so new entities follow the same pattern. The `api` module will expose handlers (currently `api::links`) that deserialize DTOs, invoke `LinkService`, then serialize responses.

## Typical Request Flow
1. HTTP handler in `api::links` receives JSON `CreateLinkDto`.
2. Handler calls `LinkService::create_link`, which constructs and stores a `Link`.
3. The stored `Link` is converted into `LinkDto` and returned to the client.
4. Subsequent visits call `visit_link`, which looks up by short code and increments clicks before responding.
This flow keeps I/O, domain logic, and serialization concerns isolated, easing rewrites or backend swaps.
