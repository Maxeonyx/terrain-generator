#version 400

layout (triangles) in;
layout (triangle_strip, max_vertices=3) out;

uniform mat4 mvp_matrix;
uniform float world_width;
uniform int which_heightmap;
uniform sampler2D tex_heightmap;
uniform sampler2D tex_heightmap_2;

const int N_TEX = 3;
out vec3[N_TEX] tex_coord;

out vec3 normal;

void set_tex_coords(vec2 xy_lerp) {
	tex_coord[0].xy = xy_lerp * 5;
	tex_coord[1].xy = xy_lerp * 10;
	tex_coord[2].xy = xy_lerp * 8;
}

const float height_scale = 20;

uniform float ash_level;
uniform float lava_level;
const float ash_blend_range = 0.15;
const float lava_blend_range = 0.03;

void set_tex_weights(float z) {
	if (z > ash_level) {
		tex_coord[0].z = 0;
		tex_coord[1].z = 0;
		tex_coord[2].z = 1;
	} else if (z <= ash_level && z > (ash_level - ash_blend_range)) {
		float lerp = (ash_level - z) / ash_blend_range;
		tex_coord[0].z = 0;
		tex_coord[1].z = lerp;
		tex_coord[2].z = 1 - lerp;
	} else if (z <= (ash_level - ash_blend_range) && z > (lava_level + lava_blend_range)) {
		tex_coord[0].z = 0;
		tex_coord[1].z = 1;
		tex_coord[2].z = 0;
	} else if (z <= (lava_level + lava_blend_range) && z > lava_level) {
		float lerp = (z - lava_level) / lava_blend_range;
		tex_coord[0].z = 1 - lerp;
		tex_coord[1].z = lerp;
		tex_coord[2].z = 0;
	} else {
		tex_coord[0].z = 1;
		tex_coord[1].z = 0;
		tex_coord[2].z = 0;
	}
}

float get_height(vec3 position) {

	vec2 heightmap_coord = position.xy/world_width;

	switch (which_heightmap) {
		case 0:
			return texture(tex_heightmap, heightmap_coord).r;
		default:
			return texture(tex_heightmap_2, heightmap_coord).r;
	}
}

vec3 calculate_normal(vec3 position) {

	float delta = world_width / 9 / 20 / 4;

	vec3 extra_point_1 = position + vec3(delta, 0, 0);
	extra_point_1.z = get_height(extra_point_1) * height_scale;

	vec3 extra_point_2 = position + vec3(0, delta, 0);
	extra_point_2.z = get_height(extra_point_2) * height_scale;

	return normalize(cross(position - extra_point_1, position - extra_point_2));
}

void main() {
	vec4 position[3];
	for (int i = 0; i < gl_in.length(); i++) {
		position[i] = gl_in[i].gl_Position;
		
		float height = get_height(position[i].xyz);

		height = max(height, lava_level);

		position[i].z = 20 * height;
	}

	for (int i = 0; i < gl_in.length(); i++) {

		set_tex_coords(position[i].xy / world_width);
		set_tex_weights(position[i].z / height_scale);

		normal = calculate_normal(position[i].xyz);

		gl_Position = mvp_matrix * position[i];
		EmitVertex();
	}
	EndPrimitive();
}