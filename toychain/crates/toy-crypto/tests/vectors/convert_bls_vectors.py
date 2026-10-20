#!/usr/bin/env python3
"""Convert the official Ethereum BLS test vectors (YAML) into one JSON file.

Source: https://github.com/ethereum/bls12-381-tests, release v0.1.2,
asset bls_tests_yaml.tar.gz (copied verbatim next to this script as
bls12-381-tests-v0.1.2.tar.gz). The conversion only changes the container
format (YAML -> JSON); every value is copied unchanged.

Usage: python3 convert_bls_vectors.py   (run inside this directory)
Needs PyYAML.
"""
import hashlib
import io
import json
import tarfile

import yaml

ARCHIVE = "bls12-381-tests-v0.1.2.tar.gz"
EXPECTED_SHA256 = "a4ef909b6248395094949a3bcdeb4e1b5b999733d4436f2c5006edba99d29279"
CATEGORIES = [
    "sign",
    "verify",
    "aggregate",
    "fast_aggregate_verify",
    "deserialization_G1",
    "deserialization_G2",
]

with open(ARCHIVE, "rb") as f:
    data = f.read()
digest = hashlib.sha256(data).hexdigest()
if digest != EXPECTED_SHA256:
    raise SystemExit(f"unexpected archive checksum {digest}")

out = {c: {} for c in CATEGORIES}
with tarfile.open(fileobj=io.BytesIO(data), mode="r:gz") as tar:
    for member in sorted(tar.getmembers(), key=lambda m: m.name):
        if not member.isfile() or not member.name.endswith(".yaml"):
            continue
        category, name = member.name.lstrip("./").split("/", 1)
        if category not in out:
            continue
        case = yaml.safe_load(tar.extractfile(member).read())
        out[category][name.removesuffix(".yaml")] = case

with open("bls12-381-tests-v0.1.2.json", "w") as f:
    json.dump(out, f, indent=1, sort_keys=True)
    f.write("\n")
print({c: len(v) for c, v in out.items()})
