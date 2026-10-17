## Headless test for the VFX pack: flash protection and that every
## effect can be built without errors.
## Run: godot --headless --path godot3d tests/vfx_test.tscn
extends Node3D

var _failures := 0


func _check(cond: bool, msg: String) -> void:
	if not cond:
		_failures += 1
		printerr("FAIL: ", msg)


func _ready() -> void:
	# Flash protection: at most 3 flashes per second in NORMAL mode.
	VfxSettings.flash_mode = VfxSettings.FlashMode.NORMAL
	VfxSettings.reset_flash_history()
	var allowed := 0
	for i in 10:
		if VfxSettings.request_flash(1000 + i * 50):  # 10 requests within 0.5 s
			allowed += 1
	_check(allowed == 3, "only 3 of 10 flashes within a second (got %d)" % allowed)
	_check(VfxSettings.request_flash(2001), "a new second allows flashes again")

	# Sliding window: never more than 3 in any 1000 ms window.
	VfxSettings.reset_flash_history()
	var times: Array[int] = []
	for t in range(0, 5000, 90):
		if VfxSettings.request_flash(t):
			times.append(t)
	var worst := 0
	for i in times.size():
		var n := 0
		for j in range(i, times.size()):
			if times[j] - times[i] < 1000:
				n += 1
		worst = max(worst, n)
	_check(worst <= 3, "sliding window max 3 (got %d)" % worst)

	VfxSettings.flash_mode = VfxSettings.FlashMode.SOFT
	_check(not VfxSettings.request_flash(9000), "SOFT never flashes")
	_check(VfxSettings.flash_intensity() < 1.0, "SOFT dims glow")
	VfxSettings.flash_mode = VfxSettings.FlashMode.OFF
	_check(not VfxSettings.lightning_allowed(), "OFF: no lightning")

	# Quality budget.
	VfxSettings.quality = VfxSettings.Quality.LOW
	_check(VfxSettings.particle_amount(1000) == 64, "LOW caps at 64 particles")
	VfxSettings.quality = VfxSettings.Quality.HIGH
	_check(VfxSettings.particle_amount(1000) == 512, "HIGH caps at 512 particles")
	VfxSettings.flash_mode = VfxSettings.FlashMode.NORMAL
	_check(VfxSettings.lightning_allowed(), "HIGH + NORMAL allows lightning")

	# Every effect builds without script errors.
	NftItems.load_catalog()
	var cam := Camera3D.new()
	add_child(cam)
	for id in ["flute", "camel", "boat", "goblet_gold", "lamp_wondrous"]:
		var fx := RarityVfx.for_item(id)
		add_child(fx)
		_check(fx.get_child_count() >= 2, "rarity vfx for %s has aura and glitter" % id)
	_check(VfxLib.star_texture().get_width() == 64, "star texture 64 px")
	var mint := VfxMoments.mint_ceremony(self, Vector3.ZERO, 4, Color(1, 0.9, 0.6))
	_check(is_instance_valid(mint), "mint ceremony")
	var up := VfxMoments.level_up_pillar(self, Vector3(3, 0, 0), Color(1, 0.8, 0.3))
	_check(is_instance_valid(up), "level-up pillar")
	VfxMoments.caravan_ring(self, [Vector3(0, 0, 0), Vector3(2, 0, 0), Vector3(1, 0, 2)])
	await get_tree().create_timer(0.5).timeout
	await get_tree().create_timer(4.0).timeout
	_check(not is_instance_valid(mint), "mint ceremony cleans itself up")

	if _failures == 0:
		print("vfx_test: ALL OK")
	else:
		printerr("vfx_test: %d FAILURES" % _failures)
	get_tree().quit(1 if _failures > 0 else 0)
