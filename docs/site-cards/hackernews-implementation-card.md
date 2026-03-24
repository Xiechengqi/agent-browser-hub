# HackerNews Implementation Card

## Scope

Priority-site execution card for `hackernews`.

## Current Snapshot

- Commands: `8`
- Current execution mix: `script x8`
- Auth strategy: `PUBLIC`
- Default ownership: builtin
- External repo default: no

## First Migration Slice

Focus on these commands first:

1. `hackernews/top`
2. `hackernews/search`
3. `hackernews/user`

Reason:

- they cover ranking list, search, and profile lookup
- they are reliable public commands for regression gating
- they should remain simple enough to expose accidental complexity quickly

## Target Runtime Shape

- `top`: keep package-local `workflow-script`
- `search`: keep package-local `workflow-script`
- `user`: keep package-local `workflow-script`

Only add richer helper structure later if shared public helpers clearly reduce duplication without adding runtime complexity.

## Helper Boundary To Introduce

- minimal shared normalization only
- avoid building a heavy helper layer unless there is clear repeated logic

## Repo Strategy

- remain builtin
- keep HackerNews as the low-risk regression anchor, not a site for architectural experimentation

## Suggested Smoke Commands

```bash
agent-browser-hub run hackernews/top --limit 10 --format table
agent-browser-hub run hackernews/search --query rust --format json
agent-browser-hub run hackernews/user pg --format json
```

## Acceptance

- smoke coverage remains stable and fast
- output remains predictable
- the site continues to serve as the public regression anchor while more complex sites change

## Blockers

- low complexity; the main risk is unnecessary churn
