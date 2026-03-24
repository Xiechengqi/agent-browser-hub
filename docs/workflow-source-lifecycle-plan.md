# Workflow Source Lifecycle Plan

## Goal

Turn external workflow sources from a loader feature into an operable package supply model.

## Supported Source Types

- `path`
- `git`

## Lifecycle Areas

### Cache Layout

- cache under `workflow.cache_dir`
- isolate by `site + source url + ref`
- keep enough metadata to identify the active checkout and last refresh
- keep diagnostics readable even when cache content is missing
- diagnostics must not create cache directories or mutate git state

### Source Ownership Model

- builtin workflows are the default operational baseline
- external sources are exceptions, not the primary fleet model
- each site may have at most one effective external override
- overrides must be traceable to an owning team or operator

### Change Windows

- changing a priority-site override should happen in a controlled deployment window
- `git` branch refs are acceptable only for development or short-lived experiments
- production overrides should prefer immutable tags or commit SHAs
- switching `ref` should be treated as a deploy, not an ad hoc settings tweak

### Update Semantics

For `path`:

- read directly from the configured directory
- no cache refresh step

For `git`:

- clone on first use
- refresh on explicit operator action or startup policy
- preserve pinned refs when configured
- diagnostics should inspect existing cache only
- fetch/checkout should happen only during execution preparation or explicit refresh actions

Recommended startup policy:

1. do not auto-refresh all git sources blindly on boot
2. inspect cached state first
3. refresh only sources explicitly configured for refresh-on-start or chosen by operator action
4. surface stale-cache state to the operator instead of hiding it

## Ref Policy

- `ref` may be a branch, tag, or commit
- production-like usage should prefer tags or commits
- branch refs are acceptable for development or controlled environments

## Fallback Rules

When `fallback_to_builtin = true`:

- invalid external package keeps builtin active
- source health should show degraded state

When `fallback_to_builtin = false`:

- invalid external package is a hard failure in `strict-external`
- `prefer-external` may still degrade with an explicit warning depending on operator intent

## Compatibility Checks

At package load time:

- verify `runtime.workflow_api`
- verify minimum hub version
- verify minimum agent-browser version when known

At governance level:

- reject packages that omit required manifests
- reject packages that reference missing command assets
- warn when external packages downgrade command coverage for a site
- warn when a branch ref is used in a production-like environment

## Operator Visibility

The UI and API should surface:

- effective source kind
- source location
- pinned ref
- cache path
- last refresh status
- fallback status
- compatibility warnings
- whether the current result came from builtin fallback or the configured external package
- whether diagnostics are reading cache-only state or a freshly prepared checkout

## Failure Handling

When a source fails:

1. preserve the configured source in diagnostics
2. attach the resolution error to the site status
3. if fallback is enabled and builtin exists, mark fallback active explicitly
4. if fallback is disabled, fail loudly and do not silently downgrade
5. do not mutate the cached checkout during diagnostics-only inspection

When a cached git checkout is stale or missing:

- diagnostics should report cache-miss or stale-cache state
- execution may prepare the checkout if policy allows it
- operators should have a future explicit refresh/clear action instead of relying on incidental side effects

## Security And Trust

- external git sources expand the trusted supply chain and must be treated as code/config intake
- use pinned refs for non-development environments
- prefer repository allowlists when this moves into multi-operator usage
- document ownership for every external override in deployment config or ops notes
- avoid mixing secrets policy into package repos unless the repo is explicitly trusted

## Future API Additions

- refresh source
- inspect effective package
- clear cached checkout
- show source diagnostics
- show cache freshness
- show compatibility check results
- show whether a ref is immutable or floating

## Current Diagnostics Endpoint

The hub should expose source inspection through:

- `GET /api/workflow/sources`

The response should include:

- configured source type and location
- current mode and fallback settings
- whether builtin fallback is available
- whether the external source resolved successfully
- effective origin after fallback
- package version and command count when resolved
- load error when resolution fails

## Release Readiness For Source Operations

Before calling source overrides production-ready:

- builtin/path/git behavior is manually exercised
- diagnostics are side-effect free
- fallback-on and fallback-off are both verified
- at least one pinned git ref path is tested
- at least one cache-miss path is tested
