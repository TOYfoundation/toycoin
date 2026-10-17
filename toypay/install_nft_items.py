#!/usr/bin/env python3
"""Install the NFT item catalog (500 items) into a TOYPAY project.

Usage:
    python3 install_nft_items.py /path/to/TOYPAY

Same conventions as install_v6.py ... install_v8.py:
- Every file that would be replaced is first copied to
  TOYPAY/_backup_nft_<timestamp>/ (same relative path).
- Safe to run several times: identical files are skipped.
- Only new files are added; no existing game script is edited.

Copied files:
    godot3d/data/nft_items.json          catalog + gang rules for the 3D game
    godot3d/vfx/**                       VFX pack: rarity glow, glitter, lightning,
                                         minting ceremony, flash protection
    godot3d/scripts/nft_items.gd         loader (class_name NftItems)
    godot3d/tests/nft_items_test.gd/.tscn headless test
    tools/i18n/master_nftitems.tsv       names (key|de|en) for the i18n pipeline
    server/data/nft_items.json           same catalog for the game server
                                         (only if server/ exists)
"""

from __future__ import annotations

import filecmp
import shutil
import subprocess
import sys
import time
from pathlib import Path

HERE = Path(__file__).resolve().parent  # toycoin/toypay/

FILES = [
    ("data/nft_items.json", "godot3d/data/nft_items.json"),
    ("godot3d/scripts/nft_items.gd", "godot3d/scripts/nft_items.gd"),
    ("godot3d/tests/nft_items_test.gd", "godot3d/tests/nft_items_test.gd"),
    ("godot3d/tests/nft_items_test.tscn", "godot3d/tests/nft_items_test.tscn"),
    ("godot3d/tests/vfx_test.gd", "godot3d/tests/vfx_test.gd"),
    ("godot3d/tests/vfx_test.tscn", "godot3d/tests/vfx_test.tscn"),
    ("tools/i18n/master_nftitems.tsv", "tools/i18n/master_nftitems.tsv"),
]
# The whole VFX pack (scripts, shaders, preview scene).
FILES += [
    (str(p.relative_to(HERE)), str(p.relative_to(HERE)))
    for p in sorted((HERE / "godot3d" / "vfx").rglob("*"))
    if p.is_file() and p.suffix in {".gd", ".gdshader", ".tscn"}
]
OPTIONAL = [
    ("data/nft_items.json", "server/data/nft_items.json", "server"),
]


def main() -> int:
    if len(sys.argv) != 2:
        print(__doc__)
        return 2
    target = Path(sys.argv[1]).expanduser().resolve()
    if not (target / "godot3d" / "project.godot").is_file():
        print(f"Not a TOYPAY project (godot3d/project.godot missing): {target}")
        return 1

    # Regenerate and validate the catalog first; abort on any error.
    gen = HERE / "tools" / "gen_nft_items.py"
    if subprocess.run([sys.executable, "-I", str(gen)], check=False).returncode != 0:
        print("Catalog invalid, nothing installed.")
        return 1

    plan = list(FILES)
    for src, dst, needed_dir in OPTIONAL:
        if (target / needed_dir).is_dir():
            plan.append((src, dst))

    backup = target / f"_backup_nft_{time.strftime('%Y%m%d_%H%M%S')}"
    copied = skipped = backed_up = 0
    for src_rel, dst_rel in plan:
        src, dst = HERE / src_rel, target / dst_rel
        if dst.exists() and filecmp.cmp(src, dst, shallow=False):
            skipped += 1
            continue
        if dst.exists():
            (backup / dst_rel).parent.mkdir(parents=True, exist_ok=True)
            shutil.copy2(dst, backup / dst_rel)
            backed_up += 1
        dst.parent.mkdir(parents=True, exist_ok=True)
        shutil.copy2(src, dst)
        copied += 1
        print(f"  installed {dst_rel}")

    print(f"Done: {copied} installed, {skipped} unchanged, {backed_up} backed up"
          + (f" to {backup.name}/" if backed_up else ""))
    print("Next steps:")
    print("  1. godot --headless --path godot3d --import")
    print("  2. godot --headless --path godot3d tests/nft_items_test.tscn")
    print("     godot --headless --path godot3d tests/vfx_test.tscn")
    print("     Preview: godot --path godot3d vfx/preview/vfx_showcase.tscn -- shot=mint")
    print("  3. If tools/i18n/gen.py reads a fixed list of master files, add master_nftitems.tsv.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
