## The big moments: minting ceremony, level-up pillar, full-caravan ring.
## Each spawns its effect under `parent`, cleans up after itself and
## follows VfxSettings (quality, flash protection).
class_name VfxMoments
extends RefCounted


## "Die Prägung": sand swirl -> seal of light descends -> glitter burst.
## Starfire (tier 4) adds a lightning strike into the seal.
## Duration about 3.5 s. Returns the root node of the effect.
static func mint_ceremony(parent: Node3D, at: Vector3, tier: int, color: Color) -> Node3D:
	var root := Node3D.new()
	root.name = "MintCeremony"
	root.position = at
	parent.add_child(root)

	# 1. Sand swirl rising around the character.
	var sand := CPUParticles3D.new()
	var quad := QuadMesh.new()
	quad.size = Vector2(0.09, 0.09)
	quad.material = VfxLib.glitter_material()
	sand.mesh = quad
	sand.amount = VfxSettings.particle_amount(220)
	sand.lifetime = 2.2
	sand.emission_shape = CPUParticles3D.EMISSION_SHAPE_RING
	sand.emission_ring_axis = Vector3.UP
	sand.emission_ring_radius = 1.4
	sand.emission_ring_inner_radius = 1.1
	sand.emission_ring_height = 0.1
	sand.gravity = Vector3(0, 1.1, 0)
	sand.tangential_accel_min = 2.5
	sand.tangential_accel_max = 4.0
	sand.radial_accel_min = -0.6
	sand.radial_accel_max = -0.3
	sand.color = Color(0.95, 0.82, 0.55, 0.9)
	sand.scale_amount_min = 0.4
	sand.scale_amount_max = 1.0
	sand.emitting = true
	root.add_child(sand)

	# 2. Seal of light descends from above.
	var seal := MeshInstance3D.new()
	var plane := PlaneMesh.new()
	plane.size = Vector2(2.6, 2.6)
	seal.mesh = plane
	var seal_mat := VfxLib.shader_material(VfxLib.SEAL_SHADER, {"seal_color": color, "fade": 0.0, "spin": 0.8})
	seal.material_override = seal_mat
	seal.position = Vector3(0, 3.2, 0)
	seal.cast_shadow = GeometryInstance3D.SHADOW_CASTING_SETTING_OFF
	root.add_child(seal)
	var tw := root.create_tween()
	tw.tween_method(func(v): seal_mat.set_shader_parameter("fade", v), 0.0, 1.0, 0.6)
	tw.parallel().tween_property(seal, "position:y", 0.05, 1.6).set_trans(Tween.TRANS_QUAD).set_ease(Tween.EASE_OUT)

	# 3. Impact: glitter burst (+ lightning for Starfire), then fade out.
	tw.tween_callback(func():
		sand.emitting = false
		var burst := VfxLib.make_glitter(color.lightened(0.3), 120 + 60 * tier, 0.4, true)
		burst.position = Vector3(0, 0.6, 0)
		root.add_child(burst)
		if tier >= 4 and VfxSettings.lightning_allowed():
			var bolt := LightningBolt.new()
			bolt.start_point = Vector3(0.3, 6.0, -0.2)
			bolt.end_point = Vector3(0, 0.1, 0)
			bolt.width = 0.1
			bolt.branches = 4
			bolt.lifetime = 0.45
			bolt.color = Color(0.6, 0.85, 1.0)
			root.add_child(bolt)
		var pillar := level_up_pillar(root, Vector3.ZERO, color, 1.2)
		pillar.scale = Vector3(1.4, 1.0, 1.4)
	)
	tw.tween_interval(1.2)
	tw.tween_method(func(v): seal_mat.set_shader_parameter("fade", v), 1.0, 0.0, 0.7)
	tw.tween_callback(root.queue_free).set_delay(0.6)
	return root


## Golden light pillar with rising glitter (level-up, master levels).
static func level_up_pillar(parent: Node3D, at: Vector3, color: Color, duration := 1.5) -> Node3D:
	var root := Node3D.new()
	root.name = "LevelUp"
	root.position = at
	parent.add_child(root)
	var pillar := MeshInstance3D.new()
	var cyl := CylinderMesh.new()
	cyl.top_radius = 0.55
	cyl.bottom_radius = 0.75
	cyl.height = 5.0
	cyl.cap_top = false
	cyl.cap_bottom = false
	cyl.radial_segments = 32
	pillar.mesh = cyl
	pillar.position.y = 2.5
	var mat := VfxLib.shader_material(VfxLib.PILLAR_SHADER, {"pillar_color": color, "fade": 0.0})
	pillar.material_override = mat
	pillar.cast_shadow = GeometryInstance3D.SHADOW_CASTING_SETTING_OFF
	root.add_child(pillar)
	var rise := VfxLib.make_glitter(color.lightened(0.35), 120, 0.6)
	rise.gravity = Vector3(0, 2.2, 0)
	rise.position.y = 0.3
	root.add_child(rise)
	var intensity := VfxSettings.flash_intensity()
	var tw := root.create_tween()
	tw.tween_method(func(v): mat.set_shader_parameter("fade", v), 0.0, intensity, 0.25)
	tw.tween_interval(duration * 0.5)
	tw.tween_method(func(v): mat.set_shader_parameter("fade", v), intensity, 0.0, duration * 0.5)
	tw.tween_callback(func(): rise.emitting = false)
	tw.tween_callback(root.queue_free).set_delay(1.6)
	return root


## Full caravan: a golden lightning ring links all gang members.
## Falls back to soft glow lines when flashes are limited or off.
static func caravan_ring(parent: Node3D, members: Array, color := Color(1.0, 0.8, 0.3)) -> void:
	var bright := VfxSettings.request_flash()  # the whole ring counts as one flash
	for i in members.size():
		var a: Vector3 = members[i]
		var b: Vector3 = members[(i + 1) % members.size()]
		var bolt := LightningBolt.new()
		bolt.start_point = a + Vector3(0, 1.0, 0)
		bolt.end_point = b + Vector3(0, 1.0, 0)
		bolt.width = 0.05
		bolt.branches = 1
		bolt.jaggedness = 0.18
		bolt.lifetime = 1.6
		bolt.light_burst = i == 0
		bolt.flash_override = 1 if bright else 0
		bolt.color = color
		bolt.seed_value = 1000 + i
		parent.add_child(bolt)
