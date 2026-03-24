# Workflow Runtime API

## Goal

Define the execution contract between the hub runtime and workflow packages.

## Supported Entry Types

### pipeline

- Backed by the existing pipeline executor
- Loads `execution.pipeline`
- Receives params through the current template renderer
- Best for simple and standard commands

### script

Current and planned contract:

- If `execution.script` points to a YAML asset, the hub should parse it as a compatibility script and execute it through the existing executor
- Non-YAML script assets remain the long-term target for a constrained workflow script runtime
- The future runtime may access helpers such as navigation, evaluate, cookies, storage, and network logs
- Intended for shared helper-heavy site commands that are still lighter than native adapters

### native

Planned contract:

- Resolves `execution.native`
- Dispatches into hub-native site adapters
- Used for advanced workflows with richer error handling, state machines, or upload/download flows

## Runtime Expectations

Each workflow command should resolve into one of:

- an executable `Script` for pipeline-backed execution
- a deferred script backend invocation
- a deferred native backend invocation

## Error Model

The runtime should normalize:

- package load errors
- compatibility errors
- command resolution errors
- execution errors
- unsupported entry type errors

## Future Work

- introduce a `ResolvedCommand` runtime enum
- add script sandbox contract
- route native workflow handlers through site-specific adapters

## Current Runtime Shape

The registry now resolves commands into a runtime enum with these variants:

- `Pipeline(Script)`
- `WorkflowScript(target)`
- `WorkflowNative(target)`
- `Native(handler)`

At the moment:

- `Pipeline` is executable end to end
- YAML-backed workflow `script` assets should converge onto the same executable path
- non-YAML `WorkflowScript` remains the unfinished backend

## Native Dispatch Contract

The codebase now includes a native dispatch skeleton that normalizes two target forms:

- registered native handlers
- workflow-native handlers

This dispatcher currently fails fast with explicit errors, but it is now the single place where future native workflow backends should be wired.

## First Native Target

The first native execution path is `bilibili_feed`. It currently uses the shared browser runtime to open bilibili, execute an authenticated in-page fetch against the dynamic feed endpoint, and format the returned rows through the existing output layer.
