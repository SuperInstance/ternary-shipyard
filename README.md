# ternary-shipyard

Construction and assembly of complex agent systems from declarative blueprints.

## Why This Exists

Fleet agents need to be built, tested, deployed, and eventually decommissioned — and the components should be recyclable. Rather than ad-hoc agent construction scattered across code, ternary-shipyard provides a declarative blueprint system where agents are specified by what they need, assembled on an assembly line, inspected by quality assurance, and recycled through a scrap yard when their lifecycle ends.

## Core Concepts

- **Blueprint**: A declarative specification of what components an agent needs and what role requirements must be met. Think of it as a build manifest.
- **Component**: A named part with a ternary role tag — Core (required for operation), Enhancement (improves performance), or Hardening (adversarial defense).
- **Assembly Line**: Builds agents from blueprints, rejecting any that don't meet role requirements.
- **Quality Assurance**: Inspects assembled agents, checking for minimum viability (at least one Core component).
- **Scrap Yard**: Recycles decommissioned agent components back into reusable parts.
- **Ship Class**: Template library of pre-configured blueprints for common agent types (scout, worker, guard).

## Quick Start

```toml
[dependencies]
ternary-shipyard = "0.1"
```

```rust
use ternary_shipyard::*;

let mut yard = Shipyard::new();

// Build from a built-in template
let mut agent = yard.build_from_class("scout", "recon-1").unwrap();
assert!(agent.passed_qa);

// Decommission and recycle
let parts = yard.decommission(agent);
println!("Recycled {} components", parts);
```

## API Overview

| Type | Description |
|------|-------------|
| `Component` | Named part with a ternary role tag |
| `TernaryRole` | Core, Enhancement, or Hardening |
| `Blueprint` | Declarative agent specification with role requirements |
| `Agent` | Assembled agent built from a blueprint |
| `AssemblyLine` | Builds agents from blueprints |
| `QualityAssurance` | Inspects agents for minimum viability |
| `ScrapYard` | Recycles dead agent components |
| `ShipClass` | Template library of common blueprints |
| `Shipyard` | Top-level coordinator for the full lifecycle |

## How It Works

The build pipeline is linear: Blueprint → AssemblyLine → QualityAssurance → Agent. The `AssemblyLine` checks that all required roles in the blueprint are satisfied by at least one component, then constructs the agent. `QualityAssurance` performs a second check (at least one Core component present and agent is assembled). Failed QA doesn't destroy the agent — it's flagged for the caller to decide.

`ScrapYard` takes ownership of a decommissioned agent via `disassemble()`, storing components for later retrieval via `salvage()`. This enables component reuse across agent generations.

`ShipClass` ships with three defaults (scout, worker, guard) and accepts custom registrations. Instantiation creates a named copy of a template blueprint.

## Known Limitations

- No versioning of blueprints — once registered, a class can't be evolved.
- QA only checks for Core component presence; no pluggable validation rules beyond `inspect_with`.
- No concept of component dependencies (e.g., "memory requires processor").
- Scrap yard doesn't track component health or wear — recycled parts are assumed good.
- Not thread-safe; all operations are single-threaded.

## Use Cases

- **Game entity factories**: Spawn NPCs from templates, recycle dead NPC components into the loot pool.
- **IoT device provisioning**: Define device blueprints (sensor + comm + battery), assemble and validate before deployment.
- **CI pipeline stages**: Treat build stages as "components" — a blueprint specifies what stages are required, QA checks all ran.

## Ecosystem Context

Part of the SuperInstance ternary fleet. Agents built here are deployed into rooms managed by `ternary-navigator`. `ternary-observatory` monitors deployed agent health. `ternary-cargo` transports blueprints and components between rooms.

## License

MIT
