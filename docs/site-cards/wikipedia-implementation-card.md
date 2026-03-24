# Wikipedia Implementation Card

## Scope

Priority-site execution card for `wikipedia`.

## Current Snapshot

- Commands: `4`
- Current execution mix: `script x4`
- Auth strategy: `PUBLIC`
- Default ownership: builtin
- External repo default: no

## First Migration Slice

Focus on these commands first:

1. `wikipedia/summary`
2. `wikipedia/search`
3. `wikipedia/trending`

Reason:

- `summary` and `search` now form the first live public `workflow-script` baseline
- `trending` is the next low-risk public command that can validate script ergonomics without auth complexity
- together they form the cleanest public regression baseline for the new runtime

## Target Runtime Shape

- `summary`: keep `workflow-script`
- `search`: keep `workflow-script`
- `trending`: evaluate for `workflow-script`

`random` can remain a simple package-local `workflow-script` wrapper unless shared public helpers make deeper conversion trivial.

## Helper Boundary To Introduce

- public page bootstrap
- summary/article normalization helper
- search result normalization helper
- trending list fetch helper

## Repo Strategy

- remain builtin
- use Wikipedia as the public reference site for script runtime quality

## Suggested Smoke Commands

```bash
agent-browser-hub run wikipedia/summary --title Rust --format json
agent-browser-hub run wikipedia/search --query async --limit 5 --format table
agent-browser-hub run wikipedia/trending --limit 10 --format json
```

## Acceptance

- script runtime stays green on a public low-risk site
- output shapes are stable across summary/search/trending
- public baseline is suitable for release verification in environments without private credentials

## Blockers

- low complexity; main risk is overengineering rather than missing capability
- script runtime contract is still young and needs a stable baseline site
