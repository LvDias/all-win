# Self-check: every contracts/**/examples/valid fixture validates and every invalid one fails.
# Usage: python3 contracts/scripts/validate_fixtures.py contracts [-v]   (needs python3-jsonschema >= 4.0)
import json,sys,glob,os
from jsonschema import Draft202012Validator, RefResolver
root=sys.argv[1]  # contracts/
schemas={}
for p in glob.glob(f"{root}/**/*.schema.json",recursive=True):
    s=json.load(open(p)); Draft202012Validator.check_schema(s); schemas[p]=s
store={s["$id"]:s for s in schemas.values()}
ok=True
for p,s in sorted(schemas.items()):
    v=Draft202012Validator(s,resolver=RefResolver(s["$id"],s,store=store))
    d=os.path.dirname(p)
    va=sorted(glob.glob(f"{d}/examples/valid/*.json")); inv=sorted(glob.glob(f"{d}/examples/invalid/*.json"))
    for f in va:
        errs=list(v.iter_errors(json.load(open(f))))
        if errs: ok=False; print("FAIL valid rejected:",os.path.basename(f),"->",[e.message[:120] for e in errs][:2])
    for f in inv:
        errs=sorted(v.iter_errors(json.load(open(f))),key=lambda e:len(list(e.path)))
        if not errs: ok=False; print("FAIL invalid accepted:",os.path.basename(f))
        elif "-v" in sys.argv: print("  ",os.path.basename(f)[:40].ljust(41),"|".join(f"{'/'.join(map(str,e.path))}:{e.validator}" for e in errs)[:100])
    print(os.path.relpath(p,root),"valid:",len(va),"invalid:",len(inv))
print("ALL OK" if ok else "FAILURES"); sys.exit(0 if ok else 1)
