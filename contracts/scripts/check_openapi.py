# Structural self-check of contracts/http/openapi.yaml: local/external $refs resolve, unique operationIds,
# component schemas valid (JSON Schema 2020-12), examples validate. Not a full OpenAPI validator.
# Usage: python3 contracts/scripts/check_openapi.py contracts/http/openapi.yaml
import sys, os, json, yaml
from jsonschema import Draft202012Validator, RefResolver
p=sys.argv[1]; base=os.path.dirname(os.path.abspath(p))
doc=yaml.safe_load(open(p))
errs=[]
assert doc['openapi'].startswith('3.1')
def walk(o,path=''):
    if isinstance(o,dict):
        for k,v in o.items():
            if k=='$ref':
                if v.startswith('#/'):
                    cur=doc
                    for part in v[2:].split('/'):
                        if part not in cur: errs.append(f'unresolved {v} at {path}'); break
                        cur=cur[part]
                else:
                    f=os.path.normpath(os.path.join(base,v.split('#')[0]))
                    if not os.path.exists(f): errs.append(f'missing file {v} at {path}')
            walk(v,f'{path}/{k}')
    elif isinstance(o,list):
        for i,v in enumerate(o): walk(v,f'{path}/{i}')
walk(doc)
# operationIds unique
ops=[op.get('operationId') for pth in doc['paths'].values() for m,op in pth.items() if isinstance(op,dict)]
dup={o for o in ops if ops.count(o)>1}
if dup: errs.append(f'duplicate operationIds {dup}')
# schemas valid
for n,s in doc['components']['schemas'].items():
    try: Draft202012Validator.check_schema(s)
    except Exception as e: errs.append(f'schema {n}: {e.message[:100]}')
# validate examples against response schema (local refs only)
store={}
inst=json.load(open(os.path.join(base,'../instruction/v1/instruction.schema.json')))
root={'components':doc['components'],'$id':'https://allwin.app/contracts/http/openapi.yaml'}
res=RefResolver('https://allwin.app/contracts/http/openapi.yaml',root,store={'https://allwin.app/contracts/http/openapi.yaml':root})
n=0
for pth,item in doc['paths'].items():
    for m,op in item.items():
        for code,r in op.get('responses',{}).items():
            for ct,c in (r.get('content') or {}).items():
                for ex in (c.get('examples') or {}).values():
                    sch=c['schema']; v=Draft202012Validator(sch,resolver=res)
                    e=list(v.iter_errors(ex['value'])); n+=1
                    if e: errs.append(f'example {pth} {code}: {e[0].message}')
print('operations:',len(ops),'examples checked:',n)
print('\n'.join(errs) if errs else 'OPENAPI CHECK OK')
sys.exit(1 if errs else 0)
