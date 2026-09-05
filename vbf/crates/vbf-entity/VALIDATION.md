# `vbf-entity` Validation Record

## 2026-08-12 — development pass

### Added

- schema-bound `ComponentData`;
- exact component-schema version references;
- Definition and Entity component validation by schema context;
- unique-key/unique-UID Definition and Entity catalogs;
- single-parent Definition inheritance;
- inheritance cycle, missing-parent, and cross-class rejection;
- resolved Definition lineage and component overlay;
- Entity template resolution and class compatibility checks;
- downstream `StateMutation::SetComponent` conversion from anonymous JSON to `ComponentData`;
- downstream Store compatibility update;
- first real-game regression fixture using M8 B-12 and its historical four-person crew.

### Source basis for first game fixture

The M8 fixture uses the current 42nd Cavalry Reconnaissance Squadron M8 QRC for the vehicle's Asset role, 6x6 drive, armored/open-topped configuration, four-person crew, 54 US gallon fuel capacity, and crew-role names. Station relationships are deliberately deferred to `vbf-relationship`.

### Status

**PENDING WORKSTATION VALIDATION**

Run from the VBF root:

```powershell
cargo fmt --check
cargo check --workspace
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
```

A failure in `vbf-event` or `vbf-store` may be caused by the intentional `SetComponent` API change and should be treated as part of this Entity integration pass.
