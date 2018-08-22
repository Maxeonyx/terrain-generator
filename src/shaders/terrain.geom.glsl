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

void set_tex_coords(vec2 xy_lerp) {
	tex_coord[0].xy = xy_lerp * 5;
	tex_coord[1].xy = xy_lerp * 10;
	tex_coord[2].xy = xy_lerp * 15;
}

const float ash_level = 0.7;
const float ash_blend_range = 0.1;
const float lava_level = 0.2;
const float lava_blend_range = 0.1;

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

void main() {
	for (int i = 0; i < gl_in.length(); i++) {
		vec4 position = gl_in[i].gl_Position;

		vec2 heightmap_coord = position.xy/world_width;
		vec4 height_sample;
		if (which_heightmap == 0) {
			height_sample = texture(tex_heightmap, heightmap_coord);
		} else {
			height_sample = texture(tex_heightmap_2, heightmap_coord);
		}

		set_tex_coords(position.xy / world_width);
		set_tex_weights(height_sample.r);

		position.z = 20*height_sample.r;
		gl_Position = mvp_matrix * position;
		EmitVertex();
	}
	EndPrimitive();
}