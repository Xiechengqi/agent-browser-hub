# Workflow Roadmap

## Status

The project has moved from ad hoc migration planning into workflow-package rollout:

- `workflows/*` now covers every site under `scripts/*`
- builtin workflow packages are the default metadata source
- site-level external overrides are supported
- `bilibili/feed` is the first real workflow-native path
- legacy YAML remains as an execution asset during the transition

## Ordered Next Steps

### 1. Governance

Lock the repository onto the new source-of-truth model:

- all new commands must be added under `workflows/*`
- `scripts/*` is now compatibility or implementation residue only
- direct additions to `scripts/*` without a matching workflow package entry should be treated as migration debt
- advanced sites should prefer `workflow` manifests first, then choose `pipeline`, `script`, or `native` as the execution entry

Deliverables:

- governance document
- README updates
- contributor rules for new command additions

### 2. Script Runtime

Define and implement the real `execution.script` backend:

- constrained runtime contract
- helper surface around `AgentBrowser`
- data and error normalization
- boundary between `script` and `native`

Deliverables:

- script runtime design spec
- first helper-backed workflow script example
- runtime execution path in hub

### 3. External Source Lifecycle

Move external workflow repos from “loadable” to “operable”:

- cache layout and invalidation
- update semantics
- `ref` pinning rules
- fallback and warning model
- version compatibility checks

Deliverables:

- lifecycle policy
- UI-visible source health fields
- clearer operator diagnostics

### 4. Workflow Center UI

Shift from command list UI to package-centric UI:

- site/package overview
- source badges and override state
- health and compatibility indicators
- workflow source management surface

Deliverables:

- workflow-center information architecture
- screen-by-screen UI plan
- API shape needed by the frontend

### 5. Verification Matrix

Make the migration defensible:

- registry precedence tests
- builtin/external/fallback tests
- workflow manifest validation
- smoke suite for priority sites

Deliverables:

- verification plan
- per-layer test matrix
- release gate checklist

### 6. OpenCLI Site Execution Program

Move from architecture readiness to site-by-site migration execution:

- define migration waves for priority sites
- decide which sites stay builtin and which justify external repos
- attach exit criteria to each wave
- track target runtime shape per site

Deliverables:

- site migration execution plan
- per-site decision records for priority sites
- release ownership for external override sites

## Migration End State

- `workflows/*` is the canonical command metadata layer
- `scripts/*` is execution residue only until fully replaced
- site-level external repos can override builtin packages safely
- `pipeline`, `script`, and `native` are all first-class workflow entry types
- UI and API expose effective source, health, and compatibility state
