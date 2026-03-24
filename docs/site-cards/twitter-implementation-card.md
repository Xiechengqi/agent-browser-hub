# Twitter Implementation Card

## Scope

Priority-site execution card for `twitter`.

## Current Snapshot

- Commands: `22`
- Current execution mix: `script x22`
- Auth strategy: `INTERCEPT`
- Default ownership: builtin
- External repo default: conditional, not immediate

## First Migration Slice

Move these commands first:

1. `twitter/timeline`
2. `twitter/profile`
3. `twitter/post`

Reason:

- they cover read timeline, read profile, and write action
- they exercise the most reusable session/navigation helpers
- they are enough to validate whether `workflow-script` is sufficient before considering an external repo

## Target Runtime Shape

- `timeline`: `workflow-script`
- `profile`: `workflow-script`
- `post`: `workflow-script`

Keep the rest on package-local `workflow-script` assets until the helper surface is stable.

## Helper Boundary To Introduce

- authenticated page bootstrap
- timeline/feed extraction helper
- profile extraction helper
- composer/write action helper
- common error normalization for auth-loss and page-shape drift

## Repo Strategy

- phase 1: stay in builtin `workflows/twitter`
- phase 2: consider external `git` repo only if selector churn and release cadence justify it
- if externalized, require pinned tag or commit SHA for non-development use

## Suggested Smoke Commands

```bash
agent-browser-hub run twitter/timeline --type for-you --format table
agent-browser-hub run twitter/profile elonmusk --format json
agent-browser-hub run twitter/post "test from workflow migration" --format json
```

## Acceptance

- the three commands resolve via workflow package metadata
- the core read paths no longer depend on central cross-package YAML references
- write-path failures produce normalized auth/action errors
- builtin fallback behavior stays understandable if an external override is later introduced

## Blockers

- auth/session volatility
- frequent DOM and route changes
- intercept-specific coupling from legacy opencli behavior
- large command surface makes premature full-site migration risky
