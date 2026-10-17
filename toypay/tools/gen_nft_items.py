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
EPITHETS = ROOT / "data" / "nft_epithets.tsv"
GANG = ROOT / "data" / "gang.json"
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
# (min_fee, max_fee, key, name_de, name_en, color, min_level, vfx_tier)
# Display names and colors: toypay/docs/07_SPIELDESIGN.md, section 7.
RARITIES = [
    (1, 4, "common", "Sandkorn", "Sand Grain", "#C8B48A", 1, 0),
    (5, 14, "uncommon", "Messing", "Brass", "#D9A441", 5, 1),
    (15, 39, "rare", "Türkis", "Turquoise", "#2EC4B6", 10, 2),
    (40, 79, "precious", "Rubinglut", "Ruby Glow", "#E0115F", 15, 3),
    (80, 100, "legendary", "Sternenfeuer", "Starfire", "#FFE9A8", 20, 4),
]
EPITHET_MIN_FEE = 40  # every precious and legendary item has an epithet
# Non-stat effect keys allowed in gang.json (handled by game logic).
SPECIAL_EFFECTS = {
    "travel_time_pct", "shop_price_pct", "riddle_hints", "boat_speed_pct",
    "wanted_decay_x", "xp_pct", "combat_night", "enemies_on_minimap",
    "special_merchant", "underwater_riddle_hints", "guard_distraction",
    "ambush_damage_x",
}

ID_RE = re.compile(r"^[a-z][a-z0-9_]{1,47}$")
HEADER = ["id", "kategorie", "de", "en", "gebuehr_toy", "slot", "boni"]


def rarity_for(fee: int) -> tuple:
    for row in RARITIES:
        if row[0] <= fee <= row[1]:
            return row
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
                "rarity": rarity_for(fee)[2],
                "min_level": rarity_for(fee)[6],
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


def load_epithets(items: list[dict], errors: list[str]) -> None:
    by_id = {i["id"]: i for i in items}
    lines = EPITHETS.read_text(encoding="utf-8").splitlines()
    if not lines or lines[0] != "id|beiname_de|beiname_en":
        errors.append("nft_epithets.tsv: header must be id|beiname_de|beiname_en")
        return
    seen = set()
    for lineno, line in enumerate(lines[1:], start=2):
        if not line.strip():
            continue
        cols = [c.strip() for c in line.split("|")]
        if len(cols) != 3 or not all(cols):
            errors.append(f"nft_epithets.tsv line {lineno}: need id, German and English epithet")
            continue
        item_id, de, en = cols
        if item_id not in by_id:
            errors.append(f"nft_epithets.tsv line {lineno}: unknown item '{item_id}'")
            continue
        if item_id in seen:
            errors.append(f"nft_epithets.tsv: duplicate '{item_id}'")
        seen.add(item_id)
        by_id[item_id].update(
            {"epithet_key": f"g_nftepi_{item_id}", "epithet_de": de, "epithet_en": en}
        )
    missing = [i["id"] for i in items if i["fee_toy"] >= EPITHET_MIN_FEE and i["id"] not in seen]
    if missing:
        errors.append(f"items from {EPITHET_MIN_FEE} TOY need an epithet: {', '.join(missing)}")


def load_gang(items: list[dict], errors: list[str]) -> dict:
    gang = json.loads(GANG.read_text(encoding="utf-8"))
    ids = {i["id"] for i in items}
    allowed = set(STATS) | SPECIAL_EFFECTS

    def check_effects(where: str, eff: dict) -> None:
        for k, v in eff.items():
            if k not in allowed:
                errors.append(f"gang.json {where}: unknown effect '{k}'")
            if not isinstance(v, (int, float)):
                errors.append(f"gang.json {where}: effect '{k}' must be a number")

    class_ids = set()
    for c in gang.get("classes", []):
        class_ids.add(c["id"])
        for s in c["stats"]:
            if s not in STATS:
                errors.append(f"gang.json class {c['id']}: unknown stat '{s}'")
        check_effects(f"class {c['id']}", c.get("aura", {}))
    if len(class_ids) != 8:
        errors.append(f"gang.json: expected 8 classes, found {len(class_ids)}")
    for d in gang.get("duos", []):
        if len(d["classes"]) != 2 or not set(d["classes"]) <= class_ids:
            errors.append(f"gang.json duo {d['id']}: needs two known classes")
        check_effects(f"duo {d['id']}", d.get("effect", {}))
    for s in gang.get("sets", []):
        unknown = [i for i in s["items"] if i not in ids]
        if unknown:
            errors.append(f"gang.json set {s['id']}: unknown items {unknown}")
        if len(s["items"]) < 2:
            errors.append(f"gang.json set {s['id']}: needs at least two items")
        check_effects(f"set {s['id']}", s.get("bonus", {}))
    for key in ("classes", "duos", "sets", "ranks"):
        names = [x["id"] for x in gang.get(key, [])]
        if len(names) != len(set(names)):
            errors.append(f"gang.json {key}: duplicate ids")
    return gang


def write(items: list[dict], gang: dict) -> None:
    payload = {
        "schema": 1,
        "generated_from": "toypay/data/nft_items.tsv",
        "rates": {"dukaten_per_toy": DUKATEN_PER_TOY, "gold_per_toy": GOLD_PER_TOY},
        "fee_bounds_toy": [FEE_MIN, FEE_MAX],
        "categories": {k: {"de": v[0], "en": v[1]} for k, v in CATEGORIES.items()},
        "stats": {k: {"de": v[0], "en": v[1]} for k, v in STATS.items()},
        "rarities": [
            {"key": key, "min_fee": lo, "max_fee": hi, "de": de, "en": en,
             "color": color, "min_level": lvl, "vfx_tier": tier}
            for lo, hi, key, de, en, color, lvl, tier in RARITIES
        ],
        "gang": gang,
        "items": items,
    }
    OUT_JSON.write_text(
        json.dumps(payload, ensure_ascii=False, indent=1) + "\n", encoding="utf-8"
    )
    rows = ["schluessel|deutsch|englisch"]
    rows += [f"g_nftcat_{k}|{de}|{en}" for k, (de, en) in CATEGORIES.items()]
    rows += [f"g_nftstat_{k}|{de}|{en}" for k, (de, en) in STATS.items()]
    rows += [f"g_nftrar_{r[2]}|{r[3]}|{r[4]}" for r in RARITIES]
    for kind in ("classes", "duos", "sets", "ranks"):
        rows += [f"g_gang_{kind[:-1] if kind != 'classes' else 'class'}_{x['id']}|{x['de']}|{x['en']}"
                 for x in gang.get(kind, [])]
    rows += [f"g_gang_trick_{c['id']}|{c['trick_de']}|{c['trick_en']}" for c in gang.get("classes", [])]
    rows += [f"{i['name_key']}|{i['name_de']}|{i['name_en']}" for i in items]
    rows += [f"{i['epithet_key']}|{i['epithet_de']}|{i['epithet_en']}" for i in items if "epithet_key" in i]
    OUT_I18N.write_text("\n".join(rows) + "\n", encoding="utf-8")


def report(items: list[dict]) -> None:
    print(f"{len(items)} items, fees {FEE_MIN}..{FEE_MAX} TOY")
    print(f"{'category':<14}{'count':>6}{'min':>6}{'max':>6}{'avg':>8}")
    for cat in CATEGORIES:
        fees = [i["fee_toy"] for i in items if i["category"] == cat]
        if fees:
            avg = sum(fees) / len(fees)
            print(f"{cat:<14}{len(fees):>6}{min(fees):>6}{max(fees):>6}{avg:>8.1f}")
    print(f"epithets: {sum(1 for i in items if 'epithet_key' in i)}")
    print("rarity: " + ", ".join(
        f"{k} {n}" for k, n in Counter(i["rarity"] for i in items).items()))


def main() -> int:
    items, errors = load()
    validate(items, errors)
    gang = {}
    if not errors:
        load_epithets(items, errors)
        gang = load_gang(items, errors)
    if errors:
        print("CATALOG INVALID:", file=sys.stderr)
        for e in errors:
            print(f"  - {e}", file=sys.stderr)
        return 1
    report(items)
    if "--check" not in sys.argv:
        write(items, gang)
        print(f"wrote {OUT_JSON.relative_to(ROOT.parent)} and {OUT_I18N.relative_to(ROOT.parent)}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
