# Workflow Resolution Spec

## Resolution Order

1. Builtin workflow packages from `workflows/*`
2. External workflow sources from config
3. Site-level override merge
4. Compatibility check
5. Registry publish

## Override Rules

- Overrides are site-level only.
- An external `twitter` package replaces builtin `twitter` entirely.
- Command-level mixing between builtin and external sources is not allowed.

## Modes

- `builtin-only`
- `prefer-external`
- `strict-external`

## Fallback

When `fallback_to_builtin = true`:

- invalid external package -> warning + builtin package remains active

When `fallback_to_builtin = false`:

- invalid external package -> startup error in strict modes

## External Source Types

- `path`
- `git`

The first implementation stage resolves metadata for both, but only `path` is expected to be operational immediately.

## Effective Precedence

Current bootstrap order is:

1. YAML commands
2. registered native commands
3. builtin workflow packages
4. external workflow packages

This means workflow packages override legacy YAML/native registrations for the same `site/name`, and external workflow packages override builtin workflow packages.
