# Xueqiu Implementation Card

## Scope

Priority-site execution card for `xueqiu`.

## Current Snapshot

- Commands: `7`
- Current execution mix: `script x7`
- Auth strategy: `COOKIE`
- Default ownership: builtin
- External repo default: conditional

## First Migration Slice

Move these commands first:

1. `xueqiu/feed`
2. `xueqiu/stock`
3. `xueqiu/watchlist`

Reason:

- they cover timeline, quote detail, and personalized list state
- they exercise market/session reuse better than one-off public commands
- they are enough to decide whether the site needs only `workflow-script` helpers or separate operational ownership

## Target Runtime Shape

- `feed`: `workflow-script`
- `stock`: `workflow-script`
- `watchlist`: `workflow-script`

Keep `earnings-date`, `hot`, `hot-stock`, and `search` on package-local `workflow-script` assets until helper extraction is stable.

## Helper Boundary To Introduce

- cookie/session bootstrap
- quote fetch and normalization helper
- watchlist/list-page extraction helper
- market content empty/error state normalization

## Repo Strategy

- stay builtin first
- consider external `git` repo only if market-facing workflows need materially faster response than hub releases
- if externalized, require pinned immutable ref outside development

## Suggested Smoke Commands

```bash
agent-browser-hub run xueqiu/feed --limit 10 --format table
agent-browser-hub run xueqiu/stock SH600519 --format json
agent-browser-hub run xueqiu/watchlist --category 1 --format json
```

## Acceptance

- feed/stock/watchlist share reusable runtime helpers
- quote fields are normalized consistently
- session loss is surfaced clearly
- metadata remains workflow-package first

## Blockers

- authenticated market content
- watchlist personalization
- potential rate-limit or anti-automation friction
