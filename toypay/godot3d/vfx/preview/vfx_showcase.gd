## Preview stage for the VFX pack. Builds a small night scene in code
## and saves a screenshot, then quits.
##
##   godot --path godot3d vfx/preview/vfx_showcase.tscn -- shot=lineup out=/tmp/shots
##   shots: lineup | mint | levelup | caravan     flash: normal | soft | off
##   quality: low | medium | high
extends Node3D

const EXAMPLES := ["flute", "camel", "boat", "goblet_gold", "lamp_wondrous"]

var _args := {"shot": "lineup", "out": "user://shots", "flash": "normal", "quality": "high"}


func _ready() -> void:
	for a in OS.get_cmdline_user_args():
		var kv := a.split("=", true, 1)
		if kv.size() == 2:
			_args[kv[0]] = kv[1]
	VfxSettings.flash_mode = {"normal": VfxSettings.FlashMode.NORMAL, "soft": VfxSettings.FlashMode.SOFT,
		"off": VfxSettings.FlashMode.OFF}.get(_args["flash"], VfxSettings.FlashMode.NORMAL)
	VfxSettings.quality = {"low": VfxSettings.Quality.LOW, "medium": VfxSettings.Quality.MEDIUM,
		"high": VfxSettings.Quality.HIGH}.get(_args["quality"], VfxSettings.Quality.HIGH)
	VfxSettings.reset_flash_history()
	NftItems.load_catalog()
	_build_stage()
	var wait := 2.0
	match _args["shot"]:
		"lineup":
			_lineup()
		"mint":
			_figure(Vector3.ZERO, Color(0.85, 0.75, 0.6))
			VfxMoments.mint_ceremony(self, Vector3.ZERO, 4, NftItems.rarity_color("lamp_wondrous"))
			_camera(Vector3(0, 2.6, 6.2), Vector3(0, 1.4, 0))
			wait = 1.72
		"levelup":
			_figure(Vector3.ZERO, Color(0.7, 0.75, 0.9))
			VfxMoments.level_up_pillar(self, Vector3.ZERO, Color(1.0, 0.8, 0.35))
			_camera(Vector3(0, 2.4, 6.0), Vector3(0, 1.8, 0))
			wait = 0.7
		"caravan":
			var colors := [Color(0.9, 0.5, 0.3), Color(0.4, 0.7, 0.9), Color(0.9, 0.8, 0.4), Color(0.5, 0.9, 0.6), Color(0.8, 0.5, 0.9)]
			var spots: Array = []
			for i in 5:
				var ang := TAU * i / 5.0 - PI / 2.0
				var pos := Vector3(cos(ang) * 2.2, 0, sin(ang) * 2.2)
				_figure(pos, colors[i])
				spots.append(pos)
			VfxMoments.caravan_ring(self, spots)
			_camera(Vector3(0, 4.8, 6.8), Vector3(0, 0.8, 0))
			wait = 0.35
	await get_tree().create_timer(wait).timeout
	await RenderingServer.frame_post_draw
	var out: String = _args["out"]
	DirAccess.make_dir_recursive_absolute(out)
	var path := "%s/vfx_%s_%s.png" % [out, _args["shot"], _args["flash"]]
	get_viewport().get_texture().get_image().save_png(path)
	print("saved ", path)
	get_tree().quit()


func _build_stage() -> void:
	var env := Environment.new()
	env.background_mode = Environment.BG_COLOR
	env.background_color = Color(0.006, 0.008, 0.028)
	env.ambient_light_source = Environment.AMBIENT_SOURCE_COLOR
	env.ambient_light_color = Color(0.35, 0.38, 0.55)
	env.ambient_light_energy = 0.6
	env.glow_enabled = true
	env.glow_intensity = 1.0
	env.glow_bloom = 0.15
	env.tonemap_mode = Environment.TONE_MAPPER_ACES
	var we := WorldEnvironment.new()
	we.environment = env
	add_child(we)
	var moon := DirectionalLight3D.new()
	moon.light_color = Color(0.6, 0.7, 1.0)
	moon.light_energy = 0.5
	moon.rotation_degrees = Vector3(-50, 30, 0)
	add_child(moon)
	var ground := MeshInstance3D.new()
	var plane := PlaneMesh.new()
	plane.size = Vector2(40, 40)
	ground.mesh = plane
	var sand := StandardMaterial3D.new()
	sand.albedo_color = Color(0.36, 0.29, 0.2)
	sand.roughness = 1.0
	ground.material_override = sand
	add_child(ground)
	# A few stars in the sky.
	var stars := VfxLib.make_glitter(Color(0.8, 0.85, 1.0), 120, 14.0)
	stars.position = Vector3(0, 16, -14)
	stars.gravity = Vector3.ZERO
	stars.initial_velocity_max = 0.0
	stars.initial_velocity_min = 0.0
	stars.lifetime = 4.0
	stars.preprocess = 4.0
	add_child(stars)


func _lineup() -> void:
	var meshes: Array = [SphereMesh.new(), PrismMesh.new(), TorusMesh.new(), BoxMesh.new(), TorusMesh.new()]
	for i in 5:
		var id: String = EXAMPLES[i]
		var x := -5.4 + i * 2.7
		var pedestal := MeshInstance3D.new()
		var cyl := CylinderMesh.new()
		cyl.top_radius = 0.55
		cyl.bottom_radius = 0.65
		cyl.height = 0.9
		pedestal.mesh = cyl
		var stone := StandardMaterial3D.new()
		stone.albedo_color = Color(0.55, 0.5, 0.45)
		pedestal.material_override = stone
		pedestal.position = Vector3(x, 0.45, 0)
		add_child(pedestal)
		var item := MeshInstance3D.new()
		item.mesh = meshes[i]
		item.scale = Vector3.ONE * 0.55
		var mat := StandardMaterial3D.new()
		mat.albedo_color = NftItems.rarity_color(id).darkened(0.25)
		mat.metallic = 0.7
		mat.roughness = 0.3
		mat.emission_enabled = true
		mat.emission = NftItems.rarity_color(id) * 0.25
		item.material_override = mat
		item.position = Vector3(x, 1.55, 0)
		item.rotation_degrees = Vector3(20, 25 * i, 10)
		add_child(item)
		var fx := RarityVfx.for_item(id, 0.55)
		fx.position = item.position
		add_child(fx)
		var label := Label3D.new()
		var info := NftItems.rarity_info(NftItems.rarity(id))
		label.text = "%s\n%s · %d TOY" % [info["de"], NftItems.display_name(id, "de"), NftItems.fee_toy(id)]
		label.font_size = 34
		label.pixel_size = 0.006
		label.outline_size = 10
		label.modulate = NftItems.rarity_color(id).lightened(0.2)
		label.position = Vector3(x, 2.65, 0)
		label.billboard = BaseMaterial3D.BILLBOARD_ENABLED
		add_child(label)
	_camera(Vector3(0, 2.5, 8.6), Vector3(0, 1.45, 0))


func _figure(at: Vector3, color: Color) -> MeshInstance3D:
	var body := MeshInstance3D.new()
	var cap := CapsuleMesh.new()
	cap.radius = 0.32
	cap.height = 1.7
	body.mesh = cap
	var m := StandardMaterial3D.new()
	m.albedo_color = color
	m.roughness = 0.6
	body.material_override = m
	body.position = at + Vector3(0, 0.85, 0)
	add_child(body)
	return body


func _camera(pos: Vector3, target: Vector3) -> void:
	var cam := Camera3D.new()
	cam.fov = 55
	add_child(cam)
	cam.look_at_from_position(pos, target)
	cam.make_current()
