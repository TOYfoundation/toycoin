## A single lightning bolt between two points: jagged main path
## (midpoint displacement), a few side branches, bright core plus glow,
## then a fast fade. Respects VfxSettings: without permission for a
## flash it shows a soft glow line instead and no light burst.
class_name LightningBolt
extends Node3D

@export var start_point := Vector3.ZERO
@export var end_point := Vector3(0, 3, 0)
@export var color := Color(0.55, 0.85, 1.0)
@export var width := 0.07
@export var lifetime := 0.35
@export var jaggedness := 0.35
@export var branches := 3
@export var light_burst := true
@export var seed_value := 0
## -1: ask VfxSettings (counts as one flash). 0: always soft. 1: bright if the
## flash mode is NORMAL, without using the flash budget (small arcs, or bolts
## of a group whose single flash was already requested).
@export var flash_override := -1

var _mesh_instance: MeshInstance3D
var _material: ShaderMaterial
var _age := 0.0
var _soft := false


func _ready() -> void:
	var rng := RandomNumberGenerator.new()
	rng.seed = seed_value if seed_value != 0 else randi()
	if VfxSettings.flash_mode == VfxSettings.FlashMode.OFF:
		queue_free()
		return
	if flash_override < 0:
		_soft = not VfxSettings.request_flash()
	else:
		_soft = flash_override == 0 or VfxSettings.flash_mode != VfxSettings.FlashMode.NORMAL
	_material = VfxLib.shader_material(VfxLib.RIBBON_SHADER, {
		"core_color": Color(1, 1, 1) if not _soft else color.lightened(0.3),
		"glow_color": color,
		"fade": 1.0,
		"core_width": 0.35 if not _soft else 0.15,
	})
	# SOFT mode: smoother, calmer path (fewer kinks) to read as a gentle glow.
	var depth := 6 if not _soft else 4
	var jag := jaggedness if not _soft else jaggedness * 0.5
	var paths: Array = [_jagged(start_point, end_point, depth, jag, rng)]
	var main: Array = paths[0]
	for i in branches:
		var idx := rng.randi_range(2, main.size() - 3)
		var from: Vector3 = main[idx]
		var dir := (end_point - start_point).normalized()
		var side := dir.cross(Vector3(rng.randf_range(-1, 1), rng.randf_range(-1, 1), rng.randf_range(-1, 1))).normalized()
		var to := from + (dir * 0.4 + side) * (end_point - start_point).length() * rng.randf_range(0.15, 0.3)
		paths.append(_jagged(from, to, 4, jaggedness * 0.7, rng))
	var mesh := ImmediateMesh.new()
	var cam := get_viewport().get_camera_3d()
	var w := width if not _soft else width * 2.5
	for k in paths.size():
		_add_ribbon(mesh, paths[k], w * (1.0 if k == 0 else 0.55), cam)
		_add_ribbon(mesh, paths[k], w * (5.0 if k == 0 else 3.0), cam)  # outer glow
	_mesh_instance = MeshInstance3D.new()
	_mesh_instance.mesh = mesh
	_mesh_instance.cast_shadow = GeometryInstance3D.SHADOW_CASTING_SETTING_OFF
	add_child(_mesh_instance)
	if light_burst and not _soft:
		var light := OmniLight3D.new()
		light.light_color = color
		light.light_energy = 6.0
		light.omni_range = 8.0
		light.position = end_point
		add_child(light)
		create_tween().tween_property(light, "light_energy", 0.0, lifetime * 0.6)


func _process(delta: float) -> void:
	if _material == null:
		return
	_age += delta
	var t := _age / lifetime
	# Short double flicker in NORMAL mode, smooth fade in SOFT mode.
	var f := 1.0 - t
	if not _soft and t < 0.4:
		f = 1.0 if fmod(t, 0.2) < 0.12 else 0.55
	_material.set_shader_parameter("fade", clampf(f, 0.0, 1.0) * VfxSettings.flash_intensity())
	if _age >= lifetime:
		queue_free()


func _jagged(a: Vector3, b: Vector3, depth: int, amount: float, rng: RandomNumberGenerator) -> Array:
	var pts: Array = [a, b]
	var offset := (b - a).length() * amount
	for d in depth:
		var next: Array = []
		for i in pts.size() - 1:
			var p0: Vector3 = pts[i]
			var p1: Vector3 = pts[i + 1]
			var mid := (p0 + p1) * 0.5
			var dir := (p1 - p0).normalized()
			var perp := dir.cross(Vector3(rng.randf_range(-1, 1), rng.randf_range(-1, 1), rng.randf_range(-1, 1))).normalized()
			next.append(p0)
			next.append(mid + perp * rng.randf_range(-offset, offset))
		next.append(pts[pts.size() - 1])
		pts = next
		offset *= 0.5
	return pts


func _add_ribbon(mesh: ImmediateMesh, pts: Array, w: float, cam: Camera3D) -> void:
	var view_pos := cam.global_position if cam != null else Vector3(0, 0, 10)
	# One side vector per point (averaged direction), so neighbouring
	# segments share their edges and the bolt has no gaps at the bends.
	var n := pts.size()
	var world: Array[Vector3] = []
	for p in pts:
		world.append(to_global(p))
	var sides: Array[Vector3] = []
	for i in n:
		var dir := (world[mini(i + 1, n - 1)] - world[maxi(i - 1, 0)]).normalized()
		var to_cam := (view_pos - world[i]).normalized()
		var taper := 1.0 - 0.45 * float(i) / float(n - 1)
		var side := dir.cross(to_cam).normalized() * w * 0.5 * taper
		if i > 0 and side.dot(sides[i - 1]) < 0.0:
			side = -side  # keep the ribbon from twisting at sharp kinks
		sides.append(side)
	mesh.surface_begin(Mesh.PRIMITIVE_TRIANGLES, _material)
	for i in n - 1:
		var a := to_local(world[i] - sides[i])
		var b := to_local(world[i] + sides[i])
		var c := to_local(world[i + 1] - sides[i + 1])
		var d := to_local(world[i + 1] + sides[i + 1])
		for v in [[a, Vector2(0, 0)], [b, Vector2(1, 0)], [c, Vector2(0, 1)], [b, Vector2(1, 0)], [d, Vector2(1, 1)], [c, Vector2(0, 1)]]:
			mesh.surface_set_uv(v[1])
			mesh.surface_add_vertex(v[0])
	mesh.surface_end()
