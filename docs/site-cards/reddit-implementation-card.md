# Reddit Implementation Card

## Scope

Priority-site execution card for `reddit`.

## Current Snapshot

- Commands: `15`
- Current execution mix: `script x15`
- Auth strategy: `COOKIE`
- Default ownership: builtin
- External repo default: no

## First Migration Slice

Move these commands first:

1. `reddit/frontpage`
2. `reddit/search`
3. `reddit/user`

Reason:

- they cover public-ish content listing, filtered query flow, and profile extraction
- they are read-heavy and safer than write actions for the first helper migration
- they create reusable primitives for subreddit/user/post extraction

## Target Runtime Shape

- `frontpage`: `workflow-script`
- `search`: `workflow-script`
- `user`: `workflow-script`

Keep write-oriented commands like `comment`, `save`, `subscribe`, `upvote` conservative until read helpers are stable.

## Helper Boundary To Introduce

- authenticated or anonymous bootstrap detection
- post-list extraction helper
- search query helper
- user profile extraction helper
- score/comment/url normalization

## Repo Strategy

- remain builtin
- do not introduce an external repo unless the site becomes operationally independent from the hub roadmap

## Suggested Smoke Commands

```bash
agent-browser-hub run reddit/frontpage --limit 10 --format table
agent-browser-hub run reddit/search rust --limit 10 --format json
agent-browser-hub run reddit/user spez --format json
```

## Acceptance

- read-oriented commands share common extraction helpers
- outputs are normalized across list and user views
- auth variation does not cause opaque runtime failures
- write commands remain unaffected while read helpers are introduced

## Blockers

- page-shape variance between logged-in and logged-out states
- subreddit-specific layout differences
- partial auth and rate-limit behavior
