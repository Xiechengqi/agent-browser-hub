# OpenCLI Site Migration Execution Plan

## Goal

Turn the current workflow-package architecture into a concrete site-by-site migration program from legacy opencli-style YAML execution toward agent-browser-centered workflows.

This plan assumes:

- workflow metadata already lives in `workflows/*`
- YAML execution assets still exist under `scripts/*`
- the target end state is workflow-package first, with package-local `workflow-script` as the default and `workflow-native` reserved for justified deep integrations
- site-level external workflow repos remain optional overrides, not the default model

## Migration Principles

- migrate at site granularity, not command granularity, when changing package ownership
- keep builtin workflow packages as the default baseline for every site
- use external repos only when a site needs faster iteration, separate ownership, or materially different release cadence
- prefer `script` before `native` when the site mostly needs richer helper logic rather than deep platform integration
- keep public low-risk sites as the baseline regression set
- avoid cross-package asset references; workflow assets should resolve inside each site package

## Site Tiers

### Tier 1: Priority Execution Sites

These sites should move first because they exercise the most important migration risks:

1. `twitter`
2. `boss`
3. `reddit`
4. `bilibili`
5. `xueqiu`
6. `notion`
7. `wikipedia`
8. `hackernews`

Tier 1 intent:

- prove external override model on a real high-change site
- prove auth-heavy site migration
- prove one stable public baseline
- prove one `native` path remains maintainable

### Tier 2: Structured Public Sites

Examples:

- `wikipedia`
- `stackoverflow`
- `arxiv`
- `google`
- `medium`

Tier 2 intent:

- keep simple extraction flows as package-local `workflow-script` wrappers unless reusable helpers clearly reduce complexity
- harden output normalization and shared page/query helpers

### Tier 3: Auth And Interaction Heavy Sites

Examples:

- `twitter`
- `boss`
- `reddit`
- `notion`
- `v2ex`
- `douban`
- `xueqiu`

Tier 3 intent:

- decide which sites merit dedicated external repos
- establish durable auth/session helper boundaries
- reduce brittle selector logic inherited from legacy YAML definitions

## Wave Plan

### Wave 1: Public Baseline

Sites:

- `wikipedia`
- `hackernews`

Deliverables:

- one low-risk public site moved to cleaner workflow-script patterns
- one low-risk public site kept as a low-risk script regression anchor
- release checklist updated with stable smoke commands

Exit criteria:

- command metadata remains workflow-package first
- at least one command per site executes without central cross-package metadata dependence
- regression smoke is easy to run in release environments

### Wave 2: Native And Rich Runtime Proof

Sites:

- `bilibili`
- `reddit`

Deliverables:

- `bilibili` native path expanded or kept as the native reference site
- one auth/content-heavy site moved toward script-backed helpers
- clearer guidance for when to stop stretching YAML

Exit criteria:

- native/script boundary is documented by example
- at least one helper-heavy site no longer depends on raw YAML complexity alone

### Wave 3: External Override Proof

Sites:

- `twitter`
- `boss`

Deliverables:

- site-level external workflow package override validated end to end
- production-style pinned ref policy exercised
- override ownership and release process documented

Exit criteria:

- builtin fallback works
- strict-external behavior is understood
- a site can ship independently without breaking builtin defaults

### Wave 4: Broader Auth Fleet

Sites:

- `xueqiu`
- `notion`
- remaining cookie/header-dependent sites

Deliverables:

- shared auth/session patterns
- shared content extraction helpers
- reduced duplication across site workflows

Exit criteria:

- priority auth-heavy sites each have a clear target runtime shape
- maintenance burden is lower than the legacy YAML-only path

## Per-Site Decision Record

Each priority site should get a short decision record covering:

- current command count
- current execution mix: `script`, `native`
- whether builtin-only is sufficient
- whether a dedicated external repo is justified
- auth strategy and fragility risks
- target owner

Current consolidated record:

- [priority-site-decision-records.md](priority-site-decision-records.md)
- [full-site-migration-tracker.md](full-site-migration-tracker.md)

## When To Use A Dedicated External Repo

Use a site-specific external workflow repo only when at least one of these is true:

- the site has a separate owning team
- the site changes faster than the hub release cadence can support
- the site requires experimental helpers or release controls that should not block the builtin fleet
- the site is operationally sensitive enough to need independent rollback/versioning

Do not use a dedicated repo when:

- the site is stable and low-change
- the only benefit is stylistic separation
- the site does not have clear ownership
- the team cannot support pinned refs, validation, and release hygiene

## Tracking Fields

Track each site with:

- status: `workflow-script` or `workflow-native`
- ownership: `builtin` or `external`
- external source type: `path` or `git`
- pinned ref policy
- smoke command
- auth dependency
- release blocker status

## End State

- builtin workflow packages remain the default catalog
- selected high-change sites may override via external workflow repos
- priority sites have explicit runtime choices instead of accidental legacy-asset growth
- opencli-style YAML execution becomes implementation residue rather than the architectural center
