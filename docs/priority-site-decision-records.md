# Priority Site Decision Records

## Goal

Provide a concrete decision layer for the first eight migration-priority sites so the workflow-package rollout can move from generic planning into site execution.

## Decision Summary

| Site | Commands | Current mix | Recommended next target | Ownership default | External repo default | Suggested smoke command |
| --- | ---: | --- | --- | --- | --- | --- |
| `twitter` | 22 | `script x22` | consolidate helper-heavy builtin scripts first, possible later external repo | builtin | conditional | `twitter/timeline` |
| `boss` | 15 | `script x15` | keep builtin script helpers until ownership is clear | builtin | conditional | `boss/joblist` |
| `reddit` | 15 | `script x15` | keep expanding content/auth helpers in `workflow-script` | builtin | no | `reddit/frontpage` |
| `bilibili` | 11 | `script x10`, `native x1` | keep `native` reference site and expand selectively | builtin | no | `bilibili/feed` |
| `xueqiu` | 7 | `script x7` | keep auth/session reuse in `workflow-script` | builtin | conditional | `xueqiu/feed` |
| `notion` | 7 | `script x7` | keep UI/session helpers in `workflow-script` | builtin | no | `notion/search` |
| `wikipedia` | 4 | `script x4` | keep as public script baseline | builtin | no | `wikipedia/summary` |
| `hackernews` | 8 | `script x8` | keep as public low-risk script regression anchor | builtin | no | `hackernews/top` |

## Site Records

### Twitter

- Current state: 22 commands, `script x22`, auth strategy `INTERCEPT`, high-change site.
- Runtime target: keep consolidating common timeline/profile/post interactions behind `workflow-script` helpers before considering more `native`.
- Ownership: keep builtin as the default baseline.
- External repo: justified only if a dedicated owner needs faster release cadence or frequent selector/auth fixes independent of hub release.
- Pinned ref policy if externalized: require `git` tag or commit SHA, not a floating branch in production-like usage.
- Main blockers: auth/session fragility, selector churn, breadth of command surface.
- Exit condition for next wave: `timeline`, `profile`, and one write action stop depending on sprawling raw YAML logic.

### Boss

- Current state: 15 commands, `script x15`, auth strategy `COOKIE`, interaction-heavy workflow.
- Runtime target: introduce shared search/detail/chat helper surface through `workflow-script`.
- Ownership: builtin until there is a clear operator or team responsible for recruiting-site changes.
- External repo: possible, but only if the release cadence and ownership are clearly separate from the hub.
- Main blockers: login/session state, anti-bot friction, multi-step interaction flows.
- Exit condition for next wave: `joblist`, `detail`, and one messaging path share runtime helpers instead of duplicating YAML logic.

### Reddit

- Current state: 15 commands, `script x15`, auth strategy `COOKIE`.
- Runtime target: move read/search/user extraction toward `workflow-script` helpers while keeping write actions conservative.
- Ownership: builtin.
- External repo: not recommended by default because the site is important but not obviously in need of separate repo governance yet.
- Main blockers: authenticated content variation, page shape drift, saved/upvoted/subreddit path diversity.
- Exit condition for next wave: at least one read path and one user path use shared helpers with normalized output.

### Bilibili

- Current state: 11 commands, `script x10`, `native x1`, auth strategy `COOKIE`.
- Runtime target: keep `bilibili/feed` as the reference `workflow-native` path and expand native only where signed APIs or deep adapters justify it.
- Ownership: builtin.
- External repo: not recommended unless platform-specific maintenance starts moving faster than hub release needs.
- Main blockers: signed API requirements, account/session handling, subtitle/feed complexity.
- Exit condition for next wave: native/script boundary is documented by example and at least one more complex flow is intentionally classified rather than left ambiguous.

### Xueqiu

- Current state: 7 commands, `script x7`, auth strategy `COOKIE`.
- Runtime target: introduce `workflow-script` helpers for market/feed/session reuse.
- Ownership: builtin.
- External repo: conditional; only justified if market-facing workflows need faster operational fixes.
- Main blockers: authenticated market content, watchlist state, feed and stock detail reuse.
- Exit condition for next wave: `feed`, `stock`, and `watchlist` share a common helper surface.

### Notion

- Current state: 7 commands, `script x7`, auth strategy `UI`.
- Runtime target: shift UI-heavy flows to `workflow-script` helpers with reusable page/session primitives.
- Ownership: builtin.
- External repo: not recommended for now because the command count is moderate and shared runtime helpers likely matter more than repo separation.
- Main blockers: UI-driven state, workspace variance, rich editor interactions.
- Exit condition for next wave: `search`, `read`, and one write path use reusable script-level helpers instead of site-specific YAML-only logic.

### Wikipedia

- Current state: 4 commands, `script x4`, auth strategy `PUBLIC`.
- Runtime target: keep as the public `workflow-script` baseline site and use it to validate script runtime ergonomics.
- Ownership: builtin.
- External repo: no.
- Main blockers: low complexity; main value is as a stable regression and runtime reference site.
- Exit condition for next wave: script runtime remains green and becomes the public baseline for release verification.

### HackerNews

- Current state: 8 commands, all `workflow-script`, auth strategy `PUBLIC`.
- Runtime target: remain the low-risk `workflow-script` regression anchor unless a strong reason emerges to add richer helper logic.
- Ownership: builtin.
- External repo: no.
- Main blockers: low complexity; the bigger risk is unnecessary churn rather than missing capability.
- Exit condition for next wave: keep smoke coverage stable and preserve it as a regression baseline while more complex sites evolve.

## Decisions That Follow From This

- first public baseline wave should center on `wikipedia` and `hackernews`
- first helper-heavy conversion wave should center on `reddit` and `notion`
- first external override proof should center on `twitter`, with `boss` as the second candidate rather than both moving at once
- `bilibili` should remain the reference native site instead of rushing more sites into `native`

## Review Trigger

Update this decision record when any of the following changes:

- a priority site changes ownership model
- a priority site changes its runtime mix materially, or a new `workflow-native` milestone lands
- a site is approved for an external repo
- release smoke commands change

## Implementation Cards

- [site-cards/twitter-implementation-card.md](site-cards/twitter-implementation-card.md)
- [site-cards/boss-implementation-card.md](site-cards/boss-implementation-card.md)
- [site-cards/reddit-implementation-card.md](site-cards/reddit-implementation-card.md)
- [site-cards/bilibili-implementation-card.md](site-cards/bilibili-implementation-card.md)
- [site-cards/xueqiu-implementation-card.md](site-cards/xueqiu-implementation-card.md)
- [site-cards/notion-implementation-card.md](site-cards/notion-implementation-card.md)
- [site-cards/wikipedia-implementation-card.md](site-cards/wikipedia-implementation-card.md)
- [site-cards/hackernews-implementation-card.md](site-cards/hackernews-implementation-card.md)
