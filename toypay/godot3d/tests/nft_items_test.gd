## Headless test for the NFT item catalog.
## Run: godot --headless --path godot3d tests/nft_items_test.tscn
extends Node

var _failures := 0


func _check(cond: bool, msg: String) -> void:
	if not cond:
		_failures += 1
		printerr("FAIL: ", msg)


func _ready() -> void:
	NftItems.unload()
	_check(NftItems.load_catalog(), "catalog loads")
	_check(NftItems.count() == 500, "500 items (got %d)" % NftItems.count())

	# Examples from decision 26 keep their fees.
	var examples := {"flute": 1, "dagger": 2, "camel": 5, "boat": 25, "house_beautiful": 100}
	for id in examples:
		_check(NftItems.has_item(id), "example %s exists" % id)
		_check(NftItems.fee_toy(id) == examples[id], "fee of %s" % id)

	# Every item: fee 1..100, Dukaten at the fixed rate, both names, bonuses.
	var seen_types := {}
	for item in NftItems.all_items():
		var id: String = item["id"]
		_check(item["fee_toy"] >= 1 and item["fee_toy"] <= 100, "fee range %s" % id)
		_check(item["fee_dukaten"] == item["fee_toy"] * NftItems.DUKATEN_PER_TOY, "dukaten %s" % id)
		_check(item["name_de"] != "" and item["name_en"] != "", "names %s" % id)
		_check(not item["bonuses"].is_empty(), "bonuses %s" % id)
		_check(not seen_types.has(item["item_type"]), "unique item_type %s" % id)
		seen_types[item["item_type"]] = true
		_check(NftItems.get_by_type(item["item_type"])["id"] == id, "type lookup %s" % id)

	# 16 categories, lookups, fallbacks.
	_check(NftItems.categories().size() == 16, "16 categories")
	_check(NftItems.items_in_category("instruments").size() == 30, "30 instruments")
	_check(NftItems.get_item("does_not_exist").is_empty(), "unknown id -> empty")
	_check(NftItems.display_name("camel", "de") == "Kamel", "German name")
	_check(NftItems.display_name("camel", "en") == "Camel", "English name")
	_check(NftItems.name_key("camel") == "g_nft_camel", "i18n key")

	# Mint requirements: 100 TOY minimum, or more for expensive items.
	var req_flute := NftItems.mint_requirements("flute")
	_check(req_flute["min_balance_toy"] == 100, "flute needs 100 TOY")
	var req_house := NftItems.mint_requirements("house_beautiful")
	_check(req_house["min_balance_toy"] == 110, "house needs 110 TOY")
	_check(req_house["lock_toy"] == 10 and req_house["lock_days"] == 20, "lock 10 TOY / 20 days")

	# Gang synergy sums shared item bonuses.
	var gang := NftItems.gang_bonus(["camel", "saddle_camel", "boots_riding"])
	_check(gang.get("ride", 0) == 5, "gang ride bonus 2+1+2 (got %s)" % gang.get("ride", 0))
	_check(NftItems.toy_to_dukaten(3) == 300, "3 TOY = 300 Dukaten")

	# Rarity names, epithets, level requirements (07_SPIELDESIGN.md).
	_check(NftItems.rarity_info("legendary").get("de", "") == "Sternenfeuer", "legendary = Sternenfeuer")
	_check(NftItems.rarity_info("common").get("en", "") == "Sand Grain", "common = Sand Grain")
	_check(NftItems.vfx_tier("lamp_wondrous") == 4 and NftItems.vfx_tier("flute") == 0, "vfx tiers")
	_check(NftItems.epithet("lamp_wondrous", "de") == "Dreimal reiben, einmal staunen", "epithet de")
	_check(NftItems.epithet("flute", "de") == "", "no epithet for cheap items")
	_check(NftItems.min_level("lamp_wondrous") == 20, "legendary needs level 20")
	_check(not NftItems.can_use("lamp_wondrous", 19), "level 19 cannot use legendary")
	_check(NftItems.can_use("flute", 1), "level 1 can use flute")
	_check(NftItems.max_supply("lamp_wondrous") == 1001, "Starfire edition 1001")
	_check(NftItems.max_supply("goblet_gold") == 0, "Ruby Glow: no lifetime cap")
	_check(NftItems.season_supply("goblet_gold") == 1001, "Ruby Glow 1001 per season")
	_check(NftItems.season_supply("lamp_wondrous") == 101, "Starfire 101 per season")
	_check(NftItems.season_supply("boat") == 0, "Turquoise unlimited")
	for item in NftItems.all_items():
		if item["fee_toy"] >= 40:
			_check(NftItems.epithet(item["id"], "en") != "", "epithet for %s" % item["id"])

	# Gang: 8 classes, diminishing returns, auras, duos.
	_check(NftItems.classes().size() == 8, "8 classes")
	var g1 := NftItems.gang_stats(["rooftop_fox", "shadow_quill"], ["camel", "saddle_camel", "boots_riding"])
	_check(g1["stats"].get("ride", 0) == 4, "ride 2+2*0.75+1*0.5 = 4 (got %s)" % g1["stats"].get("ride", 0))
	_check(g1["stats"].get("climb", 0) == 1, "rooftop fox aura climb +1")
	_check(g1["stats"].get("stealth", 0) == 2, "midnight duo stealth +2")
	_check(g1["duos"] == ["midnight_duo"], "midnight duo active")
	_check(not g1["full_caravan"], "two classes are no full caravan")

	# Completed set adds its bonus.
	var dawn := ["camel", "saddle_camel_carpet", "saddlebags_leather", "bells_camel"]
	var g2 := NftItems.gang_stats([], dawn)
	_check(g2["sets"] == ["dawn_caravan"], "dawn caravan set complete")
	_check(g2["stats"].get("ride", 0) == 6, "ride 4 from items + 2 set (got %s)" % g2["stats"].get("ride", 0))
	_check(g2["stats"].get("carry", 0) == 8, "carry 5 from items + 3 set (got %s)" % g2["stats"].get("carry", 0))
	var g2b := NftItems.gang_stats([], dawn.slice(0, 3))
	_check(g2b["sets"].is_empty(), "incomplete set gives no bonus")

	# Cap of +12 per stat from items.
	var blades := ["scimitar_moonlight", "scimitar_damascus", "scimitar_gold_hilt", "bow_roc_feather", "bow_composite", "bow_horn_lacquered"]
	_check(NftItems.gang_stats([], blades)["stats"].get("combat", 0) == 12, "combat capped at 12")

	# Full caravan: five different classes, +10 %, two duos.
	var five := ["rooftop_fox", "desert_bolt", "bazaar_tongue", "stargazer", "saber_whirl"]
	var g3 := NftItems.gang_stats(five, ["boots_riding", "camel_racing"])
	_check(g3["full_caravan"], "full caravan")
	_check(g3["duos"].size() == 2, "flying traders + star saber (got %s)" % [g3["duos"]])
	_check(g3["stats"].get("ride", 0) == 7, "ride round(round(4+2*0.75)*1.1) = 7 (got %s)" % g3["stats"].get("ride", 0))
	_check(g3["effects"].get("travel_time_pct", 0) == -15, "desert bolt aura in effects")

	# XP curve and gang names.
	_check(NftItems.xp_to_next(1) == 100, "xp to level 2")
	var xp_total := 0
	for lvl in range(1, 20):
		xp_total += NftItems.xp_to_next(lvl)
	_check(abs(xp_total - 86877) <= 2, "about 86,877 XP to level 20 (got %d)" % xp_total)
	var rng := RandomNumberGenerator.new()
	rng.seed = 7
	_check(NftItems.random_gang_name("de", rng).length() > 5, "gang name generator")

	if _failures == 0:
		print("nft_items_test: ALL OK (%d items)" % NftItems.count())
	else:
		printerr("nft_items_test: %d FAILURES" % _failures)
	get_tree().quit(1 if _failures > 0 else 0)
