#!/usr/bin/env python3
"""Verify vendored bytes, or fetch the exact pinned upstream files with --fetch."""
import argparse
import hashlib
import json
from pathlib import Path
import urllib.request

ROOT = Path(__file__).resolve().parent.parent

def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--fetch", action="store_true")
    args = parser.parse_args()
    manifest = json.loads((ROOT / "schema/sources.json").read_text())
    base = "https://raw.githubusercontent.com/openpotato/opent8/" + manifest["commit"] + "/"
    pending = {}
    for schema in manifest["schemas"]:
        for remote, local, digest in [
            (schema["path"], schema["local"], schema["sha256"]),
            (schema["license_path"], schema["license_local"], schema["license_sha256"]),
        ]:
            target = ROOT / local
            if args.fetch:
                with urllib.request.urlopen(base + remote, timeout=30) as response:
                    data = response.read()
            else:
                data = target.read_bytes()
            if hashlib.sha256(data).hexdigest() != digest:
                raise SystemExit("SHA-256 mismatch: " + local)
            pending[target] = data
    # Do not replace any files until all downloads pass checksum verification.
    if args.fetch:
        for target, data in pending.items():
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_bytes(data)
    print("Verified pinned schemas and upstream licenses.")

if __name__ == "__main__":
    main()
