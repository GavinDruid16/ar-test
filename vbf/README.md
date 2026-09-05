# VBF — Virtual Battlefield

VBF is the digital implementation environment for **The Box**. It is intended to run one or more authoritative battlefield instances on a local or networked Battlefield Host while allowing browser clients, referees, and players to interact with the same simulation at different scales and with different information access.

The first full architectural reference world is **Virginia Tidewater / Hampton Roads, 1861–1862**, supported by a deliberately artificial **Chaos Camp** verification environment. Tidewater supplies historically grounded Actors, Assets, organizations, logistics, industry, information networks, ordinary civilian behavior, environmental state, naval operations, combat, casualties, capture, and lifecycle change. Chaos Camp exists to force rare, awkward, contradictory, or invalid combinations that history does not conveniently provide as exact regression fixtures.

The development target is not merely a rules engine capable of resolving individual Actions. A mature VBF Actor should be capable of accepting an Objective or Task and then using its own knowledge, qualifications, authority, procedures, Prescripts, personality traits, available resources, and scripted behavior to alter the world until the Task is completed, superseded, abandoned, or genuinely blocked.

---

## Project status

**Current status: Layer 0 foundation under active development.**

The Rust workspace currently contains:

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

The current implementation already provides or has begun to provide:

- typed permanent UIDs;
- stable human-readable Keys;
- human-facing Display Names;
- integer simulation time;
- strict event sequence numbers;
- monotonic state revisions;
- typed distance, speed, and angle values;
- versioned component schemas;
- Definitions and instantiated Entities;
- Actor, Asset, Process, Condition, Objective, and Task entity classes;
- single-parent Definition inheritance;
- first-class versioned Relationships;
- participant-role, exclusivity, and capacity validation;
- continuous world coordinates;
- host-relative positions;
- local and world pose;
- definition-level spatial anchors;
- linear and angular velocity;
- versioned semantic Event types;
- Event participant roles;
- Event cause and correlation links;
- primitive State Mutations, including spatial mutations;
- atomic candidate-state transactions;
- strict event-sequence commit;
- non-regressing simulation time;
- Actor-held Information Records with provenance scaffolding;
- an in-memory Store suitable for architectural development.

The current M8 regression fixtures remain useful, but Virginia Tidewater and Chaos Camp are the intended environments for driving the architecture from the present Layer 0 substrate to full VBF capability.

---

## Foundational design rule

VBF separates at least five forms of data:

1. **Human-authored definitions** — readable project, rules, map, Actor, Asset, Procedure, and scenario/reference-world source.
2. **Authoritative instance state** — what is actually true in one running battlefield instance.
3. **Semantic Events and State Mutations** — why authoritative state changed and exactly what changed.
4. **Derived results** — consequences calculated from authoritative state, never silently stored as truth merely because they are useful or expensive to calculate.
5. **Epistemic state** — what a particular Actor, organization, record system, or player believes, expects, remembers, reports, assumes, or has been told.

This separation is foundational.

---

## Core architectural invariants

- **Reality is not knowledge.**
- **Belief is not required to be true.**
- **Authoritative-world validity and epistemic validity are different contracts.**
- **Viewpoint is not control.**
- **Control assignment is not command authority.**
- **Ownership is not custody.**
- **Custody is not operational control.**
- **Assignment is not physical carriage.**
- **A planned or expected object is not an existing Asset.**
- **A message can be validly delivered even when its content, addressee assumptions, or authority are stale.**
- **The battlefield exists in continuous space.**
- **Hexes are optional spatial/rules overlays, not the engine coordinate system.**
- **Derived values are not authoritative state.**
- **Players never receive hidden truth merely so the client can conceal it.**
- **All accepted authoritative state changes are atomic and historically attributable.**
- **Old instances retain the rules/content versions under which they were created.**
- **Human-readable authored data remains distinct from runtime persistence.**
- **The same engine supports solo, cooperative, opposed, command-chain, referee, and sandbox play by configuration rather than separate game modes.**
- **Personality alters plan selection and accepted risk; it does not grant impossible Actions or nonexistent qualifications.**
- **Actors should attempt to change mutable prerequisites when they possess a plausible means to do so.**
- **Task execution is based on Actor-accessible belief, while Action legality and physical consequences resolve against authoritative Reality.**

---

## Reality invariants versus belief invariants

Authoritative state must obey structural and physical invariants. Actor belief must not be forced to do so.

Examples of valid epistemic states include:

- Agent Green Hat believes they are in contact with Agent Blue Hat even though Blue Hat never existed and is only a cover identity for Agent Orange Hat.
- Bert the Builder believes he is qualified to operate a forklift even though authoritative Personnel state contains no such qualification.
- Boss Redd believes two workers can shovel coal simultaneously in a compartment whose authoritative geometry and work capacity only permit one.
- Major Bailer expects today's intelligence report to arrive even though no report was ever written and the collector is missing.
- A worker believes a crate is safe for 500 pounds of apples while its authoritative structural capacity is only 400 pounds.
- Colonel Jones continues to follow General George's last orders after George has been killed and sends a situation report addressed to him. The report remains a real Information Asset and may validly arrive at George's headquarters even though the intended officer is deceased.

Accordingly, the epistemic system must permit:

- false claims;
- stale claims;
- contradictory claims;
- uncertain claims;
- claims about nonexistent or hypothetical referents;
- aliases and cover identities;
- expected future Assets or reports that never materialize;
- incorrect assumptions about capability, capacity, qualification, authority, geometry, or availability;
- continued reliance on orders whose issuer is no longer alive, present, or in command.

Epistemic records still require their own integrity. They need a valid holder or record context, provenance, timestamps where applicable, and a well-formed referent. They do **not** require the claimed fact to correspond to authoritative Reality.

The information architecture therefore needs belief-only or unresolved referents in addition to authoritative `EntityUid` references. A false belief about “Agent Blue Hat” must not force the compiler to instantiate a fake authoritative Actor simply to make the belief record structurally valid.

Likewise, an expected report should be representable as an expectation or claimed future Information Asset without causing the report itself to exist in Reality.

The mature resolution loop is therefore:

```text
Actor belief
    ↓
Task/plan selection
    ↓
Attempted Action
    ↓
Authoritative Reality validation
    ↓
Event / consequence
    ↓
Observation / report / surprise
    ↓
Updated belief
```

A belief can be wrong. An attempted Action can fail because Reality differs from belief. That failure can itself become information.

---

## Engine layers

### Layer 0 — Canonical substrate

Defines what data can exist and how it is identified, related, validated, versioned, sourced, stored, and reconstructed.

### Layer 1 — Authoritative state

Owns one battlefield instance's live Reality and validated State Mutations.

### Layer 2 — Derived state

Computes read-only consequences such as:

- composed world pose;
- LOS;
- route eligibility;
- control;
- observation geometry;
- protection;
- readiness;
- accessible stock;
- effective capacity;
- current support availability;
- water depth under hull;
- spatial access and interfaces;
- acoustic propagation conditions.

### Layer 2.5 — Event/time/reaction kernel

Schedules and resolves time-bearing Events, reactions, continuous Actions, Processes, pauses, simultaneous declarations, and Administrative Cycles.

### Layer 3 — Action resolution

Validates individual Action declarations and converts valid attempts into semantic Events plus atomic State Mutations.

### Layer 3.5 — Task and Procedure execution

Turns higher-level Tasks into practical sequences of enabling Actions, including Actions that change currently-unsatisfied prerequisites.

### Layer 4 — Actor recommendation and goal pursuit

Selects and ranks Tasks and plans using only the Actor's available information, authority, procedures, qualifications, traits, Prescripts, objectives, and available Actions. Recommendations never directly mutate authoritative state.

---

## Mature Actor behavior target

A principal maturity target for VBF is:

> A player can declare an Objective or Task for an Actor, and the Actor can use scripted procedures, available Actions, authority, resources, knowledge, personality traits, and current circumstances to complete it without requiring the player to micromanage obvious intermediate steps.

For example, a player should be able to tell Franklin Buchanan:

> Take command of CSS *Virginia*.

If Buchanan begins on the dockside, the engine should not simply reject `AssumeCommand` because he is not aboard. Nor should it select a physically shorter but behaviorally absurd straight-line swim when a gangway is available.

A plausible execution chain is:

```text
TakeCommand(Virginia)
    ↓
requires valid command location
    ↓
requires Buchanan aboard Virginia
    ↓
requires accessible boarding method
    ↓
walk to gangway
    ↓
board
    ↓
move to command location
    ↓
assume command
```

Every intermediate movement and state change remains real and interruptible.

If the usual boarding method is unavailable, the Actor should ask whether the prerequisite can be changed.

If *Virginia* is anchored six feet from shore and a suitable ten-foot plank is lying nearby, an Actor may be able to establish a temporary boarding interface.

If a required ferry is rowing away, an Actor with suitable command authority may order it back. An Actor without command authority may still hail or request it. A Risk-Tolerant or Reckless Actor may consider jumping or swimming sooner than a cautious Actor when ordinary options fail.

The architecture should therefore distinguish:

- physical possibility;
- ordinary procedural method;
- available affordance;
- authority over other Actors;
- known versus unknown resources;
- estimated risk;
- urgency;
- personality-driven plan ranking.

Personality does not override physics. Recklessness does not teach a non-swimmer to swim. It changes which otherwise-possible plans the Actor is willing to consider.

---

## Spatial direction

VBF uses continuous authoritative space.

A fictional battlefield such as Vaux may use a local Cartesian coordinate frame. Virginia Tidewater should use a georeferenced local frame suitable for accurate rivers, shorelines, installations, vessel movement, water depth, roads, and fixed works.

Actors and Assets may occupy:

- direct world positions;
- host-relative positions;
- station-relative positions;
- definition-level spatial anchors.

Hex membership, sectors, buildings, rail blocks, rooms, water areas, and similar structures are derived or explicit spatial-region relationships rather than the engine coordinate system.

The composed-space model must support chains such as:

```text
Buchanan
→ pilot-house station
→ Virginia
→ Hampton Roads world position
```

and:

```text
gun muzzle
→ gun mount
→ Monitor
→ world position
```

### Spatial interfaces

The spatial model must also represent meaningful access interfaces, including:

- gangways;
- ladders;
- hatches;
- doors;
- gates;
- docks;
- ferry landings;
- bridges;
- embarkation points;
- gunports;
- temporary bridging arrangements.

Path selection must not reduce ordinary behavior to shortest Euclidean distance. Procedures and available interfaces determine normal access; exceptional Actions such as jumping or swimming are considered when appropriate to the Actor and circumstances.

---

## Information direction

The server should eventually project an Actor-local worldview:

```text
Reality
→ geometry / signatures
→ observation
→ physical records / reports / tracks
→ Actor-accessible information
→ Actor belief
→ player view / Actor planning
```

Information records must support different precision and confidence for different claims.

An Actor may know that a crossroads exists while not knowing the exact reverse-slope dead ground around it. An Actor may believe a report exists when it does not. An Actor may receive two apparently independent reports that actually share one original source.

Planning tools must operate on the Actor's believed state, never hidden authoritative truth.

Physical Information Assets and Actor belief should remain separate. A written order, letter, map, courier packet, or intelligence report can exist physically even when unread, misaddressed, stale, false, or addressed to a deceased officer.

---

## Server direction

Terminology:

- **Battlefield Host** — machine-level supervisor/catalog capable of storing and starting multiple instances.
- **Battlefield Server** — one booted battlefield/reference-world instance with its own state, clock, event history, clients, permissions, and persistence.
- **Battlefield Instance** — the in-memory/runtime object owned by a Battlefield Server.

No global battlefield state should exist. Every running world owns its state through a `BattlefieldInstance`.

Browser clients are the primary intended user interface. The server remains authoritative.

Clients should receive Actor/player-appropriate projections rather than unrestricted hidden `WorldState` data.

---

# Development roadmap

The following roadmap takes the current Rust infrastructure to full support for the Virginia Tidewater / Chaos Camp reference setup.

The numbering is architectural rather than a promise of public semantic-version releases.

---

## VBF 0.1 — Canonical Layer-0 baseline

Stabilize the current September 2026 code before adding another conceptual layer.

Requirements:

- update architecture documentation to match current code;
- validate all current crates with formatting, compiler, Clippy, and tests;
- add serialization round-trip tests for new spatial and Event structures;
- freeze representative serialized compatibility fixtures;
- confirm crate dependency direction remains clean.

Acceptance:

- current workspace compiles and tests cleanly;
- serialized `EventTypeRef`, participant, pose, motion, hosted-position, anchor, Entity, Relationship, and Component fixtures round-trip without reinterpretation.

---

## VBF 0.2 — Invariant-valid authoritative Reality

The compiler and Store must enforce complete authoritative-world invariants.

Compilation and mutation must reject invalid Reality such as:

- nonexistent relationship participants;
- illegal participant classes;
- invalid component payloads;
- nonexistent coordinate frames;
- impossible exclusive station double occupancy;
- host cycles;
- nonexistent hosted locations;
- invalid relationship capacity;
- invalid package dependencies;
- dangling references created by deletion.

State mutation should follow:

```text
current state
→ candidate state
→ complete authoritative validation
→ atomic commit or rejection
```

Ended Relationships should remain historically recoverable rather than disappearing merely because they are inactive.

Critical rule:

> These authoritative-world validators do not force Actor beliefs to agree with Reality.

---

## VBF 0.3 — Composed space, motion, and interfaces

Turn the current spatial primitives into an authoritative geometry system.

Add:

- recursive host-relative transform composition;
- anchor/station resolution;
- host motion inheritance;
- cycle rejection;
- bounding geometry;
- containment and occupancy;
- distance, bearing, and elevation queries;
- network-to-world geometry;
- access interfaces;
- collision and clearance query contracts.

Acceptance:

Moving and rotating *Virginia* automatically changes the derived world pose of its crew stations, guns, carried stores, observers, and acoustic source anchors without individually mutating each hosted subject.

Chaos Camp should include deliberately deep nested-host transforms and invalid host cycles.

---

## VBF 0.4 — Event-sourced reference world and Epoch reconstruction

Make semantic Events the canonical history mechanism.

Add:

- Event-type registry;
- participant-role validation;
- typed payload validation;
- replay from baseline;
- replay to Event sequence;
- replay to simulation time;
- deterministic state hashing;
- snapshots/checkpoints;
- branchable hypothetical histories;
- historical Epoch definitions.

Virginia Tidewater should ultimately support a persistent chain such as:

```text
Merrimack intact
→ Gosport destruction
→ wreck
→ salvage
→ Virginia reconstruction
→ fitting-out
→ 8 March sortie
→ post-battle states
→ 9 March dawn
→ later readiness/repair
→ Norfolk evacuation
→ Virginia destruction
```

Epochs should ordinarily be reconstructed from shared history rather than maintained as unrelated scenario files.

---

## VBF 0.5 — `BattlefieldInstance` and clock separation

Add a runtime composition crate owning one running battlefield instance.

A `BattlefieldInstance` should own:

- package/version lock;
- compiled definitions;
- authoritative `WorldState`;
- Event history;
- simulation clock;
- scheduler;
- validator registry;
- derived-query registry;
- Action registry;
- output subscriptions.

### Simulation time versus render time

VBF must distinguish authoritative simulation time from presentation/render time.

When simulation is paused:

- Actors do not move;
- Actions and Processes do not advance;
- projectiles do not progress;
- wind state does not continue gusting;
- engines do not consume additional authoritative fuel;
- no new simulation Events occur.

Presentation systems may continue rendering the frozen current state.

This distinction is foundational for Acoustics.

---

## VBF 0.6 — Actors, organizations, authority, and agency

Implement the universal Actor/organization kernel.

Support:

- Leader Actors;
- Follower Actors;
- Routine Actors;
- individual and Group resolution;
- military units;
- vessel crews;
- organized work crews;
- contractors;
- civilian populations;
- animal Routine Actors;
- organizations;
- membership;
- qualifications;
- Decision Rights;
- command relationships;
- uplinks/downlinks/crosslinks;
- ownership;
- custody;
- operational control;
- assignment;
- carriage;
- station occupancy.

These meanings must remain distinct.

A privately owned tug under military control is not automatically government-owned. A worker assigned to a transport is not necessarily physically aboard it.

---

## VBF 0.7 — Epistemic referents and Actor belief substrate

Deepen `vbf-information` before mature planning depends upon it.

Support:

- authoritative Entity referents;
- belief-only named or unresolved referents;
- aliases and cover identities;
- hypothetical/expected objects;
- claims about future Events or reports;
- false self-qualification beliefs;
- stale authority beliefs;
- conflicting reports;
- common-source provenance;
- confidence and precision;
- acquisition time and observation/reference time;
- correction and contradiction.

A belief-only referent must not instantiate an authoritative Entity merely to satisfy a foreign-key requirement.

Acceptance examples:

- Agent Blue Hat can exist in Green Hat's belief without existing in Reality.
- Major Bailer can expect a report that was never created.
- Colonel Jones can still believe General George is alive and authoritative after George's death.
- a physical sitrep addressed to George can be created and delivered to headquarters despite George being dead.

---

## VBF 0.8 — Objectives, Tasks, and completion conditions

Make goal state explicit.

Separate:

- Objective;
- Task;
- completion condition;
- abandonment/failure conditions;
- assigned Actor or organization;
- priority and urgency.

A Task expresses desired state, not a hard-coded movement script.

Example:

```text
Task: Take command of CSS Virginia
Actor: Franklin Buchanan
```

The Task Executor determines how to make the completion condition true.

---

## VBF 0.9 — Primitive Actions and state-changing affordances

Add a generic Action system.

Start with noncombat Actions:

- Move;
- Board;
- Disembark;
- Open/Close;
- Carry;
- Lay/Place;
- Attach/Detach;
- Transfer;
- Assume Station;
- Relinquish Station;
- Issue Order;
- Send Message;
- Direct Routine Actor;
- Begin Work;
- Load;
- Unload.

Action resolution follows:

```text
Action declaration
→ precondition validation
→ resolution
→ semantic Event
→ proposed State Mutations
→ authoritative invariant validation
→ commit/reject
```

### Affordance queries

The planner needs to ask:

> What available Actions could make condition X true?

An unsatisfied prerequisite is not automatically a terminal failure.

If Buchanan needs an accessible boarding interface, possible state-changing Actions may include:

- use existing gangway;
- move a gangway;
- lay a suitable plank;
- order vessel/ferry alongside;
- request vessel/ferry alongside;
- use a boat;
- jump;
- swim.

---

## VBF 0.10 — Established Procedures and practical execution

Add a Task/Procedure Executor between high-level goals and primitive Actions.

Procedures constrain and rank ordinary solutions without replacing Reality.

Example:

```text
Board Moored Vessel

Preferred:
1. designated gangway or boarding ladder
2. other established safe interface
3. suitable temporary bridging arrangement
4. boat transfer

Exceptional:
5. jump
6. swim
```

A shortest-path algorithm must not treat swimming as the normal route merely because the straight-line distance is smaller.

Procedures operate on Actor Knowledge. If the Actor believes the gangway exists when it does not, the Actor may move toward it, discover the problem, update belief, and replan.

---

## VBF 0.11 — Traits, Prescripts, risk, and plan ranking

Make personality traits operationally meaningful through plan ranking.

Candidate-plan evaluation may consider:

- physical feasibility;
- believed feasibility;
- estimated completion time;
- effort;
- risk;
- uncertainty;
- procedural conformity;
- resource consumption;
- disruption to others;
- command implications;
- reversibility;
- urgency;
- Prescripts;
- qualifications;
- stable personality traits;
- current Personnel state.

A Risk-Tolerant or Reckless Actor may accept dangerous alternatives earlier than a cautious Actor.

Traits do not invent capability.

---

## VBF 0.12 — Replanning and persistent goal pursuit

Actors should not construct one plan and blindly execute it.

Goal pursuit should loop:

```text
evaluate Task
→ select next useful Action
→ act
→ Reality changes
→ observations/beliefs may change
→ re-evaluate Task
```

Replanning triggers include:

- lost prerequisite;
- inaccessible expected interface;
- target movement;
- Action failure;
- new hazard;
- new order;
- injury;
- resource depletion;
- newly discovered affordance;
- Prescript activation;
- changed authority relationship.

Chaos Camp should heavily test moving targets, disappeared resources, false assumptions, and alternate means.

---

## VBF 0.13 — Logistics, material conservation, carriage, and custody

Implement real located resources rather than abstract counters wherever physical location matters.

Support:

- commodity/stock lots;
- quantity;
- location;
- storage;
- accessibility;
- condition;
- ownership/custody where relevant;
- carrying capacity;
- loading/unloading;
- partial transfer;
- consumption;
- loss/destruction;
- transport assignment;
- actual carrage;
- route eligibility;
- transport cycles.

Virginia Tidewater acceptance cases include:

- coal and provisions aboard *Virginia*;
- Richmond iron delivery;
- Burnside's heterogeneous transports;
- horses and supplies aboard named vessels;
- transfer of only part of a formation;
- cargo lost with a transport.

Chaos Camp must reject impossible duplication while permitting false records about where cargo is believed to be.

---

## VBF 0.14 — Weapon load-state architecture and improvised loads

A weapon fires from its current physical load state, not from a nearby ammunition counter.

Core invariants:

- a cannon cannot fire unless it contains a fireable load;
- a cannon cannot be loaded without the required material actually being available and transferable;
- firing consumes/releases the actual loaded material;
- removing all spare ammunition from a ship does not unload a cannon that is already loaded.

For flexible historical weapons, distinguish:

- qualified/calibrated service load;
- physically possible improvised load.

A muzzle-loader may physically accept a nonstandard combination such as a reduced charge and improvised payload. That does not grant standard ballistic performance, safety, or qualification.

Combat derives effects from the actual load state and the strength of available calibration.

---

## VBF 0.15 — Time, continuous Processes, and reactions

Implement the full Layer 2.5 kernel.

Support:

- future Event queue;
- deterministic ordering;
- exact simulation time;
- simultaneous Event batches;
- continuous Action intervals;
- interruptible Actions;
- Processes;
- periodic evaluation;
- Administrative Cycles;
- Reaction Opportunities;
- reaction eligibility;
- simultaneous reactions;
- pause/advance controls.

Do not make a universal turn the ontology of time.

Tidewater must support milliseconds/seconds for impacts, minutes for maneuver and gunnery, hours for voyage/fatigue, days for repair, and months for construction/lifecycle change on the same authoritative clock.

---

## VBF 0.16 — Derived state and environment

Add a read-only derived-state system plus environmental support.

Derived queries include:

- composed pose;
- LOS;
- route eligibility;
- region membership;
- accessible stock;
- carrying capacity;
- readiness;
- support availability;
- local control;
- gun arcs;
- water depth under hull;
- clearance;
- acoustic propagation inputs.

Environmental state includes enough Weather, Hydrological, Terrain, and Hazard support for:

- wind;
- fog;
- visibility;
- tide/current;
- water depth;
- sea state;
- road condition;
- mud;
- fire;
- smoke;
- flooding;
- heat;
- ventilation-related conditions.

Derived results are never silently promoted to authoritative state simply because they are useful.

---

## VBF 0.17 — Routine Actor behavior

Routine Actors use bounded ordinary behavior rather than tactical omniscience.

Support:

- intrinsic routines;
- current needs;
- available Actions;
- recognized categories of legitimate authority;
- environmental triggers;
- observed information;
- interruption and redirection rules.

Historical acceptance case:

```text
ordinary work/routine
→ unusual Virginia signatures
→ information spreads
→ civilians seek viewpoints
→ local movement and work patterns change
```

Organized work crews remain Followers when they possess formal roles, reporting duties, or continuing retaskability.

---

## VBF 0.18 — Command, communications, and delegated Tasks

Represent distributed agency.

Support:

- Decision Rights;
- Orders;
- requests;
- cross-command coordination;
- temporary task authority;
- force transfer;
- message carriers;
- communication latency;
- lost/delayed messages;
- acknowledgement;
- stale orders;
- delegated/subordinate Tasks.

Important pattern:

> An Actor may satisfy one of its own Task prerequisites by causing another Actor to pursue a subordinate Task.

If Buchanan needs a ferry returned and has authority over its crew, he may issue the order instead of treating the ferry's current position as immutable.

If he lacks command authority, he may still be able to request or hail them.

The physical message/order remains valid as an Information Asset even if the intended recipient has died or the underlying authority has changed.

---

## VBF 0.20 — Goal-Directed Actor Kernel maturity milestone

This is a major architectural maturity gate.

Acceptance scenario:

A scenario begins with Buchanan somewhere dockside and *Virginia* nearby. The player declares only:

> Take command of CSS *Virginia*.

The engine must handle variants including:

| Situation | Expected architecture-level behavior |
|---|---|
| Gangway open | walk, board, reach command location, take command |
| Gangway moved | seek another ordinary access method |
| Six-foot gap + suitable plank | plausibly establish/use temporary access |
| Ferry departing + valid authority | order ferry returned |
| Ferry departing + no authority | hail/request or seek another method |
| Ordinary access unavailable | consider exceptional alternatives |
| Risk-Tolerant Actor | accepts higher-risk alternatives earlier |
| Reckless Actor + urgent Task | may jump/swim where others would wait |
| Actor cannot swim | trait does not create capability |
| Useful plank exists but Actor does not know it | cannot select until discovered |
| Actor falsely believes gangway exists | attempt, discover error, update, replan |
| Ship moves during approach | stale plan is abandoned/replanned |
| Superior order arrives | Task may be superseded |

At this milestone, the architecture should be capable of player-assigned goals driving autonomous, state-changing Actor behavior without scenario-specific scripts for every intermediate step.

---

## VBF 0.21 — Tidewater Naval systems

Implement vessel-specific operating mechanics.

Support:

- propulsion;
- steering;
- speed through water;
- environmental displacement;
- draft;
- trim;
- grounding;
- towing;
- anchoring;
- machinery reliability;
- compartment state;
- ventilation/openings;
- flooding;
- freeboard/downflooding;
- fuel consumption;
- vessel stations.

First major vertical slice should be *Virginia*'s movement down the Elizabeth River before combat:

```text
ready ship
→ depart
→ maneuver
→ consume fuel
→ hosted world geometry updates
→ signatures generated
→ observers react
→ reports propagate
```

---

## VBF 0.22 — Hampton Roads Combat and damage

Implement:

- weapon Assets;
- mounts and arcs;
- crew/station prerequisites;
- physical weapon load state;
- load/fire/reload Actions;
- projectile flight;
- impact geometry;
- protection path;
- local damage;
- subsystem damage;
- structural damage;
- personnel casualty;
- fire/flooding initiation;
- damage-control reactions.

Significant projectile chains should remain causally linked:

```text
weapon fire
→ projectile flight
→ passage event where applicable
→ impact
→ damage
→ casualty/capability consequences
```

---

## VBF 0.23 — Personnel consequence, casualty transport, capture, and reconstitution

Support:

- wounds/incapacitation;
- duty availability;
- qualified substitution;
- casualty movement;
- treatment location;
- casualty transport;
- surrender;
- capture;
- custody;
- parole;
- exchange;
- return to duty;
- unit reconstitution.

Roanoke is a key historical test because an organization may continue to exist while only part of it is captured, paroled, missing, or still operational.

---

## VBF 0.24 — Specialized Tidewater capability packages

Finish unusual but architecturally valuable cases:

- tethered observation balloon;
- balloon-support vessel configuration;
- mine/torpedo equipment;
- specialized carried equipment;
- capture and technical exploitation;
- *Teaser* configuration lineage;
- civilian → military → captured → later civilian service transitions.

These should require new domain content, not new universal ontology.

---

## VBF 0.25 — Chaos Camp full conformance suite

Chaos Camp is a deliberately incoherent but locally valid reference environment.

Design rule:

> Every local state must obey Reality invariants. The combined situation does not need to make narrative sense.

Chaos Camp exists to test exact edge conditions and invalid mutations.

Permanent categories should include:

### Reality conservation

- same physical crate transferred twice simultaneously;
- impossible over-capacity loading;
- exclusive station double occupancy;
- hosted entity assigned to mutually exclusive hosts;
- deletion of host while occupants remain unresolved;
- stock destroyed only where it physically exists.

### Belief/Reality disagreement

- nonexistent Agent Blue Hat believed to exist;
- Actor falsely believes self qualified;
- supervisor believes impossible workforce capacity;
- expected report does not exist;
- crate believed capable of 500 lb but fails above authoritative 400 lb capacity;
- order chain continues after superior's death;
- sitrep addressed to deceased officer still arrives at headquarters;
- two apparently independent reports share one hidden source.

### Goal pursuit and affordances

- expected gangway missing;
- nearby plank can create access;
- target ferry moving away;
- Actor can order another Actor to change prerequisite state;
- Actor lacks authority and must request instead;
- reckless versus cautious plan ranking;
- useful resource exists but is unknown;
- stale belief causes failed Action followed by replanning.

### Logistics and weapon state

- partial cargo transfer;
- inaccessible stock incorrectly believed available;
- cannon loaded before all spare ammunition is lost and still able to fire;
- loading fails without actual required material;
- physically possible improvised load accepted but marked unqualified/uncalibrated.

### Temporal and Event integrity

- duplicate Event rejected;
- sequence gap rejected;
- simulation-time regression rejected;
- replay reconstructs exact previous state;
- same derived query changes when dependency changes without authoritative cache mutation.

### Acoustics

- pause during idling engine;
- pause during fixed current wind state;
- pause during steam venting;
- pause during decaying cannon report/reverb;
- pause during projectile flight;
- no ambient battle gunfire appears without Events;
- persistent emitter survives pause without advancing authoritative world state.

Every future architectural bug should be minimized into a permanent Chaos Camp regression whenever possible.

---

## VBF 0.26 — Persistence and Battlefield Server

After simulation contracts stabilize, add persistent Store implementation and network/runtime services.

Persistence belongs behind `vbf-store` and should support:

- package lock;
- baseline state;
- Event log;
- snapshots;
- branches;
- instance metadata.

The Battlefield Server then exposes:

- instance lifecycle;
- player/referee connections;
- Actor-local state projections;
- Task/Action declaration APIs;
- event subscriptions;
- audio/output subscriptions;
- permissions.

Clients never receive hidden Reality merely so front-end code can conceal it.

---

# Acoustics integration roadmap

VBF Acoustics is a parallel downstream rendering subsystem. SuperCollider renders sound from VBF state and Events; it does not own simulation truth.

Dependency direction:

```text
VBF authoritative state + semantic Events + derived geometry/environment
                              ↓
                      Acoustics bridge
                              ↓
                       SuperCollider
```

SuperCollider failure must not stop the battlefield. Turning audio off must not alter simulation results. Changing a SynthDef must not invalidate saved world state.

## Two classes of acoustic source

### Continuous state sounds

Examples:

- idling engines;
- generators;
- machinery hum;
- steam venting;
- steady current wind;
- rain;
- flowing water;
- persistent fire.

These are renderings of current authoritative state.

For an M4A2 whose GM 6046 is currently idling at approximately 350 RPM, the audio engine maintains a persistent emitter representing that state.

### Event sounds

Examples:

- gunfire;
- projectile passage;
- impact;
- ricochet;
- explosion;
- collision;
- structural break;
- discrete shouted or signaled events where modeled.

These exist only because committed semantic Events occurred.

There is no generic ambient battle gunfire that continues independently of the event stream.

## Pause semantics

When the simulation pauses, the **world stops evolving but sound rendering does not necessarily stop**.

Frozen during pause:

- authoritative simulation time;
- Actor motion;
- Process advancement;
- projectile motion;
- fuel consumption;
- changing RPM state;
- weather evolution;
- new simulation Events.

Continues during pause:

- persistent audio emitters representing the frozen state;
- renderer-internal waveform evolution;
- harmless cycle-to-cycle/timbral variation;
- reverb and echo tails already emitted;
- already-established acoustic render consequences.

Example: if Weather state is currently 10 mph wind inside an 8–15 mph gust regime and the player pauses, Weather does not continue advancing through the gust cycle. Acoustics continues rendering the character of approximately 10 mph wind with minor renderer-local noise variation.

Example: if a GM 6046 is frozen at approximately 350 RPM, the audible engine continues idling. Those additional rendered cycles do not represent extra authoritative crankshaft revolutions, fuel consumption, wear, or elapsed simulation time.

If a gun fired immediately before pause, the already-emitted report and reverberation may finish naturally. If a projectile has not yet struck when the simulation pauses, the projectile remains frozen and no impact Event or impact sound occurs until simulation time resumes and the impact actually happens.

Rule:

> Acoustic consequences already emitted may finish; future world causes do not occur while simulation time is paused.

## Rust-side acoustic truth versus SuperCollider rendering

Rust owns simulation-relevant acoustic state:

- emission existed;
- source pose and anchor;
- emission time;
- broad signature characteristics;
- propagation delay;
- attenuation/obstruction inputs;
- whether an Actor can detect the sound;
- resulting Observation/Information.

SuperCollider owns human-facing rendering:

- waveform;
- timbre;
- spatial audio;
- natural microvariation;
- reverb;
- source synthesis.

An NPC can hear and react to a gunshot even if no SuperCollider process is attached.

## Acoustics development track

| Main VBF stage | Acoustics integration |
|---|---|
| 0.1–0.2 | freeze semantic source/output contract |
| 0.3 | source/listener spatial anchors |
| 0.4 | deterministic Event replay into renderer |
| 0.5 | separate simulation and render clocks |
| 0.6–0.12 | persistent emitter lifecycle and Actor-linked listener state |
| 0.15 | propagation timing against simulation clock |
| 0.16 | weather/terrain/environmental acoustic inputs |
| 0.17 | audible signatures create Actor Observations |
| 0.21 | engines, boilers, machinery, water interaction |
| 0.22 | muzzle, projectile, impact, fragmentation, structural response |
| 0.25 | pause/resume and acoustic Chaos Camp regressions |

The existing SuperCollider organization into modular source families should remain compatible with this architecture. Muzzle, projectile, and impact sources should remain independent so one VBF causal chain can produce separate acoustic consequences.

---

# Virginia Tidewater reference-world development sequence

The implementation order inside the historical world should introduce one architectural family at a time while retaining everything previously built.

Recommended vertical slices:

1. **Static CSS *Virginia* at Gosport** — Definitions, Entities, hosted positions, crew stations, guns, stores, relationships, compile/reload.
2. **Merrimack salvage** — organizations, contractors, authority, work, support, custody, lifecycle.
3. **Burnside transport system** — carrying, capacity, partial formations, assignment versus carriage, route/environment interaction.
4. **8 March information propagation** — signatures, civilians, Routine Actors, records, rumors, lookouts, knowledge.
5. **Monitor transit** — time, fatigue, weather, towing, operating envelope, progressive failure.
6. **Virginia movement down Elizabeth River** — Naval movement before combat, hosted geometry, continuous acoustic sources.
7. **8–9 March Hampton Roads combat** — reaction, loading, firing, projectile, impact, damage, casualty, acoustic Event rendering.
8. **Roanoke surrender/parole** — capture, custody, partial formation state, legal/personnel consequences.
9. **Teaser** — configuration lineage, special equipment, tethered balloon, capture, exploitation.
10. **Chaos Camp** — full conformance and pathological integration testing.

The battle itself should therefore run on an engine already proven to handle workmen, contractors, stores, civilians, crowds, messages, false beliefs, transports, authority, weather, fatigue, maintenance, and persistent Asset state.

---

# Architectural completion criterion

Virginia Tidewater and Chaos Camp are intended to **generate the universal architecture**, not merely consume it.

A later environment may require new domain rules, new Assets, new Procedures, new calibration, or new content. It should not require redefining what an Actor, Asset, Relationship, Task, Event, Information Record, Position, resource transfer, belief, or State Mutation fundamentally is.

A mature VBF should be able to accept a high-level player goal, let an Actor pursue it through plausible state-changing behavior, permit that Actor to be wrong about the world, expose the consequences of that error through Reality, and keep all accepted world changes deterministic, attributable, replayable, and observable only to Actors who have a legitimate information path to them.

That is the standard by which the Virginia Tidewater / Chaos Camp architecture should be judged.

---

## Development and validation model

The project assumes that substantial implementation work may be LLM-assisted. Human review therefore occurs mainly at the specification and behavior level rather than by manually auditing every line of Rust.

Every subsystem should be accepted through:

1. explicit behavioral specification;
2. implementation;
3. compiler/type checks;
4. formatting and lint checks;
5. unit tests;
6. serialization tests;
7. property/fuzz tests where valuable;
8. integration tests;
9. permanent Virginia Tidewater historical regression fixtures;
10. permanent Chaos Camp conformance fixtures.

An LLM-written subsystem is acceptable. An **LLM-unverified** subsystem is not.

Required Rust validation command set:

```bash
cargo fmt --check
cargo check --workspace
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
```

No later VBF subsystem should need to inspect or write SQLite directly. Persistence remains behind Store interfaces.
