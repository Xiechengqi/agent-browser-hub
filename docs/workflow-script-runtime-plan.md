# Workflow Script Runtime Plan

## Goal

Implement a real backend for `execution.script` that sits between declarative pipelines and fully native adapters.

## Why It Exists

`pipeline` is sufficient for linear extraction flows, but several sites need:

- shared helpers
- branching logic
- retry wrappers
- request assembly
- richer intermediate state

Without a script runtime, those commands either stay trapped in YAML or jump too early into bespoke native code.

## Runtime Contract

### Input

- workflow package root
- script asset path from `execution.script`
- normalized params
- current command metadata

### Output

- `serde_json::Value`
- formatted through the existing output layer

### Required Helpers

The first version should expose a small, explicit helper surface:

- `open(url)`
- `wait(ms)`
- `eval(js)`
- `click(selector)`
- `fill(selector, value)`
- `press(key)`
- `snapshot()`
- `cookies()`
- `set_cookie(name, value, domain?)`

## Proposed Model

### Stage 1

Support YAML-backed script assets as compatibility input.

- if `execution.script` points to `.yaml` or `.yml`, parse it as a legacy `Script`
- apply workflow metadata defaults
- execute through the existing executor

This removes the current fail-fast path for YAML-backed script assets and keeps migration moving.

### Stage 2

Introduce a constrained helper-backed runtime.

Options to evaluate:

- embed a lightweight scripting engine
- use a small Rust-native DSL over structured steps

Preferred direction:

- start with a Rust-native constrained script format instead of embedding a large JS runtime immediately
- keep helper calls explicit and typed
- avoid arbitrary filesystem and process access

### Stage 3

Add shared helper modules by site/package:

- `helpers/*.yaml` or future helper assets
- controlled import model
- package-local reusable routines

## Boundary With Native

Use `native` when the command needs:

- complex state machines
- binary uploads/downloads
- multi-phase auth recovery
- platform or protocol adapters
- performance-sensitive custom logic

Use `script` when the command mainly needs:

- helper reuse
- moderate branching
- richer orchestration over existing browser primitives

## Execution Errors

The runtime should normalize:

- script load failure
- helper resolution failure
- param validation failure
- browser interaction failure
- unsupported helper usage

## First Target Sites

After the runtime exists, the best migration candidates are:

1. `twitter`
2. `boss`
3. `reddit`
4. `notion`
5. `xueqiu`
