# Boss Implementation Card

## Scope

Priority-site execution card for `boss`.

## Current Snapshot

- Commands: `15`
- Current execution mix: `script x15`
- Auth strategy: `COOKIE`
- Default ownership: builtin
- External repo default: conditional, only with clear owner

## First Migration Slice

Move these commands first:

1. `boss/joblist`
2. `boss/detail`
3. `boss/chatmsg`

Reason:

- they cover list, detail, and message retrieval
- they exercise the key search/detail/chat helper boundary
- they reveal whether session persistence and anti-bot handling need richer runtime support

## Target Runtime Shape

- `joblist`: `workflow-script`
- `detail`: `workflow-script`
- `chatmsg`: `workflow-script`

Keep remaining commands on package-local `workflow-script` assets while the helper layer proves stable.

## Helper Boundary To Introduce

- authenticated bootstrap and cookie sanity checks
- job list pagination helper
- job detail fetch/extraction helper
- chat thread/message fetch helper
- anti-fragile empty/error state normalization

## Repo Strategy

- builtin first
- external repo only if the recruiting workflow needs separate ownership and faster releases
- avoid externalization before helper boundaries and smoke coverage are stable

## Suggested Smoke Commands

```bash
agent-browser-hub run boss/joblist --format json
agent-browser-hub run boss/detail --security-id <id> --format json
agent-browser-hub run boss/chatmsg --uid <uid> --format json
```

## Acceptance

- list/detail/chat paths share reusable runtime helpers
- session loss and anti-bot failures surface normalized errors
- command metadata stays workflow-package first
- no new Boss command is added as YAML-only metadata

## Blockers

- session expiry
- anti-bot friction
- multi-step flows with brittle selectors
- unclear long-term ownership for site-specific maintenance
