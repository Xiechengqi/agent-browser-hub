from __future__ import annotations

import json
import sys
from collections import defaultdict
from pathlib import Path

import yaml

ROOT = Path(__file__).resolve().parents[1]
SCRIPTS_DIR = ROOT / "scripts"
WORKFLOWS_DIR = ROOT / "workflows"

SUPPORTED_ENTRY_KINDS = {"pipeline", "script", "native"}
PRIORITY_SITES = [
    "twitter",
    "boss",
    "reddit",
    "bilibili",
    "xueqiu",
    "notion",
    "wikipedia",
    "hackernews",
]


def read_yaml(path: Path):
    return yaml.safe_load(path.read_text(encoding="utf-8"))


def parse_toml_value(raw: str):
    raw = raw.strip()
    if raw == "":
        return ""
    return yaml.safe_load(raw)


def read_toml(path: Path) -> dict:
    data: dict = {}
    current = data
    array_section_name: str | None = None

    for original_line in path.read_text(encoding="utf-8").splitlines():
        line = original_line.strip()
        if not line or line.startswith("#"):
            continue

        if line.startswith("[[") and line.endswith("]]"):
            section = line[2:-2].strip()
            bucket = data.setdefault(section, [])
            if not isinstance(bucket, list):
                raise ValueError(f"{path}: section '{section}' is not an array")
            item: dict = {}
            bucket.append(item)
            current = item
            array_section_name = section
            continue

        if line.startswith("[") and line.endswith("]"):
            section = line[1:-1].strip()
            bucket = data.setdefault(section, {})
            if not isinstance(bucket, dict):
                raise ValueError(f"{path}: section '{section}' is not a table")
            current = bucket
            array_section_name = None
            continue

        if "=" not in line:
            raise ValueError(f"{path}: invalid TOML line '{original_line}'")

        key, value = line.split("=", 1)
        key = key.strip()
        value = parse_toml_value(value)
        current[key] = value

        if array_section_name is None:
            current = data if current is data else current

    return data


def validate() -> dict:
    errors: list[str] = []
    warnings: list[str] = []

    script_sites = sorted(path.name for path in SCRIPTS_DIR.iterdir() if path.is_dir())
    workflow_sites = sorted(
        path.name
        for path in WORKFLOWS_DIR.iterdir()
        if path.is_dir() and (path / "workflow.toml").exists()
    )

    missing_sites = sorted(set(script_sites) - set(workflow_sites))
    extra_sites = sorted(set(workflow_sites) - set(script_sites))
    if missing_sites:
        errors.append(f"missing workflow sites: {', '.join(missing_sites)}")
    if extra_sites:
        warnings.append(f"extra workflow-only sites: {', '.join(extra_sites)}")

    total_commands = 0
    site_command_counts: dict[str, int] = {}
    source_breakdown: dict[str, int] = defaultdict(int)

    for site in workflow_sites:
        site_dir = WORKFLOWS_DIR / site
        workflow_path = site_dir / "workflow.toml"
        commands_dir = site_dir / "commands"

        try:
            workflow = read_toml(workflow_path)
        except Exception as exc:
            errors.append(f"{workflow_path}: invalid workflow manifest: {exc}")
            continue

        for field in ["site", "display_name", "version"]:
            if not workflow.get(field):
                errors.append(f"{workflow_path}: missing field '{field}'")

        if workflow.get("site") != site:
            errors.append(
                f"{workflow_path}: site mismatch, manifest={workflow.get('site')} dir={site}"
            )

        include = workflow.get("commands", {}).get("include", [])
        if not isinstance(include, list):
            errors.append(f"{workflow_path}: commands.include must be a list")
            include = []

        manifest_names: list[str] = []
        seen_names: set[str] = set()

        for command_path in sorted(commands_dir.glob("*.toml")):
            try:
                command = read_toml(command_path)
            except Exception as exc:
                errors.append(f"{command_path}: invalid command manifest: {exc}")
                continue

            name = command.get("name")
            if not name:
                errors.append(f"{command_path}: missing field 'name'")
                continue

            manifest_names.append(name)
            total_commands += 1
            site_command_counts[site] = site_command_counts.get(site, 0) + 1

            if name in seen_names:
                errors.append(f"{command_path}: duplicate command name '{name}' in site {site}")
            seen_names.add(name)

            if not command.get("strategy"):
                errors.append(f"{command_path}: missing field 'strategy'")

            execution = command.get("execution", {})
            if not isinstance(execution, dict):
                errors.append(f"{command_path}: [execution] must be a table")
                continue

            entry = execution.get("entry")
            if entry not in SUPPORTED_ENTRY_KINDS:
                errors.append(f"{command_path}: unsupported execution.entry '{entry}'")
                continue

            source_breakdown[entry] += 1
            asset_key = {"pipeline": "pipeline", "script": "script", "native": "native"}[entry]
            asset_value = execution.get(asset_key)

            if entry in {"pipeline", "script"}:
                if not asset_value:
                    errors.append(f"{command_path}: missing execution.{asset_key}")
                else:
                    asset_path = (site_dir / asset_value).resolve()
                    if not asset_path.exists():
                        errors.append(
                            f"{command_path}: referenced asset does not exist: {asset_value}"
                        )
                    elif site_dir.resolve() not in asset_path.parents:
                        errors.append(
                            f"{command_path}: referenced asset escapes workflow package: {asset_value}"
                        )
                    elif entry == "script":
                        suffix = asset_path.suffix.lower()
                        if suffix not in {".yaml", ".yml", ".json", ".toml"}:
                            errors.append(
                                f"{command_path}: unsupported script asset extension '{suffix}'"
                            )
                    else:
                        try:
                            payload = read_yaml(asset_path)
                            if payload is None:
                                warnings.append(f"{asset_path}: empty pipeline asset")
                        except Exception as exc:
                            errors.append(f"{asset_path}: invalid yaml asset: {exc}")

            if entry == "native" and not asset_value:
                errors.append(f"{command_path}: missing execution.native")

        if sorted(include) != sorted(manifest_names):
            errors.append(
                f"{workflow_path}: commands.include mismatch manifests={sorted(manifest_names)} include={sorted(include)}"
            )

    for site in PRIORITY_SITES:
        if site not in workflow_sites:
            errors.append(f"priority site missing workflow package: {site}")
            continue
        if site_command_counts.get(site, 0) == 0:
            errors.append(f"priority site has no commands: {site}")

    return {
        "ok": not errors,
        "errors": errors,
        "warnings": warnings,
        "summary": {
            "script_sites": len(script_sites),
            "workflow_sites": len(workflow_sites),
            "workflow_commands": total_commands,
            "priority_sites": len(PRIORITY_SITES),
            "source_breakdown": dict(sorted(source_breakdown.items())),
        },
    }


def main() -> int:
    result = validate()
    print(json.dumps(result, ensure_ascii=False, indent=2))
    return 0 if result["ok"] else 1


if __name__ == "__main__":
    sys.exit(main())
