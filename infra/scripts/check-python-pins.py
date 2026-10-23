#!/usr/bin/env python3
"""Fails when an installed package differs from the pin in infra/requirements-check.txt."""
import pathlib
import sys
from importlib import metadata

pins = pathlib.Path(__file__).resolve().parent.parent / "requirements-check.txt"
bad = []
for line in pins.read_text().splitlines():
    line = line.strip()
    if not line or line.startswith("#"):
        continue
    name, want = line.split("==")
    try:
        have = metadata.version(name)
    except metadata.PackageNotFoundError:
        have = "missing"
    if have != want:
        bad.append(f"{name}: installed {have}, pinned {want}")
if bad:
    print("python pins differ (pip install -r infra/requirements-check.txt):\n  " + "\n  ".join(bad), file=sys.stderr)
    sys.exit(1)
