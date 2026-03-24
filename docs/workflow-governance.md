# Workflow Governance

## Source Of Truth

Effective immediately:

- `workflows/*` is the canonical source of command metadata
- `scripts/*` is a compatibility layer and implementation asset store
- new product-facing commands must not be introduced as YAML-only entries

## Rules

### New Commands

- every new command must start with a workflow package entry in `workflows/<site>/commands/<name>.toml`
- execution may temporarily point to an existing YAML asset, but the metadata must live in the workflow package
- if a command requires richer logic, choose `execution.script` or `execution.native` explicitly instead of extending YAML without bounds

### Existing YAML Assets

- existing files under `scripts/*` may remain during migration
- editing an existing YAML asset is acceptable when:
  - the command is already represented in `workflows/*`
  - the change is implementation-only
- adding a brand new `scripts/<site>/<command>.yaml` without a matching workflow manifest is not acceptable

### Site Overrides

- override granularity is site-level only
- command-level mixing between builtin and external packages is disallowed
- an external site package replaces the builtin package for the same site

### Entry Type Selection

- use `pipeline` for straightforward navigation and extraction flows
- use `script` for helper-heavy but still runtime-constrained flows
- use `native` for complex adapters, stateful logic, or platform-specific behavior

## Review Checklist

When reviewing command additions or refactors:

- does the command exist in `workflows/*` first
- is the chosen entry type justified
- is the workflow package the only metadata source
- if YAML is touched, is it clearly an execution asset rather than the primary definition
- if an external source is introduced, is it site-scoped and version-pinned appropriately

## Migration Policy

- priority sites should steadily move from YAML-backed workflow entries toward `script` or `native` where justified
- bulk-generated workflow manifests are acceptable as an intermediate state
- future cleanup can remove legacy YAML only after equivalent workflow execution paths are stable
