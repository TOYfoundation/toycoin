## Rarity glow for an item: aura shell, glitter and (Starfire only)
## small lightning arcs. Attach as child of the item and call setup().
##   tier 0 Sand Grain  : faint rim, few dust motes
##   tier 1 Brass       : glint sweep
##   tier 2 Turquoise   : soft aura + twinkling sparkles
##   tier 3 Ruby Glow   : pulsing ember glow, dense glitter
##   tier 4 Starfire    : stardust rain + lightning arcs
class_name RarityVfx
extends Node3D

@export_range(0, 4) var tier := 0
@export var color := Color.WHITE
@export var radius := 0.5

var _arc_timer := 0.0
var _rng := RandomNumberGenerator.new()


static func for_item(item_id: String, item_radius: float = 0.5) -> RarityVfx:
	var v := RarityVfx.new()
	v.tier = NftItems.vfx_tier(item_id)
	v.color = NftItems.rarity_color(item_id)
	v.radius = item_radius
	return v


func _ready() -> void:
	_rng.seed = hash(name) + tier
	var shell := MeshInstance3D.new()
	var sphere := SphereMesh.new()
	sphere.radius = radius * 1.18
	sphere.height = radius * 2.36
	sphere.radial_segments = 32
	sphere.rings = 16
	shell.mesh = sphere
	shell.material_override = VfxLib.aura_material(color, tier)
	shell.cast_shadow = GeometryInstance3D.SHADOW_CASTING_SETTING_OFF
	add_child(shell)
	var amounts := [6, 16, 40, 90, 160]
	var glitter := VfxLib.make_glitter(color.lightened(0.25), amounts[tier], radius * 1.1)
	if tier == 4:
		glitter.gravity = Vector3(0, -0.35, 0)  # stardust rains down
		glitter.emission_sphere_radius = radius * 1.4
		glitter.position.y = radius * 0.8
	add_child(glitter)


func _process(delta: float) -> void:
	if tier < 4 or not VfxSettings.lightning_allowed():
		return
	_arc_timer -= delta
	if _arc_timer > 0.0:
		return
	_arc_timer = _rng.randf_range(0.9, 1.8)  # well below 3 flashes per second
	var a := Vector3(_rng.randf_range(-1, 1), _rng.randf_range(-0.6, 1), _rng.randf_range(-1, 1)).normalized() * radius * 1.15
	var b := Vector3(_rng.randf_range(-1, 1), _rng.randf_range(-0.6, 1), _rng.randf_range(-1, 1)).normalized() * radius * 1.15
	var bolt := LightningBolt.new()
	bolt.start_point = a
	bolt.end_point = b
	bolt.width = 0.025
	bolt.branches = 1
	bolt.jaggedness = 0.5
	bolt.lifetime = 0.22
	bolt.light_burst = false
	bolt.flash_override = 1  # tiny arcs, not a screen flash
	bolt.color = Color(0.6, 0.85, 1.0)
	add_child(bolt)
