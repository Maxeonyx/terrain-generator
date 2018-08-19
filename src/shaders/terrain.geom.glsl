#version 400

layout (triangles) in;
layout (triangle_strip, max_vertices=3) out;

uniform mat4 mvp_matrix;
uniform float world_width;
uniform int which_heightmap;
uniform sampler2D tex_heightmap;
uniform sampler2D tex_heightmap_2;

const int N_TEX = 2;
out vec3[N_TEX] tex_coord;

void set_tex_coords(vec2 xy_lerp) {
	tex_coord[0].xy = xy_lerp * 5;
	tex_coord[1].xy = xy_lerp * 10;
}

void set_tex_weights(float z) {
	if (z < 0.5) {
		tex_coord[0].z = 1.0;
		tex_coord[1].z = 0.0;
	} else {
		tex_coord[0].z = 0.0;
		tex_coord[1].z = 1.0;
	}
}

void main() {
	int i;
	for (i = 0; i < gl_in.length(); i++) {
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