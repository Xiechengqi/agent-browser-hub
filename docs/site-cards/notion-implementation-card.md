# Notion Implementation Card

## Scope

Priority-site execution card for `notion`.

## Current Snapshot

- Commands: `7`
- Current execution mix: `script x7`
- Auth strategy: `UI`
- Default ownership: builtin
- External repo default: no

## First Migration Slice

Move these commands first:

1. `notion/search`
2. `notion/read`
3. `notion/write`

Reason:

- they cover discovery, read, and mutation
- they exercise the core workspace/session/page helper boundary
- they are the minimum useful slice for deciding whether the UI runtime contract is sufficient

## Target Runtime Shape

- `search`: `workflow-script`
- `read`: `workflow-script`
- `write`: `workflow-script`

Keep `favorites`, `new`, `sidebar`, and `status` on package-local `workflow-script` assets until the base helper layer is proven.

## Helper Boundary To Introduce

- workspace bootstrap and session validation
- search surface helper
- page content read helper
- editor append/write helper
- page-not-found and permission error normalization

## Repo Strategy

- remain builtin
- prioritize shared runtime helpers over repository separation

## Suggested Smoke Commands

```bash
agent-browser-hub run notion/search --query roadmap --format json
agent-browser-hub run notion/read --format json
agent-browser-hub run notion/write --text "workflow migration test" --format json
```

## Acceptance

- search/read/write share page/session primitives
- write-path errors are explicit and recoverable
- workspace variance does not collapse output shape entirely
- new Notion behavior lands in workflow metadata first

## Blockers

- UI-driven navigation and editor state
- permission variance across workspaces
- brittle selectors in rich editor surfaces
