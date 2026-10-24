# Preset releases

Published rule data for the presets catalog (V1-S16). Format: `contracts/presets/v1/release.schema.json`.

## Layout

```
data/presets/
  generators/<firm_snake>.py       # builds a firm's release files from its research file
  releases/<firm>/v<N>.json        # one file per published version
```

- One file per published version of a firm's catalog: `releases/<firm>/v1.json`, `v2.json`, …
  `<firm>` is the firm slug; `N` equals the file's `release.version_number`.
- A committed file is immutable. A correction or update is a new file `v<N+1>.json`, whose
  `release.changes` lists what changed against `v<N>` (the import checks this against its own diff).
- The publish CLI publishes a firm's files in version order. Publishing a version again with the
  same content does nothing; different content under a published version number is refused.
- Content comes only from the firm's research file (`docs/product/presets/research/<firm>.md`) and the
  user's decisions. Every file is approved by the PO (content) before it is committed.

## Generators

`python3 data/presets/generators/<firm_snake>.py v<N> data/presets/releases/<firm>/v<N>.json`

A generator rebuilds every version it has published: running it for an existing version must
reproduce the committed file byte for byte (check with `cmp`). Adding a version means adding its
arguments to the generator, not editing earlier output.
