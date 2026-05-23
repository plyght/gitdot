# gitdot-core

Business-logic crate for the [gitdot](https://gitdot.io) platform. Hosts every domain service, repository, external client, DTO, model, and error type. The HTTP servers (`gitdot-server`, `gitdot-auth`, `gitdot-metrics`) and the Kafka consumer (`gitdot-consumer`) all delegate to services defined here.

## Layout

```
src/
├── service/       Trait + Impl per domain (business logic)
├── repository/    Trait + Impl per domain (sqlx data access)
├── client/        Trait + Impl for external services (git2, GitHub, S2, ClickHouse, etc.)
├── dto/           Request/response DTOs + validated types (OwnerName, RepositoryName, …)
├── model/         Database row structs (`#[derive(FromRow)]`)
├── error/         Domain-specific error enums (thiserror)
└── util/          Internal helpers (reserved names, token generation)
```

Migrations live in `migrations/` (sqlx `.up.sql` / `.down.sql`).

## Architecture

Every layer is expressed as a trait with a corresponding `Impl` struct, so each layer is independently testable. Services are generic over their repository/client traits; production code wires them up with concrete `*Impl` types.

```
Handler (gitdot-server) → Service (gitdot-core) → Repository (gitdot-core) → PostgreSQL (sqlx)
                                                  Client     (gitdot-core) → git2 / GitHub / S2 / ClickHouse
```

See [`CLAUDE.md`](CLAUDE.md) for the canonical service catalogue and conventions.

## License

Licensed under the [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0). Copyright © gitdot contributors.
