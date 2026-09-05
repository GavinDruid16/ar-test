# vbf-schema

`vbf-schema` is the Layer 0 contract for component structure. It describes what a serialized component is allowed to contain without deciding what battlefield rules mean.

## Responsibilities

The crate owns:

- component schema identity and version;
- owning module key;
- field names, types, required/optional status, and persistence class;
- collection/numeric/string constraints;
- coexistence of multiple schema versions;
- structural validation of serialized component payloads.

It deliberately does **not** own:

- Entity existence/reference resolution;
- relationship cardinality between Entities;
- LOS, combat, movement, command, or other domain effects;
- database persistence;
- migrations between schema versions.

## Persistence contexts

A field declares one `PersistenceClass`:

- `definition` — reusable content/template data;
- `initial` — per-instance immutable starting state;
- `mutable` — authoritative state that may change;
- `event` — Event-only data;
- `derived` — calculated read-only output;
- `ephemeral` — UI/cache/session-only data.

Payload validation is context-aware. A required Definition field is not required in Runtime State, and a Definition-only field appearing in Runtime State is a validation error.

## Versioning

The registry keys schemas by `(component key, version)`. Thus:

```text
component.spatial v1
component.spatial v2
```

may coexist, while a second registration of `component.spatial v2` is rejected.

This is required so older saved instances can remain interpretable after a component schema evolves.

## Example

Conceptually, a vehicle state schema can say:

```text
component.vehicle_state v1
owner: module.vehicle

state
  type: string
  required
  persistence: mutable
  allowed: operational | degraded | immobilized | disabled

engine_hours
  type: unsigned integer
  optional
  persistence: mutable
```

The schema can prove that `state = "operational"` is structurally valid. It does not decide whether combat damage should change that state; Vehicle Operations owns that rule.

## Validation boundary

A field declared `EntityUid` is validated as a syntactically valid, non-nil VBF Entity UID. Whether that UID actually identifies an Entity in the current package or battlefield is checked later by cross-record validation.
