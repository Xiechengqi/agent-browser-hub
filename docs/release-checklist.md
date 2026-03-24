# Release Checklist

## Scope

This checklist is for the workflow-package migration release of `agent-browser-hub`.

## Architecture Gates

- [x] `workflows/*` covers every site under `scripts/*`
- [x] builtin workflow packages are the primary metadata source
- [x] site-level external overrides are supported
- [x] at least one `workflow-native` command exists
- [x] at least one `workflow-script` command exists
- [x] governance rules are documented

## Static Validation

Run:

```bash
python3 tools/release_audit.py
python3 tools/validate_workflows.py
```

Expected current summary:

- `script_sites = 44`
- `workflow_sites = 44`
- `workflow_commands = 262`
- `source_breakdown.script = 261`
- `source_breakdown.native = 1`
- `pipeline_entries = 0`
- no workflow command references `../../scripts/...`
- no stale migration wording remains in release docs

## Frontend Parse Check

Run:

```bash
node - <<'JS'
const fs = require('fs');
const path = require('path');
const ts = require('./web/node_modules/typescript');
const files = [
  'web/app/page.tsx',
  'web/components/command/WorkflowOverview.tsx',
  'web/components/command/CommandCard.tsx',
  'web/components/command/CommandGroup.tsx',
  'web/components/command/CommandOutline.tsx',
  'web/components/layout/SettingsDialog.tsx',
  'web/lib/hooks/useCommands.ts',
  'web/lib/store/commands.ts',
  'web/types/command.ts',
  'web/lib/api/commands.ts',
];
let failed = false;
for (const file of files) {
  const source = fs.readFileSync(file, 'utf8');
  const result = ts.transpileModule(source, {
    compilerOptions: {
      jsx: ts.JsxEmit.ReactJSX,
      target: ts.ScriptTarget.ES2020,
      module: ts.ModuleKind.ESNext,
      esModuleInterop: true,
    },
    reportDiagnostics: true,
    fileName: path.basename(file),
  });
  if (result.diagnostics && result.diagnostics.length) {
    failed = true;
    console.log('## ' + file);
    for (const d of result.diagnostics) {
      console.log(ts.flattenDiagnosticMessageText(d.messageText, '\n'));
    }
  }
}
if (failed) process.exit(1);
console.log('ok');
JS
```

## Runtime/API Smoke Targets

Manually verify:

- `GET /api/commands`
- `GET /api/workflow/sources`
- `GET /api/settings`
- `POST /api/execute/wikipedia/summary`
- `POST /api/execute/bilibili/feed`

Recommended order:

1. start the hub with the default builtin workflow configuration
2. verify `/api/commands` returns workflow-backed metadata for representative builtin sites
3. verify `/api/workflow/sources` returns builtin resolution with no external mutation side effects
4. verify `/api/settings` returns the persisted workflow block
5. execute two representative `workflow-script` commands, including one wrapper-style script and one helper-heavy script
6. execute one `workflow-native` command

Required evidence to record:

- response sample for `GET /api/commands`
- response sample for `GET /api/workflow/sources`
- one successful wrapper-style `workflow-script` execution result
- one successful helper-heavy `workflow-script` execution result
- one successful `native` execution result

## UI Smoke Targets

Manually verify:

- home page workflow overview cards
- source badges on command cards and site groups
- settings dialog workflow source status section
- workflow override settings save/load roundtrip

Required UI path:

1. open home page and confirm workflow overview totals render
2. open at least one builtin site group and confirm source badges are visible
3. open settings and confirm workflow source diagnostics load only after auth/open state is ready
4. save a workflow override configuration and confirm command list plus source diagnostics refresh without page reload
5. trigger an invalid workflow override and confirm the error is visible in settings
6. confirm fallback badge appears in settings, site group, and command card when builtin fallback is active

## External Source Smoke

Manually verify:

- builtin-only mode keeps builtin packages active
- valid external `path` override replaces builtin site package
- invalid external source reports error through `/api/workflow/sources`
- fallback state is visible in UI and command metadata

Cover all three operator modes:

1. `builtin-only`
2. `prefer-external`
3. `strict-external`

Minimum external cases:

- valid `path` override for a priority site
- valid `git` override for a priority site with cached checkout already present
- invalid `path` override
- invalid `git` override with fallback enabled
- invalid `git` override with fallback disabled

Sign-off rule:

- a release is not ready unless builtin, path override, git override, fallback-on, and fallback-off have all been exercised at least once in a clean environment

## Out Of Scope For This Environment

These should be run in a clean release environment, not in the current restricted workflow:

- `cargo check`
- `cargo build --release`
- end-to-end browser execution against real credentials/cookies
- release asset packaging
