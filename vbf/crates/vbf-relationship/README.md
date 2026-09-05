# `vbf-relationship`

`vbf-relationship` owns Layer 0 relationship structure. Core and domain rules
packages define relationship meaning by registering versioned schemas; this
crate validates whether relationship records are structurally legal.

## Architectural rules

- Relationship types are versioned by stable Key + exact version.
- Relationships are first-class records with their own UID and stable Key.
- Relationships may be binary or n-ary.
- Participants have named semantic roles.
- A participant may optionally occupy a named internal slot such as a vehicle
  station.
- Role schemas declare minimum/maximum cardinality and permitted Entity classes.
- Slot rules may require, permit, or forbid slots and may enumerate legal slots.
- Relationship properties reuse `vbf-schema` field typing and persistence rules.
- Relationship intervals are start-inclusive and end-exclusive.
- Cross-record rules may enforce role exclusivity and per-host slot capacity.
- Entity existence and Entity class are checked through `vbf-entity`.
- Relationship schemas do not execute game effects.

## Rules boundary

Current Box Core rules distinguish, among others:

- `Attached To`;
- `Carried By`;
- `Crewed By`;
- `Operated By`;
- `Occupied By`;
- `Stored By`;
- `Commanded By`;
- `Remote Operation`;
- `Tethered To`;
- Shared-Token Representation.

Vehicle Operations additionally defines relationships including:

- `Towed By`;
- `Coupled To`;
- `Guided By`;
- `Mounted On`;
- Mutual Support.

These names are **not hard-coded Rust enums**. A rules/content package registers
the appropriate relationship schema and its roles, slots, properties, and
cross-record constraints.

This keeps Layer 0 era-neutral and allows later rules versions to coexist with
older scenario/save data.

## What this crate validates

For one relationship record:

- exact relationship schema exists;
- validity interval is legal;
- required roles are present;
- role cardinalities are not exceeded;
- participant roles exist;
- participant Entity references resolve;
- participant Entity classes are eligible;
- duplicate participants are rejected;
- same-Entity multi-role use is rejected unless the schema permits it;
- required/forbidden/allowed slot rules are enforced;
- property payloads match the schema.

Across active relationships:

- an exclusive participant role may appear in only one active relationship of
  that schema;
- a host+slot may not exceed its declared capacity.

## What this crate does not validate yet

It does not decide:

- whether an Actor has the qualification to use a station;
- whether a vehicle has minimum crew for movement;
- whether a command link is physically available;
- whether a carried subject inherits Defense;
- whether a relationship changes position;
- whether a tow is geometrically possible;
- whether an attachment order is authorized;
- whether an occupied station unlocks an Action.

Those are higher-layer or domain-rule consequences.

## First game regression

`tests/m8_crew_relationships.rs` represents Baker Troop M8 B-12 with its four
separate crew Actors and four historical vehicle stations.

The regression proves that:

1. four distinct crew assignments are structurally legal;
2. a Capacity-1 station cannot contain two crew Actors at once;
3. one crew Actor cannot simultaneously Crew two vehicles under the same
   `Crewed By` relationship schema;
4. an unknown M8 station is rejected;
5. a station handoff may end and begin at the same simulation instant without
   temporal overlap.

The relationship layer does not yet evaluate crew qualifications or vehicle
movement permission.
