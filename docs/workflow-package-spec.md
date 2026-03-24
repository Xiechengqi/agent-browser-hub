# Workflow Package Spec

## Goals

- Replace `yaml-only` as the long-term first-class execution unit.
- Group commands, helpers, metadata, and tests by site.
- Keep builtin workflows in this repository while allowing site-level external overrides.

## Package Layout

```text
workflows/<site>/
  workflow.toml
  commands/
    <command>.toml
  scripts/
    <command>.json
  helpers/
  fixtures/
  tests/
```

## Site Manifest

`workflow.toml` defines package metadata:

- `schema_version`
- `site`
- `display_name`
- `version`
- `runtime.workflow_api`
- `runtime.min_hub_version`
- `runtime.min_agent_browser_version`
- `package.kind`
- `auth.strategy`
- `auth.login_required`
- `auth.domains`
- `ui.icon`
- `ui.category`
- `ui.tags`

## Command Manifest

Each `commands/*.toml` defines:

- `name`
- `description`
- `strategy`
- `params`
- `execution.entry`
- `execution.pipeline | execution.script | execution.native`
- `output.columns`

## Execution Entry Types

- `pipeline`: declarative workflow backed by the runtime pipeline executor
- `script`: package-local command asset, including wrapper-style script payloads and helper-backed execution
- `native`: runtime-native command for complex site adapters

## Design Constraints

- Site is the smallest override unit.
- Workflow manifests provide metadata only; they do not bypass runtime validation.
- Package metadata is exposed to CLI and web UI.
- Output schema must remain stable per command.
