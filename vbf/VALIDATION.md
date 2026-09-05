# VBF Validation Record

## 2026-08-12 — `vbf-types` bootstrap

Environment: Windows PowerShell / Rust toolchain on project owner's workstation.

### Results

- `cargo check --workspace`: **PASS**
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`: **PASS**
- `cargo test --workspace`: **PASS**
  - 24 unit tests passed
  - 0 failed
  - 0 ignored
  - 0 measured
  - 0 filtered out
  - 0 doctests present
- `cargo fmt --check`: reported formatting-only differences in `id.rs`, `lib.rs`, `quantity.rs`, and `time.rs`.
  - No semantic/compiler/test failure was reported.
  - The reported rustfmt formatting was incorporated into the canonical source after this run.
  - A subsequent `cargo fmt --check` should confirm the normalized source.

### Validated behaviors

The passing unit suite covers:

- generated Entity UIDs are non-nil UUIDv7 values;
- generated Entity UIDs are distinct and ordered;
- UID text and JSON round trips;
- nil UID rejection;
- valid and malformed human-readable Key handling;
- borrowed `HashMap` lookup by string key;
- Key JSON round trip;
- Unicode Display Names;
- malformed Display Name rejection;
- Display Name JSON round trip;
- angle normalization;
- distance conversion and negative-distance rejection;
- speed conversion;
- quantity JSON round trip;
- initial and monotonic state revisions;
- revision overflow detection;
- non-negative simulation durations;
- integer simulation-time arithmetic;
- independent event ordering at equal simulation times;
- simulation-time JSON round trip.

### Status

`vbf-types` is **compiler-, lint-, and unit-test validated at v0.1.0**, subject to confirmation that the canonical formatting-only update also passes a fresh `cargo fmt --check`.

Future validation records should distinguish:

1. compile/type validation;
2. static lint validation;
3. unit/serialization tests;
4. integration tests;
5. property/fuzz tests;
6. permanent Vaux regression scenarios.


## 2026-08-12 — Full Layer 0 workspace scaffold

### Created

The workspace now contains:

- `vbf-types`
- `vbf-schema`
- `vbf-entity`
- `vbf-relationship`
- `vbf-spatial`
- `vbf-package`
- `vbf-validation`
- `vbf-information`
- `vbf-event`
- `vbf-store`
- `vbf-compiler`
- `vbf-cli`

The Drive tree and root workspace manifest have been read back and confirmed present.

### Status

**PENDING VALIDATION**

The new cross-crate scaffold was authored in an environment without a Rust toolchain. It must not be treated as compiler-validated until the following commands pass on the project workstation:

```bash
cargo fmt --check
cargo check --workspace
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
```

The prior `vbf-types` v0.1 validation remains valid for that crate unless a later change modifies it.

### Expected purpose of this pass

This validation pass is intended primarily to detect:

- dependency-cycle or manifest errors;
- missing imports or re-exports;
- Serde trait incompatibilities;
- ownership/borrowing mistakes in the first Store transaction implementation;
- formatting differences;
- Clippy warnings;
- any cross-crate type mismatch.

Functional acceptance of each new subsystem requires later subsystem-specific tests beyond merely compiling the scaffold.


## 2026-08-12 — Layer 0 scaffold validation attempt 1

### Results

- `cargo fmt --check`: **FAIL — formatting only** in `vbf-event/src/mutation.rs` and `vbf-relationship/src/relationship.rs`.
- `cargo check --workspace`: **FAIL** with Rust error E0204 in `vbf-spatial/src/position.rs`.
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`: **BLOCKED by the same E0204 compile error**.
- `cargo test --workspace`: **BLOCKED by the same E0204 compile error**.

### Root cause

`WorldPosition` derived `Copy` while containing `frame: Key`. `Key` owns a `String`, so it is intentionally not `Copy`. The numeric coordinate fields are copyable, but the complete position record is not.

### Correction

- Removed `Copy` from `WorldPosition`.
- Incorporated the exact rustfmt changes reported for `StateMutation` and `Relationship::involves`.
- Performed a static sweep of all other `Copy` derives in the Layer 0 tree; no other type combining `Copy` with a `Key`/owned-string field was found.

### Status

**FIXED IN SOURCE; REVALIDATION REQUIRED.**

Run the standard four-command gate again. A fresh failure after this point should be treated as a newly exposed scaffold defect rather than a continuation of E0204.

## 2026-08-12 — `vbf-schema` development pass

### Added

- context-aware persistence classes;
- expanded typed field vocabulary;
- recursive typed arrays;
- numeric, string, allowed-value, and array-cardinality constraints;
- component schema self-validation;
- versioned `ComponentSchemaRef`;
- multi-version schema registry;
- exact-version and latest-version lookup;
- owner queries;
- structural JSON payload validation;
- required-field, unknown-field, persistence, domain-type, array-item, and constraint violation reports;
- schema-specific unit tests;
- `vbf-schema/README.md` contract documentation.

### Primitive invariant correction discovered during schema work

`SimDuration` and `Angle` previously used derived Serde deserialization, which could construct invalid private states directly. This pass changes those two types so deserialization goes through their validating constructors. New tests reject negative serialized durations and noncanonical raw angles.

### Status

**PENDING WORKSTATION VALIDATION**

Run:

```bash
cargo fmt --check
cargo check --workspace
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
```

This pass should not be marked validated until all four commands succeed.

## 2026-09-05 — Full migrated workspace validation

Environment: GitHub Actions `ubuntu-24.04` runner; Rust `1.98.0`; Cargo `1.98.0`; rustfmt `1.9.0-stable`; Clippy `0.1.98`.

Validated commit: `67f5b92370719468c1636a84353e6371ac24c2ee` (`Finalize VBF migration cleanup`).

### Results

- `cargo check --workspace`: **PASS**
- `cargo test --workspace`: **PASS**
  - 70 unit/integration tests passed across the workspace
  - 0 failed
  - all doctest targets passed
- `cargo clippy --workspace --all-targets -- -D warnings`: **PASS**
- `cargo fmt --all -- --check`: **PASS**

### Migration state covered by this run

- all 12 workspace crates are present and active;
- September 4 Event, Store, spatial-anchor/pose/motion, correlation UID, and rotational-speed changes are included;
- the current root README is present;
- `TREE.txt` reflects the migrated September 4 source layout;
- temporary migration/probe workflows are removed except the permanent workspace verification workflow;
- temporary `vbf-cargo-verify` and `vbf-transfer` trees are removed;
- tracked Cargo build output (`vbf/target`) is removed.

### Status

**FULL WORKSPACE VALIDATED.**

This record supersedes the earlier pending cross-crate validation notices for the current migrated source. Future architectural changes should preserve the same four-command CI gate and add subsystem-specific regression coverage as the corresponding layers mature.
