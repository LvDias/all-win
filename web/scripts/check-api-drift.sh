#!/usr/bin/env sh
# Fails when the committed generated API types differ from what the contract produces (CLAUDE.md §5).
set -eu
spec="${ALLWIN_OPENAPI:-../contracts/http/openapi.bundled.yaml}"
tmp="$(mktemp)"
trap 'rm -f "$tmp"' EXIT
pnpm exec openapi-typescript "$spec" -o "$tmp" >/dev/null
if ! diff -q "$tmp" src/shared/api/schema.gen.ts >/dev/null; then
  echo "src/shared/api/schema.gen.ts is out of date with $spec. Run: pnpm gen:api" >&2
  exit 1
fi
echo "api types match the contract"
