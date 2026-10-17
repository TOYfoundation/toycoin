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

	if _failures == 0:
		print("nft_items_test: ALL OK (%d items)" % NftItems.count())
	else:
		printerr("nft_items_test: %d FAILURES" % _failures)
	get_tree().quit(1 if _failures > 0 else 0)
