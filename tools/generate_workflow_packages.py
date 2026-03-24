from __future__ import annotations

import json
from pathlib import Path
import yaml

ROOT = Path(__file__).resolve().parents[1]
SCRIPTS_DIR = ROOT / 'scripts'
WORKFLOWS_DIR = ROOT / 'workflows'

TYPE_MAP = {
    'str': 'string',
    'string': 'string',
    'int': 'int',
    'integer': 'int',
    'float': 'float',
    'number': 'float',
    'bool': 'bool',
    'boolean': 'bool',
}


def titleize(site: str) -> str:
    return ' '.join(part.capitalize() for part in site.replace('_', '-').split('-'))


def normalize_type(type_name: str | None) -> str:
    if not type_name:
        return 'string'
    return TYPE_MAP.get(type_name.lower(), type_name)


def toml_string(value: str) -> str:
    return json.dumps(value, ensure_ascii=False)


def toml_value(value):
    return json.dumps(value, ensure_ascii=False)


def load_yaml(path: Path):
    with path.open('r', encoding='utf-8') as fh:
        return yaml.safe_load(fh) or {}


def extract_strategy(data: dict) -> str:
    config = data.get('config') or {}
    return config.get('strategy') or data.get('strategy') or 'PUBLIC'


def extract_description(data: dict) -> str:
    meta = data.get('meta') or {}
    return meta.get('description') or data.get('description') or ''


def extract_domain(data: dict, site: str) -> str:
    config = data.get('config') or {}
    return data.get('domain') or config.get('domain') or site


def extract_params(data: dict):
    args = data.get('args')
    if isinstance(args, dict):
        params = []
        for name, spec in args.items():
            spec = spec or {}
            params.append({
                'name': name,
                'type': normalize_type(spec.get('type')),
                'required': bool(spec.get('required', False)),
                'default': spec.get('default', None),
                'description': spec.get('description', ''),
                'positional': bool(spec.get('positional', False)),
            })
        return params

    params = data.get('params')
    if isinstance(params, list):
        out = []
        for spec in params:
            spec = spec or {}
            out.append({
                'name': spec.get('name', ''),
                'type': normalize_type(spec.get('type')),
                'required': bool(spec.get('required', False)),
                'default': spec.get('default', None),
                'description': spec.get('description', ''),
                'positional': bool(spec.get('positional', False)),
            })
        return [p for p in out if p['name']]

    return []


def write_workflow_manifest(site_dir: Path, site: str, site_scripts: list[Path]):
    workflow_toml = site_dir / 'workflow.toml'
    if workflow_toml.exists():
        return

    payloads = [load_yaml(path) for path in site_scripts]
    domains = []
    for payload in payloads:
        domain = extract_domain(payload, site)
        if domain and domain not in domains:
            domains.append(domain)

    strategies = [extract_strategy(payload) for payload in payloads]
    site_strategy = next((strategy for strategy in strategies if strategy != 'PUBLIC'), 'PUBLIC')
    login_required = 'true' if site_strategy != 'PUBLIC' else 'false'

    lines = [
        'schema_version = 1',
        f'site = {toml_string(site)}',
        f'display_name = {toml_string(titleize(site))}',
        'version = "0.1.0"',
        '',
        '[runtime]',
        'workflow_api = "1"',
        'min_hub_version = "0.1.0"',
        'min_agent_browser_version = "0.1.0"',
        '',
        '[package]',
        'kind = "standard"',
        'default_enabled = true',
        '',
        '[auth]',
        f'strategy = {toml_string(site_strategy)}',
        f'login_required = {login_required}',
        f'domains = [{", ".join(toml_string(domain) for domain in domains)}]' if domains else 'domains = []',
        '',
        '[ui]',
        f'tags = [{toml_string(site)}]',
        '',
        '[commands]',
        'include = []',
        '',
    ]
    workflow_toml.write_text('\n'.join(lines), encoding='utf-8')


def write_command_manifest(site_dir: Path, site: str, script_path: Path):
    payload = load_yaml(script_path)
    command_name = script_path.stem
    command_path = site_dir / 'commands' / f'{command_name}.toml'
    if command_path.exists():
        return

    description = extract_description(payload)
    strategy = extract_strategy(payload)
    params = extract_params(payload)
    columns = payload.get('columns') if isinstance(payload.get('columns'), list) else []
    rel_script = f'scripts/{command_name}.json'

    lines = [
        f'name = {toml_string(command_name)}',
        f'description = {toml_string(description)}',
        f'strategy = {toml_string(strategy)}',
        '',
    ]
    for param in params:
        lines.extend([
            '[[params]]',
            f'name = {toml_string(param["name"])}',
            f'type = {toml_string(param["type"])}',
            f'required = {str(param["required"]).lower()}',
            f'positional = {str(param["positional"]).lower()}',
            f'description = {toml_string(param["description"])}',
        ])
        if param['default'] is not None:
            lines.append(f'default = {toml_value(param["default"])}')
        lines.append('')

    lines.extend([
        '[execution]',
        'entry = "script"',
        f'script = {toml_string(rel_script)}',
        '',
        '[output]',
        f'columns = [{", ".join(toml_string(str(column)) for column in columns)}]' if columns else 'columns = []',
        '',
    ])
    command_path.write_text('\n'.join(lines), encoding='utf-8')


def write_script_asset(site_dir: Path, script_path: Path):
    payload = load_yaml(script_path)
    scripts_dir = site_dir / 'scripts'
    scripts_dir.mkdir(parents=True, exist_ok=True)
    script_asset_path = scripts_dir / f'{script_path.stem}.json'
    if script_asset_path.exists():
        return
    script_asset_path.write_text(
        json.dumps(payload, ensure_ascii=False, indent=2) + '\n',
        encoding='utf-8',
    )


def sync_include_list(site_dir: Path):
    commands_dir = site_dir / 'commands'
    names = sorted(path.stem for path in commands_dir.glob('*.toml'))
    workflow_toml = site_dir / 'workflow.toml'
    text = workflow_toml.read_text(encoding='utf-8')
    lines = []
    replaced = False
    for line in text.splitlines():
        if line.startswith('include = '):
            lines.append('include = [{}]'.format(', '.join(toml_string(name) for name in names)))
            replaced = True
        else:
            lines.append(line)
    if not replaced:
        if lines and lines[-1] != '':
            lines.append('')
        lines.extend(['[commands]', 'include = [{}]'.format(', '.join(toml_string(name) for name in names))])
    workflow_toml.write_text('\n'.join(lines) + '\n', encoding='utf-8')


def main():
    generated_sites = 0
    generated_commands = 0
    for site_path in sorted(path for path in SCRIPTS_DIR.iterdir() if path.is_dir()):
        site = site_path.name
        script_files = sorted(site_path.glob('*.yaml'))
        if not script_files:
            continue
        site_dir = WORKFLOWS_DIR / site
        (site_dir / 'commands').mkdir(parents=True, exist_ok=True)
        created_workflow = not (site_dir / 'workflow.toml').exists()
        write_workflow_manifest(site_dir, site, script_files)
        if created_workflow:
            generated_sites += 1
        before = len(list((site_dir / 'commands').glob('*.toml')))
        for script_path in script_files:
            write_script_asset(site_dir, script_path)
            write_command_manifest(site_dir, site, script_path)
        after = len(list((site_dir / 'commands').glob('*.toml')))
        generated_commands += max(after - before, 0)
        sync_include_list(site_dir)

    print(f'generated_sites={generated_sites}')
    print(f'generated_commands={generated_commands}')


if __name__ == '__main__':
    main()
