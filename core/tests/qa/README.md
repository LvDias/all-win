# qa-backend — QA [Backend] test crate

Black-box tests of the Core through public crate APIs and `crates/app` wiring. No production code,
`publish = false`, dev-dependencies only. Implementers review; DevOps commits.

| File | Owner |
|---|---|
| `Cargo.toml`, `src/`, this README | Sonda (QA Backend) |
| `tests/s6_profiles.rs`, `tests/s7_routing.rs` | Sonda |
| `tests/s5_accounts.rs`, S3 identity case list | Prisma |
| `tests/s3_identity.rs` | Agulha |

Each slice's test target is gated by a feature until the slice is on `main`
(`[[test]] required-features`). Add your `[[test]]` entry and dev-dependencies by asking the
crate owner (single writer of `Cargo.toml`).

```bash
cargo test -p qa-backend                          # main: gated targets are skipped
cargo test -p qa-backend --features s7            # at an S7 slice SHA (disposable merge)
ALLWIN_REQUIRE_DB_TESTS=1 cargo test -p qa-backend --features s6   # PG tests need qa-db
```

Spec authority: ADR-0003 and the HTTP contract. Cases pending a decision are `#[ignore = "pending …"]`.
