# `vbf-entity`

`vbf-entity` owns the canonical Layer 0 representation of reusable Definitions and instantiated Entities.

## Core rules

- Actor, Asset, Process, Condition, and Objective/Task are `EntityClass` values within one `EntityUid` identity universe.
- Every component payload carries an exact `ComponentSchemaRef`.
- Definition components validate in `SchemaContext::Definition`.
- Entity components validate in `SchemaContext::InitialState` or `RuntimeState`.
- A Definition may extend one parent Definition of the same Entity class.
- Inherited components are resolved root-to-child. A child component with the same component key replaces the parent's complete component payload; no silent field-level merge occurs.
- Definition and Entity catalogs enforce unique UIDs and stable human-readable Keys.
- Entity template references must resolve, and template class must match Entity class.

## Boundary

This crate does **not** own:

- crew/organization relationships;
- spatial position or geometry;
- Action legality;
- derived game values;
- combat, vehicle, personnel, or other domain effects.

Those systems may consume Entities but must not bypass the component-schema boundary.

## First real-game regression fixture

`tests/m8_entities.rs` uses the 42nd Cavalry Reconnaissance Squadron M8 Light Armored Car as the first non-synthetic Entity fixture. It represents the M8 as an Asset definition and B-12 as an Asset instance, alongside the historical four crew-member Actors. Crew-to-station assignment is deliberately deferred to `vbf-relationship`.
