# VBF Layer 0 — Source Architecture

Layer 0 is the canonical substrate beneath VBF rules resolution.

## Dependency direction

```text
vbf-types
├── vbf-schema
│   └── vbf-entity
│       └── vbf-relationship
├── vbf-spatial
├── vbf-package
├── vbf-validation
└── vbf-information

vbf-entity + vbf-relationship + vbf-spatial
                    │
                    ▼
                vbf-event
                    │
                    ▼
                vbf-store

package/schema/entity/relationship/spatial/information/validation/store
                    │
                    ▼
               vbf-compiler

vbf-types
    │
    ▼
  vbf-cli
```

No lower crate may depend on a higher crate merely for convenience.

## Crate ownership

### `vbf-types`
Primitive validated types shared by every other crate: identities, keys,
display names, simulation time, event sequence, state revision, distances,
speeds, and angles.

### `vbf-schema`
Describes versioned component schemas, legal fields, typed serialized values,
required/optional status, persistence contexts, structural constraints, and
component-payload validation. Multiple versions of one component schema may
coexist. It does not interpret battlefield rules or resolve cross-record
references.

### `vbf-entity`
Owns reusable Definitions, instantiated Entities, schema-bound `ComponentData`,
Definition/Entity catalogs, template resolution, and Entity-class
compatibility. Core Actor, Asset, Process, Condition, and Objective/Task classes
share one `EntityUid` universe. It does not own spatial location or
relationships.

### `vbf-relationship`
Owns versioned relationship schemas, first-class relationship records,
participant roles, Entity-class eligibility, participant slots, temporal
validity, relationship catalogs, and cross-record exclusivity/capacity
validation. Rules packages define relationship meaning; the Layer 0 crate does
not hard-code Core/Vehicle relationship names as Rust enums.

### `vbf-spatial`
Owns coordinate frames, continuous positions, geometry primitives, spatial
regions, optional grid overlays, and network primitives. Hexes are not
authoritative world coordinates.

### `vbf-package`
Owns package identity, kind, version declaration, dependencies, and
content-lock references.

### `vbf-validation`
Owns validation severities, issue reports, and validator contracts. Domain
modules may later register validators without owning the validation framework.

### `vbf-information`
Owns Layer 0 record structures for Actor-held information, epistemic precision,
and provenance. It does not perform observation resolution.

### `vbf-event`
Owns semantic Event envelopes and primitive State Mutations. Component changes
carry schema-bound `ComponentData`; Relationship changes carry first-class
Relationship records.

### `vbf-store`
Owns authoritative state containers and transaction boundaries. The initial
`MemoryStore` exists for Layer 0 development. SQLite belongs here later behind
the same public contracts.

### `vbf-compiler`
Converts authored Layer 0 records into a bootable initial state and rejects
structural conflicts. Its scope expands as Layer 0 validation becomes concrete.

### `vbf-cli`
Human/automation-facing diagnostics. It is replaceable and never owns
simulation truth.

## Current implementation status

- `vbf-types` provides validated primitive domain types.
- `vbf-schema` provides versioned component schemas and structural payload
  validation.
- `vbf-entity` binds component payloads to exact schema versions; supports
  unique Definition/Entity catalogs; resolves single-parent Definition
  inheritance; rejects inheritance cycles, missing parents, and cross-class
  inheritance; and validates Entity templates and instance components.
- `vbf-relationship` provides versioned relationship schemas and validates
  participant role cardinality, Entity references/classes, slots, temporal
  state, role exclusivity, and per-host slot capacity.
- the real-game regression fixture now represents M8 B-12, its four crew
  Actors, and their individual vehicle-station assignments.
- continuous space and host-relative positions remain present in
  `vbf-spatial`, which is the next Layer 0 subsystem scheduled for development.
- Events carry explicit mutations and schema-bound component/relationship
  changes.
- the in-memory Store applies primitive mutations atomically by candidate-state
  commit.
- the Compiler currently rejects duplicate Entity UIDs/Keys and duplicate
  information keys.

Layer 0 is not complete. The next milestone is to connect these real Entity and
Relationship records to continuous/host-relative Vaux spatial state.
