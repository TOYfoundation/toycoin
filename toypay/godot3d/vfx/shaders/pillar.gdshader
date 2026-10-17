// Light pillar for level-ups: vertical gradient, rim glow, rising stripes.
shader_type spatial;
render_mode unshaded, blend_add, depth_draw_never, cull_disabled, shadows_disabled;

uniform vec4 pillar_color : source_color = vec4(1.0, 0.82, 0.35, 1.0);
uniform float fade = 1.0;

void fragment() {
	float up = UV.y;                             // CylinderMesh: UV.y = 1 at the bottom
	float rim = pow(1.0 - abs(dot(NORMAL, VIEW)), 2.0);
	float stripes = 0.65 + 0.35 * sin((UV.y * 18.0) - TIME * 9.0);
	float top_fade = smoothstep(0.0, 0.35, up);    // soft end towards the sky
	float a = top_fade * (0.3 + 0.7 * pow(up, 1.5)) * (0.35 + 0.65 * rim) * stripes * fade;
	ALBEDO = pillar_color.rgb * a * 2.4;
	ALPHA = clamp(a, 0.0, 1.0);
}
