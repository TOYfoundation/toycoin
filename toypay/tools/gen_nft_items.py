#!/usr/bin/env python3
"""Validate the NFT item catalog and generate game, server and i18n data.

Source of truth: toypay/data/nft_items.tsv (pipe-separated, like the
TOYPAY i18n tables). Rules come from docs/02-entscheidungen.md:
decision 26 (minting fee 1..100 TOY per item) and the fixed rate
1 TOY = 100 Dukaten = 1000 Gold (decision 23).

IMPORTANT: item_type numbers are the 1-based row order of the TSV.
They become on-chain identifiers, so rows must only ever be APPENDED,
never reordered or deleted (retire an item with a flag instead).

Usage:
    python3 toypay/tools/gen_nft_items.py          # validate + write
    python3 toypay/tools/gen_nft_items.py --check  # validate only
"""

from __future__ import annotations

import json
import re
import sys
from collections import Counter
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]  # toypay/
SRC = ROOT / "data" / "nft_items.tsv"
OUT_JSON = ROOT / "data" / "nft_items.json"
OUT_I18N = ROOT / "tools" / "i18n" / "master_nftitems.tsv"

EXPECTED_COUNT = 500
FEE_MIN, FEE_MAX = 1, 100  # TOY, enforced again by the NFT contract
TOY_DECIMALS = 18
DUKATEN_PER_TOY = 100
GOLD_PER_TOY = 1000

# Examples given by the project owner (decision 26); must keep these fees.
REQUIRED_EXAMPLES = {
    "flute": 1,
    "dagger": 2,
    "camel": 5,
    "boat": 25,
    "house_beautiful": 100,
}

CATEGORIES = {
    "clothing": ("Kleidung", "Clothing"),
    "accessories": ("Schmuck und Zubehör", "Jewelry and Accessories"),
    "instruments": ("Instrumente", "Instruments"),
    "tools": ("Werkzeuge", "Tools"),
    "weapons": ("Waffen", "Weapons"),
    "armor": ("Rüstung und Schilde", "Armor and Shields"),
    "animals": ("Tiere und Reittiere", "Animals and Mounts"),
    "vessels": ("Boote und Fahrzeuge", "Boats and Vehicles"),
    "buildings": ("Gebäude", "Buildings"),
    "furniture": ("Einrichtung", "Furnishings"),
    "household": ("Haushalt", "Household"),
    "trade": ("Handel und Markt", "Trade and Market"),
    "scholarship": ("Wissenschaft und Erfindungen", "Scholarship and Inventions"),
    "travel": ("Reise und Navigation", "Travel and Navigation"),
    "magic": ("Magie und Alchemie", "Magic and Alchemy"),
    "treasures": ("Schätze und Kunst", "Treasures and Art"),
}

SLOTS = {
    "head", "body", "feet", "hands", "jewelry", "trinket", "carry",
    "weapon", "offhand", "instrument", "tool", "mount", "companion",
    "livestock", "vessel", "vehicle", "building", "furnishing",
    "household", "trade", "scholarship", "travel", "magic", "treasure",
}

STATS = {
    "combat": ("Kampf", "Combat"),
    "defense": ("Schutz", "Defense"),
    "climb": ("Klettern", "Climbing"),
    "swim": ("Schwimmen", "Swimming"),
    "ride": ("Reiten", "Riding"),
    "stealth": ("Heimlichkeit", "Stealth"),
    "perception": ("Spürsinn", "Perception"),
    "stamina": ("Ausdauer", "Stamina"),
    "carry": ("Tragkraft", "Carrying"),
    "trade": ("Handel", "Trade"),
    "charm": ("Ausstrahlung", "Charm"),
    "lore": ("Wissen", "Lore"),
    "craft": ("Handwerk", "Craft"),
}
STAT_MIN, STAT_MAX = 1, 6

# Rarity derived from the fee, so it can never disagree with the price.
RARITIES = [
    (1, 4, "common", "Gewöhnlich", "Common"),
    (5, 14, "uncommon", "Ungewöhnlich", "Uncommon"),
    (15, 39, "rare", "Selten", "Rare"),
    (40, 79, "precious", "Kostbar", "Precious"),
    (80, 100, "legendary", "Legendär", "Legendary"),
]

ID_RE = re.compile(r"^[a-z][a-z0-9_]{1,47}$")
HEADER = ["id", "kategorie", "de", "en", "gebuehr_toy", "slot", "boni"]


def rarity_for(fee: int) -> str:
    for lo, hi, key, _de, _en in RARITIES:
        if lo <= fee <= hi:
            return key
    raise ValueError(f"fee {fee} outside rarity table")


def parse_bonuses(raw: str, where: str, errors: list[str]) -> dict[str, int]:
    bonuses: dict[str, int] = {}
    for part in raw.split(";"):
        if ":" not in part:
            errors.append(f"{where}: bad bonus '{part}'")
            continue
        stat, _, val = part.partition(":")
        if stat not in STATS:
            errors.append(f"{where}: unknown stat '{stat}'")
            continue
        if stat in bonuses:
            errors.append(f"{where}: duplicate stat '{stat}'")
        try:
            num = int(val)
        except ValueError:
            errors.append(f"{where}: bonus value '{val}' is not an integer")
            continue
        if not STAT_MIN <= num <= STAT_MAX:
            errors.append(f"{where}: bonus {stat}:{num} outside {STAT_MIN}..{STAT_MAX}")
        bonuses[stat] = num
    return bonuses


def load() -> tuple[list[dict], list[str]]:
    errors: list[str] = []
    lines = SRC.read_text(encoding="utf-8").splitlines()
    if not lines or lines[0].split("|") != HEADER:
        return [], [f"header must be: {'|'.join(HEADER)}"]
    items: list[dict] = []
    for lineno, line in enumerate(lines[1:], start=2):
        if not line.strip():
            continue
        cols = line.split("|")
        where = f"line {lineno}"
        if len(cols) != len(HEADER):
            errors.append(f"{where}: expected {len(HEADER)} columns, got {len(cols)}")
            continue
        item_id, cat, de, en, fee_raw, slot, boni = (c.strip() for c in cols)
        where = f"line {lineno} ({item_id})"
        if not ID_RE.match(item_id):
            errors.append(f"{where}: id must match {ID_RE.pattern}")
        if cat not in CATEGORIES:
            errors.append(f"{where}: unknown category '{cat}'")
        if slot not in SLOTS:
            errors.append(f"{where}: unknown slot '{slot}'")
        if not de or not en:
            errors.append(f"{where}: German and English names are required")
        try:
            fee = int(fee_raw)
        except ValueError:
            errors.append(f"{where}: fee '{fee_raw}' is not an integer")
            continue
        if not FEE_MIN <= fee <= FEE_MAX:
            errors.append(f"{where}: fee {fee} outside {FEE_MIN}..{FEE_MAX} TOY")
            continue
        items.append(
            {
                "item_type": len(items) + 1,
                "id": item_id,
                "category": cat,
                "name_key": f"g_nft_{item_id}",
                "name_de": de,
                "name_en": en,
                "slot": slot,
                "fee_toy": fee,
                "fee_wei": str(fee * 10**TOY_DECIMALS),
                "fee_dukaten": fee * DUKATEN_PER_TOY,
                "fee_gold": fee * GOLD_PER_TOY,
                "rarity": rarity_for(fee),
                "bonuses": parse_bonuses(boni, where, errors),
            }
        )
    return items, errors


def validate(items: list[dict], errors: list[str]) -> None:
    if len(items) != EXPECTED_COUNT:
        errors.append(f"expected {EXPECTED_COUNT} items, found {len(items)}")
    for field in ("id", "name_de", "name_en"):
        dupes = [k for k, n in Counter(i[field] for i in items).items() if n > 1]
        if dupes:
            errors.append(f"duplicate {field}: {', '.join(sorted(dupes))}")
    by_id = {i["id"]: i for i in items}
    for item_id, fee in REQUIRED_EXAMPLES.items():
        if item_id not in by_id:
            errors.append(f"required example '{item_id}' missing")
        elif by_id[item_id]["fee_toy"] != fee:
            errors.append(f"example '{item_id}' must cost {fee} TOY")


def write(items: list[dict]) -> None:
    payload = {
        "schema": 1,
        "generated_from": "toypay/data/nft_items.tsv",
        "rates": {"dukaten_per_toy": DUKATEN_PER_TOY, "gold_per_toy": GOLD_PER_TOY},
        "fee_bounds_toy": [FEE_MIN, FEE_MAX],
        "categories": {k: {"de": v[0], "en": v[1]} for k, v in CATEGORIES.items()},
        "stats": {k: {"de": v[0], "en": v[1]} for k, v in STATS.items()},
        "rarities": [
            {"key": key, "min_fee": lo, "max_fee": hi, "de": de, "en": en}
            for lo, hi, key, de, en in RARITIES
        ],
        "items": items,
    }
    OUT_JSON.write_text(
        json.dumps(payload, ensure_ascii=False, indent=1) + "\n", encoding="utf-8"
    )
    rows = ["schluessel|deutsch|englisch"]
    rows += [f"g_nftcat_{k}|{de}|{en}" for k, (de, en) in CATEGORIES.items()]
    rows += [f"g_nftstat_{k}|{de}|{en}" for k, (de, en) in STATS.items()]
    rows += [f"g_nftrar_{key}|{de}|{en}" for _lo, _hi, key, de, en in RARITIES]
    rows += [f"{i['name_key']}|{i['name_de']}|{i['name_en']}" for i in items]
    OUT_I18N.write_text("\n".join(rows) + "\n", encoding="utf-8")


def report(items: list[dict]) -> None:
    print(f"{len(items)} items, fees {FEE_MIN}..{FEE_MAX} TOY")
    print(f"{'category':<14}{'count':>6}{'min':>6}{'max':>6}{'avg':>8}")
    for cat in CATEGORIES:
        fees = [i["fee_toy"] for i in items if i["category"] == cat]
        if fees:
            avg = sum(fees) / len(fees)
            print(f"{cat:<14}{len(fees):>6}{min(fees):>6}{max(fees):>6}{avg:>8.1f}")
    print("rarity: " + ", ".join(
        f"{k} {n}" for k, n in Counter(i["rarity"] for i in items).items()))


def main() -> int:
    items, errors = load()
    validate(items, errors)
    if errors:
        print("CATALOG INVALID:", file=sys.stderr)
        for e in errors:
            print(f"  - {e}", file=sys.stderr)
        return 1
    report(items)
    if "--check" not in sys.argv:
        write(items)
        print(f"wrote {OUT_JSON.relative_to(ROOT.parent)} and {OUT_I18N.relative_to(ROOT.parent)}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
