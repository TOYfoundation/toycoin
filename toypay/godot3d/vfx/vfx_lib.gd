## Shared building blocks for the TOYQUEST effects: procedural star
## sprite, glitter particles and shader materials. No external assets.
class_name VfxLib
extends RefCounted

const AURA_SHADER := preload("res://vfx/shaders/aura.gdshader")
const RIBBON_SHADER := preload("res://vfx/shaders/glow_ribbon.gdshader")
const PILLAR_SHADER := preload("res://vfx/shaders/pillar.gdshader")
const SEAL_SHADER := preload("res://vfx/shaders/seal.gdshader")

static var _star_texture: ImageTexture


## Four-pointed glitter star with a soft halo, generated once (64x64).
static func star_texture() -> ImageTexture:
	if _star_texture != null:
		return _star_texture
	var size := 64
	var img := Image.create(size, size, false, Image.FORMAT_RGBA8)
	var c := (size - 1) / 2.0
	for y in size:
		for x in size:
			var dx := (x - c) / c
			var dy := (y - c) / c
			var r := sqrt(dx * dx + dy * dy)
			var halo := pow(clampf(1.0 - r, 0.0, 1.0), 3.0)
			var spike_h := pow(clampf(1.0 - abs(dy) * 9.0, 0.0, 1.0), 2.0) * clampf(1.0 - abs(dx), 0.0, 1.0)
			var spike_v := pow(clampf(1.0 - abs(dx) * 9.0, 0.0, 1.0), 2.0) * clampf(1.0 - abs(dy), 0.0, 1.0)
			var v := clampf(halo + spike_h + spike_v, 0.0, 1.0)
			img.set_pixel(x, y, Color(1, 1, 1, v))
	_star_texture = ImageTexture.create_from_image(img)
	return _star_texture


## Additive, unshaded billboard material that takes the particle color.
static func glitter_material() -> StandardMaterial3D:
	var m := StandardMaterial3D.new()
	m.shading_mode = BaseMaterial3D.SHADING_MODE_UNSHADED
	m.blend_mode = BaseMaterial3D.BLEND_MODE_ADD
	m.transparency = BaseMaterial3D.TRANSPARENCY_ALPHA
	m.billboard_mode = BaseMaterial3D.BILLBOARD_PARTICLES
	m.vertex_color_use_as_albedo = true
	m.albedo_texture = star_texture()
	m.no_depth_test = false
	m.disable_receive_shadows = true
	return m


## Glitter particles in a color. `base_amount` is scaled by VfxSettings.
static func make_glitter(color: Color, base_amount: int, radius: float, one_shot := false) -> CPUParticles3D:
	var p := CPUParticles3D.new()
	var quad := QuadMesh.new()
	quad.size = Vector2(0.12, 0.12)
	quad.material = glitter_material()
	p.mesh = quad
	p.amount = VfxSettings.particle_amount(base_amount)
	p.lifetime = 1.6
	p.one_shot = one_shot
	p.explosiveness = 0.9 if one_shot else 0.0
	p.emission_shape = CPUParticles3D.EMISSION_SHAPE_SPHERE
	p.emission_sphere_radius = radius
	p.direction = Vector3.UP
	p.spread = 180.0 if one_shot else 35.0
	p.gravity = Vector3(0, 0.25, 0) if not one_shot else Vector3(0, -1.2, 0)
	p.initial_velocity_min = 0.1 if not one_shot else 1.4
	p.initial_velocity_max = 0.45 if not one_shot else 3.2
	p.scale_amount_min = 0.35
	p.scale_amount_max = 1.25
	var ramp := Gradient.new()
	ramp.set_color(0, Color(1, 1, 1, 0))
	ramp.set_color(1, Color(color.r, color.g, color.b, 0))
	ramp.add_point(0.15, Color(1, 1, 1, 1))
	ramp.add_point(0.55, Color(color.r, color.g, color.b, 0.9))
	p.color_ramp = ramp
	if not one_shot:
		p.preprocess = p.lifetime  # already sparkling when it appears
	p.emitting = true
	return p


static func aura_material(color: Color, tier: int) -> ShaderMaterial:
	var m := ShaderMaterial.new()
	m.shader = AURA_SHADER
	m.set_shader_parameter("aura_color", color)
	m.set_shader_parameter("intensity", 0.3 if tier == 0 else 0.6 + 0.35 * tier)
	m.set_shader_parameter("rim_power", 2.6 - 0.25 * tier)
	m.set_shader_parameter("sweep_strength", 0.9 if tier >= 1 else 0.0)
	m.set_shader_parameter("pulse_speed", 3.0 if tier >= 3 else 0.0)
	m.set_shader_parameter("sparkle_strength", 0.0 if tier < 2 else 0.6 + 0.4 * (tier - 2))
	return m


static func shader_material(shader: Shader, params: Dictionary) -> ShaderMaterial:
	var m := ShaderMaterial.new()
	m.shader = shader
	for k in params:
		m.set_shader_parameter(k, params[k])
	return m
