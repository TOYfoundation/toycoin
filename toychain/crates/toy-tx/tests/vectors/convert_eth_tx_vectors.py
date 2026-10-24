#!/usr/bin/env python3
"""Converts ethereum/tests TransactionTests into one compact JSON file.

Usage: python3 convert_eth_tx_vectors.py <path to ethereum/tests checkout>

Only the format changes: for every test the raw transaction bytes and the
expected result of the newest fork in the file (Prague, else Cancun) are
kept: sender and hash for valid transactions, the exception name for
invalid ones. The SHA-256 of every source file is recorded, so the output
can be checked against a fresh checkout (commit c67e485f).

Transactions larger than 64 KB are listed under "omitted" with their
expected result instead of their bytes (they carry megabytes of call data;
ToyChain refuses call data and blocks above 128 KB anyway). This keeps the
file small enough for the repository.
"""
import hashlib
import json
import pathlib
import sys

root = pathlib.Path(sys.argv[1]) / "TransactionTests"
out = {"source": "ethereum/tests TransactionTests @ c67e485ff8b5be9abc8ad15345ec21aa22e290d9",
       "files": {}, "tests": [], "omitted": []}
LIMIT = 64 * 1024
for path in sorted(root.glob("*/*.json")):
    rel = str(path.relative_to(root))
    data = path.read_bytes()
    out["files"][rel] = hashlib.sha256(data).hexdigest()
    for name, test in sorted(json.loads(data).items()):
        result = test["result"].get("Prague") or test["result"]["Cancun"]
        entry = {"file": rel, "name": name, "txbytes": test["txbytes"]}
        if "exception" in result:
            entry["exception"] = result["exception"]
        else:
            entry["sender"] = result["sender"]
            entry["hash"] = result["hash"]
        if (len(test["txbytes"]) - 2) // 2 > LIMIT:
            entry["size"] = (len(entry.pop("txbytes")) - 2) // 2
            out["omitted"].append(entry)
        else:
            out["tests"].append(entry)
json.dump(out, sys.stdout, indent=1, sort_keys=True)
sys.stdout.write("\n")
