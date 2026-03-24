# Bilibili Implementation Card

## Scope

Priority-site execution card for `bilibili`.

## Current Snapshot

- Commands: `11`
- Current execution mix: `script x10`, `native x1`
- Auth strategy: `COOKIE`
- Default ownership: builtin
- External repo default: no

## First Migration Slice

Focus on these commands first:

1. `bilibili/feed`
2. `bilibili/subtitle`
3. `bilibili/search`

Reason:

- `feed` is already the live `workflow-native` reference path
- `subtitle` and `search` are useful comparison points for deciding `script` vs `native`
- together they cover signed API, structured extraction, and list query behavior

## Target Runtime Shape

- `feed`: keep `workflow-native`
- `subtitle`: evaluate for `workflow-script` before considering `native`
- `search`: evaluate for `workflow-script`

Do not expand `native` by default. Use it only where signed APIs or low-level adapters clearly justify it.

## Helper Boundary To Introduce

- signed request helper boundary versus pure page extraction boundary
- subtitle selection and normalization helper
- search query/result normalization helper
- shared account/session validation

## Repo Strategy

- remain builtin
- keep Bilibili as the reference site that documents when `native` is truly warranted

## Suggested Smoke Commands

```bash
agent-browser-hub run bilibili/feed --format table
agent-browser-hub run bilibili/search --query 鬼灭之刃 --format json
agent-browser-hub run bilibili/subtitle --bvid <bvid> --format json
```

## Acceptance

- `feed` remains green as the native reference path
- `search` and `subtitle` have a documented runtime choice instead of accidental YAML growth
- output normalization is consistent across native and non-native paths
- signed-request logic does not leak into generic helper surfaces

## Blockers

- signed API requirements
- account/session handling
- mixed data sources between page extraction and API access
