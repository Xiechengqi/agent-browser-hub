# Release Summary

## Theme

This release shifts `agent-browser-hub` from a YAML-only command catalog toward a workflow-package architecture built for agent-browser-native execution and site-level overrides.

## Delivered

### Workflow Package Rollout

- builtin workflow packages now exist for all 44 legacy script sites
- workflow packages are the primary metadata source for command listing and resolution
- external site-level workflow overrides are supported via config

### Runtime Progress

- `workflow-script` commands now execute from package-local assets across the builtin catalog
- many migrated `workflow-script` assets are still wrapper-style script payloads around prior pipeline logic
- `native` dispatch is wired and includes a real `bilibili/feed` path

### UI Progress

- the home page now includes a workflow-center overview
- command cards and site groups show source and fallback state
- settings now expose workflow source configuration and live source diagnostics

### Verification

- static workflow validation tool added
- release audit tool added for preflight catalog and doc consistency checks
- workflow/site coverage currently validates cleanly
- frontend workflow-center files pass TypeScript parse checks

## Current Shape

- 44 workflow sites
- 262 workflow commands
- 261 script-backed commands
- 1 native-backed command

## Follow-Up After Release

- deepen priority sites from wrapper-style script entries into richer `script` and `native` implementations
- add explicit source refresh and cache maintenance actions
- add registry/runtime tests in a build-enabled environment
