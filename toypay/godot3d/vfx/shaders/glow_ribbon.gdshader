// Lightning ribbon: bright core across UV.x, soft falloff, fades out.
shader_type spatial;
render_mode unshaded, blend_add, depth_draw_never, cull_disabled, shadows_disabled;

uniform vec4 core_color : source_color = vec4(1.0, 1.0, 1.0, 1.0);
uniform vec4 glow_color : source_color = vec4(0.5, 0.8, 1.0, 1.0);
uniform float fade = 1.0;
uniform float core_width = 0.35;

void fragment() {
	float d = abs(UV.x * 2.0 - 1.0);
	float core = smoothstep(core_width, 0.0, d);
	float glow = pow(1.0 - d, 2.5);
	vec3 col = core_color.rgb * core * 2.0 + glow_color.rgb * glow;
	ALBEDO = col * fade;
	ALPHA = clamp((core + glow * 0.6) * fade, 0.0, 1.0);
}
