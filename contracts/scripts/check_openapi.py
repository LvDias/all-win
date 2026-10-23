# Structural self-check of contracts/http/openapi.yaml. Not a full OpenAPI validator.
# - every $ref resolves (local pointers and external files);
# - operationIds are unique;
# - every schema is valid JSON Schema 2020-12: components and inline (parameters, headers, bodies);
# - every example validates against its schema (operations and components/responses);
# - contract conventions: root session security, 401 on session operations, 500 everywhere,
#   400 where there is input, 403 on state-changing /api/v1 operations and on x-step-up operations,
#   x-step-up values, ingress has no 400/401/403.
# Usage: python3 contracts/scripts/check_openapi.py contracts/http/openapi.yaml
import os
import sys

import yaml
from jsonschema import Draft202012Validator

ROOT_URI = 'https://allwin.app/contracts/http/openapi.yaml'
METHODS = ('get', 'put', 'post', 'patch', 'delete')
STATE_CHANGING = ('put', 'post', 'patch', 'delete')


def make_validator_factory(doc):
    root = {'components': doc['components']}
    try:  # jsonschema >= 4.18
        from referencing import Registry, Resource
        from referencing.jsonschema import DRAFT202012
        registry = Registry().with_resource(ROOT_URI, Resource(contents=root, specification=DRAFT202012))
        return lambda schema: Draft202012Validator({'$id': ROOT_URI, **schema}, registry=registry)
    except ImportError:  # older jsonschema: RefResolver is the only option and is not deprecated there
        from jsonschema import RefResolver
        resolver = RefResolver(ROOT_URI, root)
        return lambda schema: Draft202012Validator(schema, resolver=resolver)


def check_refs(doc, base, errs):
    def walk(o, path):
        if isinstance(o, dict):
            for k, v in o.items():
                if k == '$ref' and isinstance(v, str):
                    if v.startswith('#/'):
                        cur = doc
                        for part in v[2:].split('/'):
                            if not isinstance(cur, dict) or part not in cur:
                                errs.append(f'unresolved {v} at {path}')
                                break
                            cur = cur[part]
                    elif not os.path.exists(os.path.normpath(os.path.join(base, v.split('#')[0]))):
                        errs.append(f'missing file {v} at {path}')
                walk(v, f'{path}/{k}')
        elif isinstance(o, list):
            for i, v in enumerate(o):
                walk(v, f'{path}/{i}')
    walk(doc, '')


def resolve(doc, obj):
    """Follow a local $ref (responses, parameters) to the referenced object."""
    while isinstance(obj, dict) and '$ref' in obj and obj['$ref'].startswith('#/'):
        cur = doc
        for part in obj['$ref'][2:].split('/'):
            cur = cur[part]
        obj = cur
    return obj


def operations(doc):
    for pth, item in doc['paths'].items():
        for m in METHODS:
            if m in item:
                yield pth, m, item[m]


def main(p):
    doc = yaml.safe_load(open(p))
    base = os.path.dirname(os.path.abspath(p))
    errs = []
    if not str(doc.get('openapi', '')).startswith('3.1'):
        errs.append('openapi must be 3.1.x')
    check_refs(doc, base, errs)
    validator = make_validator_factory(doc)

    ops = list(operations(doc))
    ids = [op.get('operationId') for _, _, op in ops]
    if None in ids:
        errs.append('operation without operationId')
    dup = {i for i in ids if ids.count(i) > 1}
    if dup:
        errs.append(f'duplicate operationIds {sorted(dup)}')

    # Collect every schema (components + inline) and every example.
    schemas = [(f'components/schemas/{n}', s) for n, s in doc['components']['schemas'].items()]
    examples = []

    def content_of(where, content):
        for ct, c in (content or {}).items():
            if 'schema' in c:
                schemas.append((f'{where} {ct}', c['schema']))
                for name, ex in (c.get('examples') or {}).items():
                    examples.append((f'{where} {ct} example {name}', c['schema'], ex['value']))

    def headers_of(where, headers):
        for h, hd in (headers or {}).items():
            hd = resolve(doc, hd)
            if 'schema' in hd:
                schemas.append((f'{where} header {h}', hd['schema']))

    for n, r in doc['components'].get('responses', {}).items():
        content_of(f'components/responses/{n}', r.get('content'))
        headers_of(f'components/responses/{n}', r.get('headers'))
    for n, h in doc['components'].get('headers', {}).items():
        schemas.append((f'components/headers/{n}', h['schema']))
    for n, prm in doc['components'].get('parameters', {}).items():
        schemas.append((f'components/parameters/{n}', prm['schema']))
    for pth, m, op in ops:
        where = f'{m.upper()} {pth}'
        for prm in op.get('parameters', []):
            prm = resolve(doc, prm)
            schemas.append((f'{where} param {prm["name"]}', prm['schema']))
        content_of(f'{where} request', (op.get('requestBody') or {}).get('content'))
        for code, r in op['responses'].items():
            if '$ref' not in r:  # referenced responses are checked once, above
                content_of(f'{where} {code}', r.get('content'))
                headers_of(f'{where} {code}', r.get('headers'))

    for where, s in schemas:
        try:
            Draft202012Validator.check_schema(s)
        except Exception as e:  # noqa: BLE001 - report any schema error
            errs.append(f'schema {where}: {getattr(e, "message", e)!s:.120}')
    for where, s, value in examples:
        if '$ref' in s and not s['$ref'].startswith('#/'):
            continue  # external schema (instruction v1) has its own fixture suite
        e = next(iter(validator(s).iter_errors(value)), None)
        if e:
            errs.append(f'{where}: {e.message:.120}')

    # Conventions.
    if doc.get('security') != [{'session': []}]:
        errs.append('root security must be [{session: []}]')
    for pth, m, op in ops:
        where = f'{op.get("operationId")} ({m.upper()} {pth})'
        codes = set(op['responses'])
        sec = op.get('security')
        if pth.startswith('/ingress/'):
            if sec != []:
                errs.append(f'{where}: ingress must declare security: []')
            if codes & {'401', '403'} or '404' not in codes:
                errs.append(f'{where}: ingress must use the empty 404, never 401/403')
        else:
            if sec is None and '401' not in codes:
                errs.append(f'{where}: session operation without 401')
            if (op.get('requestBody') or op.get('parameters')) and '400' not in codes:
                errs.append(f'{where}: operation with input but no 400')
            if pth.startswith('/api/v1/') and m in STATE_CHANGING and '403' not in codes:
                errs.append(f'{where}: state-changing /api/v1 operation without 403 (Origin check)')
        if '500' not in codes:
            errs.append(f'{where}: no 500')
        step_up = op.get('x-step-up')
        if step_up is not None:
            if step_up not in (True, 'conditional'):
                errs.append(f'{where}: x-step-up must be true or "conditional"')
            if (step_up == 'conditional') != isinstance(op.get('x-step-up-when'), str):
                errs.append(f'{where}: x-step-up-when is required exactly when x-step-up is conditional')
            if '403' not in codes:
                errs.append(f'{where}: x-step-up operation without 403')

    print(f'operations: {len(ops)}, schemas checked: {len(schemas)}, examples checked: {len(examples)}')
    print('\n'.join(errs) if errs else 'OPENAPI CHECK OK')
    return 1 if errs else 0


if __name__ == '__main__':
    sys.exit(main(sys.argv[1]))
