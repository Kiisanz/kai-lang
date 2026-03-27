# Kai Programming Language

[![License MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](./LICENSE)
[![Development Status](https://img.shields.io/badge/status-experimental-orange)](./README.md)
[![Roadmap](https://img.shields.io/badge/roadmap-on--track-blue)](./README.md)

Kai-lang is an experimental, statically typed programming language for building web backends, where the gap between your code and the outside world is explicit, visible, and tracked.

Its name reflects breadth (Ocean 海) and openness (Beginning 开) in language design.

## The Problem

Every backend service depends on things outside its own code: a database with a schema that changes, external APIs that deprecate endpoints, event streams that evolve their payloads, config keys that get renamed.

Most languages treat these as your problem. Type mismatches surface at runtime. Schema drift is discovered in production. A breaking API change shows up as a crash, not a compiler error.

Kai takes a different position: external contracts are part of the type system, and when they drift, Kai tells you before you ship.

## What This Looks Like

### Database queries with schema-aware types

```kai
type Order = {
    total: Decimal
}

type UserWithOrders = {
    id:     UserId   // UUID, not String
    name:   String
    orders: Order[]
}

fn getPaidOrders() -> Result<UserWithOrders[], Error> {
    dsl sql -> UserWithOrders[] {
        select users.id, users.name, orders.total
        from users
        join orders on users.id = orders.user_id
        where orders.status = "PAID"
    }
}
```

Column types map directly to Kai types with no manual casting. `UserId` is a distinct type from `String`, not an alias. The compiler verifies the query against your schema snapshot at build time.

### Schema drift warnings, when you ask

```kai
dsl sql -> User[] {
    select id, name from users
}
```

Run `kai check --schema` and you might see:

```
warning: column 'name' will be removed in migration 2026_04
```

Or with a version mismatch:

```
error: schema version mismatch, compiled against v12, current is v14
hint: run `kai db sync` to update
```

By default, Kai stays quiet. Warnings appear when you ask, or when CI enforces them.

### Versioned contracts

```kai
// acknowledging this query targets schema v12
dsl sql(v12) -> User[] {
    select id, name from users
}
```

Useful during multi-service deployments where not everything migrates at once. But it is not a free pass:

```
warning: schema debt, v12 is 4 versions behind current (v16)
hint: run `kai debt` to see full impact
```

### External API contracts from OpenAPI

```bash
kai api sync https://api.stripe.com/openapi.json
```

```kai
dsl api("stripe", v3) -> PaymentIntent {
    POST /payment_intents
    body: { amount: int, currency: String }
}
// warning: stripe v3 is 2 versions behind (v5)
// breaking change in v4: 'currency' now requires ISO 4217 enum
```

### Escape hatch for when the spec is wrong

External specs are not always accurate. Kai provides an override mechanism, but it is visible and tracked:

```kai
@override("field 'amount' actually nullable, spec incorrect as of v3")
dsl api("stripe", v3) -> PaymentIntent {
    POST /payment_intents
}
// warning: override active, you own this contract now
```

Overrides show up in `kai debt` and get flagged if the vendor spec is later corrected.

## `kai debt`

```
$ kai debt

Debt report:
  [database]  sql(v12)          4 versions behind
  [api]       stripe v3         breaking change in v4 unacknowledged
  [api]       stripe.amount     manual override active, verify still valid
  [event]     event(v2)         field 'userId' type changed int to uuid

Overrides:  1 (needs human review)
Drift:      3 contracts
Risk:       HIGH, 1 breaking change unacknowledged
```

## Design Philosophy

**Quiet by default.** Kai does not interrupt your flow. No warnings unless you ask, no noise during development.

**Explicit when it matters.** `kai check`, `kai debt`, `kai build --strict` give you the full picture when you need it.

**Honest about limits.** External specs are not always correct. Migrations do not always run before deploys. Kai gives you tools to handle this, not a false sense of safety.

**Informed, not opinionated.** Kai trusts you know what you are doing, but makes sure you can always find out if something has changed.

## Supported Contract Types

| Source | Sync Command | DSL |
|---|---|---|
| Database migrations | `kai db sync` | `dsl sql` |
| OpenAPI spec | `kai api sync` | `dsl api` |
| AsyncAPI spec | `kai event sync` | `dsl event` |
| JSON Schema | `kai config sync` | `dsl config` |

## Language Features

- Strong static typing with optional type inference
- Distinct primitive types (UUID is not String)
- Structured concurrency via `async`, `spawn`, and `par`
- First-class DSL blocks with type-checked interpolation
- Result-based error handling

## Development Status

Kai-lang is experimental and not production-ready.

### v0.1.0 (current)

- Grammar and lexer complete
- Parser and basic AST in progress
- Variable declarations stable
- Function declarations in stabilization

### v0.2.0

- Full AST
- Semantic analysis
- Basic type checking

### v0.3.0

- IR and code generation
- Minimal runtime
- Hello world compiles and runs

### v0.4.0

- Standard library foundations (io, string, result)
- Basic HTTP server support
- JSON in and out

### v0.5.0

- `dsl sql` with schema snapshot and type mapping
- `kai db sync` command
- Schema version mismatch detection

### v0.6.0

- `dsl api` with OpenAPI sync
- `kai debt` command
- Override mechanism with tracking

### v0.7.0

- Tide formatter
- Compass linter
- Lantern language server (LSP)

### v0.8.0

- Buoy testing framework
- KaiDoc documentation generator
- Dock package manager

### v1.0.0

- Self-hosted: at least one real service running on Kai
- Stable enough for daily use by the author

## Tooling Ecosystem

| Tool | Purpose |
|---|---|
| Kaido | Compiler, translates .kai to native executables |
| Dock | Package manager |
| Tide | Code formatter |
| Compass | Linter |
| Lantern | Language server (LSP) |
| Buoy | Testing framework |
| KaiDoc | Documentation generator |

## License

MIT, see [LICENSE](./LICENSE)
