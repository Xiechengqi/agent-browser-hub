# Workflow Verification Plan

## Goal

Provide a release-quality validation matrix for the workflow-package architecture.

## Layers

### Registry

Validate:

- builtin workflow discovery
- external path override precedence
- external git override precedence
- `builtin-only`
- `prefer-external`
- `strict-external`
- fallback behavior when external packages are invalid

### Resolution

Validate:

- YAML command resolution
- workflow `pipeline` resolution
- workflow `script` YAML compatibility resolution
- workflow `native` dispatch resolution
- site-level override replacement behavior

### Runtime

Validate:

- pipeline parameter rendering
- output formatting across `json/yaml/table/csv/md`
- native dispatch result normalization
- workflow metadata default injection

### UI/API

Validate:

- `/api/commands` source metadata
- `/api/settings` workflow config roundtrip
- effective workflow source visibility in the UI
- settings save invalidates and refreshes command/source queries without reload

## Verification Phases

### Phase 1: Static Structure

- workflow validator passes
- frontend parse check passes
- builtin workflow coverage matches `scripts/*`

### Phase 2: Resolution Semantics

- builtin resolution precedence is stable
- external `path` override replaces builtin package at site granularity
- external `git` override replaces builtin package at site granularity
- invalid external packages respect `fallback_to_builtin`
- `strict-external` fails loudly when fallback is disabled

### Phase 3: Runtime Execution

- one representative `pipeline` workflow succeeds
- one representative `script` workflow succeeds
- one representative `native` workflow succeeds
- output formatting is checked for at least `json`, `table`, and `md`

### Phase 4: Operator Experience

- workflow diagnostics API does not trigger clone/fetch side effects
- workflow settings roundtrip preserves source JSON exactly
- fallback and source-kind badges are visible in all major UI surfaces
- invalid workflow config returns a user-visible server error

## Smoke Matrix

Priority sites:

1. `twitter`
2. `boss`
3. `reddit`
4. `bilibili`
5. `xueqiu`
6. `notion`
7. `wikipedia`
8. `hackernews`

For each site:

- at least one command resolves
- params render correctly
- source label is correct
- expected auth strategy is visible
- chosen execution entry (`pipeline`, `script`, or `native`) is recorded

Priority-site execution ownership:

- `twitter`: external override and auth-heavy workflow validation
- `boss`: auth-heavy navigation and anti-fragile selector review
- `reddit`: content extraction plus cookie-dependent execution
- `bilibili`: native execution path remains healthy
- `xueqiu`: cookie-dependent market content extraction
- `notion`: structured page/content extraction
- `wikipedia`: simple public script or pipeline baseline
- `hackernews`: public low-risk regression baseline

## Manifest Validation

Add a lightweight validator for:

- required package fields
- required command fields
- duplicate command names
- missing referenced assets
- unsupported entry kinds

Current tool:

- `python3 tools/validate_workflows.py`

The validator should report:

- site coverage between `scripts/*` and `workflows/*`
- manifest field errors
- `commands.include` drift
- missing referenced assets
- entry-type distribution
- priority-site presence

## Release Gate

Before declaring workflow migration stable:

- workflow coverage matches script-site coverage
- no new YAML-only commands are introduced
- priority smoke matrix passes
- external override fallback behavior is verified
- at least one workflow-native path remains green
- at least one workflow-script path remains green
- diagnostics and settings UI reflect source changes without reload

## Current Constraints

In the current environment:

- do not rely on `cargo check` or `cargo build`
- prefer validator, targeted formatting, frontend parse checks, and manual API/UI smoke
- runtime browser validation must be completed in a release-capable environment
