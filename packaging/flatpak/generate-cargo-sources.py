#!/usr/bin/env python3
"""Writes cargo-sources.json for the Flatpak build from Cargo.lock.

flatpak-builder builds without network access, so every Rust crate listed in
Cargo.lock must be declared as a source with its checksum. Run this again
whenever Cargo.lock changes:

    python3 packaging/flatpak/generate-cargo-sources.py

It needs Python 3.11 or newer and no network access.
"""
import json
import sys
import tomllib
from pathlib import Path

here = Path(__file__).resolve().parent
lock = here.parent.parent / "Cargo.lock"
out = here / "cargo-sources.json"

with open(lock, "rb") as f:
    packages = tomllib.load(f)["package"]

sources = []
for p in sorted(packages, key=lambda p: (p["name"], p["version"])):
    source = p.get("source", "")
    if not source:
        continue  # the workspace's own crates
    if not source.startswith("registry+https://github.com/rust-lang/crates.io-index"):
        sys.exit(f"unsupported source for {p['name']}: {source} (only crates.io is handled)")
    name, version, checksum = p["name"], p["version"], p["checksum"]
    dest = f"cargo/vendor/{name}-{version}"
    sources.append({
        "type": "archive",
        "archive-type": "tar-gzip",
        "url": f"https://static.crates.io/crates/{name}/{name}-{version}.crate",
        "sha256": checksum,
        "dest": dest,
    })
    sources.append({
        "type": "inline",
        "contents": json.dumps({"package": checksum, "files": {}}),
        "dest": dest,
        "dest-filename": ".cargo-checksum.json",
    })

sources.append({
    "type": "inline",
    "contents": '[source.vendored-sources]\ndirectory = "cargo/vendor"\n\n'
                '[source.crates-io]\nreplace-with = "vendored-sources"\n',
    "dest": "cargo",
    "dest-filename": "config",
})

out.write_text(json.dumps(sources, indent=4) + "\n")
print(f"wrote {out} ({(len(sources) - 1) // 2} crates)")
