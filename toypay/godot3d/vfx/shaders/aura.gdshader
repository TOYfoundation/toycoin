// Rarity aura: fresnel rim glow, optional glint sweep (Brass),
// pulse (Ruby Glow) and twinkling sparkles (Turquoise and up).
// Additive and unshaded, works in Forward+, Mobile and Compatibility.
shader_type spatial;
render_mode unshaded, blend_add, depth_draw_never, cull_back, shadows_disabled;

uniform vec4 aura_color : source_color = vec4(1.0, 0.85, 0.4, 1.0);
uniform float intensity = 1.0;
uniform float rim_power = 2.2;
uniform float pulse_speed = 0.0;
uniform float sweep_strength = 0.0;
uniform float sparkle_strength = 0.0;
uniform float sparkle_density = 9.0;

varying vec3 obj_pos;

void vertex() {
	obj_pos = VERTEX;
}

float hash3(vec3 p) {
	p = fract(p * 0.3183099 + vec3(0.11, 0.17, 0.13));
	p *= 17.0;
	return fract(p.x * p.y * p.z * (p.x + p.y + p.z));
}

void fragment() {
	float fres = pow(1.0 - clamp(dot(NORMAL, VIEW), 0.0, 1.0), rim_power);
	float pulse = 1.0 + 0.35 * sin(TIME * pulse_speed) * step(0.001, pulse_speed);

	float band = 0.0;
	if (sweep_strength > 0.0) {
		float s = fract(TIME * 0.35) * 2.4 - 1.2;
		band = smoothstep(0.18, 0.0, abs(obj_pos.y - s)) * sweep_strength;
	}

	float sp = 0.0;
	if (sparkle_strength > 0.0) {
		vec3 cell = floor(obj_pos * sparkle_density);
		float h = hash3(cell);
		float twinkle = sin(TIME * 5.0 + h * 40.0) * 0.5 + 0.5;
		sp = step(0.9, h) * pow(twinkle, 6.0) * sparkle_strength;
	}

	vec3 col = aura_color.rgb * (fres * pulse + band) * intensity + vec3(1.0, 0.97, 0.9) * sp;
	ALBEDO = col;
	ALPHA = clamp(max(fres * pulse, max(band, sp)) * aura_color.a, 0.0, 1.0);
}
