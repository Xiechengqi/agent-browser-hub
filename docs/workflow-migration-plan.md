# Workflow Migration Plan

## Current State

- builtin workflow coverage now matches all sites under `scripts/*`
- workflow packages are the primary metadata layer
- legacy YAML remains as execution input during migration
- site-level external overrides are wired into registry resolution

## Completed

### Phase 1

- Add workflow package spec and resolver model
- Introduce builtin workflow discovery
- Keep `scripts/*` execution intact as a compatibility path

### Phase 2

- Convert existing `scripts/*` metadata into `workflows/*`
- Use workflow manifests as the primary command listing source
- Keep YAML pipeline files as implementation assets during migration

## Completed Migration Target

### Phase 3

- Migrate simple sites first
- Migrate standard sites with shared helpers
- Migrate advanced sites into native/script-backed workflow packages

Current final status:

- `bilibili/feed` is now a real workflow-native execution path
- all non-native workflow commands have been lifted to package-local `workflow-script`
- the command catalog is now `261 workflow-script + 1 workflow-native`
- all workflow commands now reference package-local assets under `workflows/<site>/*`
- all 44 sites now have an assigned migration wave in the full-site tracker

## Next

1. harden the real `execution.script` runtime beyond wrapper-based migration
2. complete release-environment runtime and UI smoke validation
3. harden external source lifecycle and fallback operations
4. continue native uplift only where wrappers are insufficient

## Site Priority

1. `twitter`
2. `boss`
3. `reddit`
4. `bilibili`
5. `xueqiu`
6. `notion`
7. `wikipedia`
8. `hackernews`

## End State

- `workflows/*` is the canonical source of command metadata
- `scripts/*` becomes implementation input or compatibility residue only
- external workflow repositories can override builtin site packages cleanly

## Execution Tracker

- [full-site-migration-tracker.md](full-site-migration-tracker.md)
