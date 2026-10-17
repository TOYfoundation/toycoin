// Seal of light for the minting ceremony: rotating rings, 12 rays and
// a star in the middle, drawn procedurally on a plane (no texture).
shader_type spatial;
render_mode unshaded, blend_add, depth_draw_never, cull_disabled, shadows_disabled;

uniform vec4 seal_color : source_color = vec4(1.0, 0.85, 0.4, 1.0);
uniform float fade = 1.0;
uniform float spin = 0.6;

float ring(float r, float center, float width) {
	return smoothstep(width, 0.0, abs(r - center));
}

void fragment() {
	vec2 p = UV * 2.0 - 1.0;
	float r = length(p);
	float ang = atan(p.y, p.x) + TIME * spin;
	float rays = pow(abs(cos(ang * 6.0)), 24.0) * smoothstep(0.92, 0.55, r) * smoothstep(0.18, 0.3, r);
	float rings = ring(r, 0.9, 0.035) + ring(r, 0.78, 0.02) + ring(r, 0.32, 0.03);
	float ticks = step(0.5, fract(ang * 6.0 / 3.14159 * 4.0)) * ring(r, 0.84, 0.03);
	float star = smoothstep(0.22, 0.0, r * (0.7 + 0.3 * abs(cos(ang * 4.0))));
	float a = (rings + rays * 0.8 + ticks * 0.6 + star) * smoothstep(1.0, 0.95, r) * fade;
	ALBEDO = seal_color.rgb * a * 1.5;
	ALPHA = clamp(a, 0.0, 1.0);
}
