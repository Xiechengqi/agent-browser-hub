from __future__ import annotations

import json
import re
import sys
from pathlib import Path

from validate_workflows import ROOT, validate

DOC_EXPECTATIONS = {
    ROOT / "docs" / "release-checklist.md": [
        "workflow_commands = 262",
        "source_breakdown.script = 261",
        "source_breakdown.native = 1",
    ],
    ROOT / "docs" / "release-summary.md": [
        "44 workflow sites",
        "262 workflow commands",
        "261 script-backed commands",
        "1 native-backed command",
    ],
    ROOT / "docs" / "workflow-migration-plan.md": [
        "261 workflow-script + 1 workflow-native",
    ],
    ROOT / "docs" / "full-site-migration-tracker.md": [
        "261 workflow-script + 1 workflow-native",
    ],
}

STALE_DOC_PATTERNS = {
    "workflow-pipeline": re.compile(r"\bworkflow-pipeline\b"),
    "yaml-backed-status": re.compile(r"status:\s*`yaml-backed`"),
    "in-progress-status": re.compile(r"`in-progress`"),
    "planned-status": re.compile(r"`planned`"),
    "legacy-pipeline-mix": re.compile(r"pipeline x\d+"),
}


def collect_pipeline_entries() -> list[str]:
    pipeline_entries: list[str] = []
    for command_path in sorted((ROOT / "workflows").glob("*/commands/*.toml")):
        text = command_path.read_text(encoding="utf-8")
        if re.search(r'^entry\s*=\s*"pipeline"', text, re.M):
            pipeline_entries.append(str(command_path.relative_to(ROOT)))
    return pipeline_entries


def collect_escaping_assets() -> list[str]:
    problems: list[str] = []
    for command_path in sorted((ROOT / "workflows").glob("*/commands/*.toml")):
        site_dir = command_path.parent.parent
        text = command_path.read_text(encoding="utf-8")
        for key in ("pipeline", "script"):
            match = re.search(rf'^{key}\s*=\s*"([^"]+)"', text, re.M)
            if not match:
                continue
            asset_value = match.group(1)
            asset_path = (site_dir / asset_value).resolve()
            if site_dir.resolve() not in asset_path.parents:
                problems.append(f"{command_path.relative_to(ROOT)} -> {asset_value}")
    return problems


def collect_stale_doc_hits() -> list[str]:
    hits: list[str] = []
    for path in sorted((ROOT / "docs").rglob("*.md")):
        text = path.read_text(encoding="utf-8")
        for label, pattern in STALE_DOC_PATTERNS.items():
            for match in pattern.finditer(text):
                line = text.count("\n", 0, match.start()) + 1
                hits.append(f"{path.relative_to(ROOT)}:{line}: {label}")
    return hits


def collect_missing_doc_expectations() -> list[str]:
    missing: list[str] = []
    for path, snippets in DOC_EXPECTATIONS.items():
        text = path.read_text(encoding="utf-8")
        for snippet in snippets:
            if snippet not in text:
                missing.append(f"{path.relative_to(ROOT)} missing '{snippet}'")
    return missing


def main() -> int:
    workflow_validation = validate()

    pipeline_entries = collect_pipeline_entries()
    escaping_assets = collect_escaping_assets()
    stale_doc_hits = collect_stale_doc_hits()
    missing_doc_expectations = collect_missing_doc_expectations()

    errors = []
    if not workflow_validation["ok"]:
        errors.extend(workflow_validation["errors"])
    if pipeline_entries:
        errors.append(f"pipeline entries remain: {len(pipeline_entries)}")
        errors.extend(pipeline_entries)
    if escaping_assets:
        errors.append("workflow asset references escape package roots")
        errors.extend(escaping_assets)
    if stale_doc_hits:
        errors.append("stale migration wording remains in docs")
        errors.extend(stale_doc_hits)
    if missing_doc_expectations:
        errors.append("release docs are missing current catalog expectations")
        errors.extend(missing_doc_expectations)

    result = {
        "ok": not errors,
        "errors": errors,
        "warnings": workflow_validation["warnings"],
        "summary": {
            **workflow_validation["summary"],
            "pipeline_entries": len(pipeline_entries),
            "escaping_asset_refs": len(escaping_assets),
            "stale_doc_hits": len(stale_doc_hits),
            "missing_doc_expectations": len(missing_doc_expectations),
        },
    }
    print(json.dumps(result, ensure_ascii=False, indent=2))
    return 0 if result["ok"] else 1


if __name__ == "__main__":
    sys.exit(main())
