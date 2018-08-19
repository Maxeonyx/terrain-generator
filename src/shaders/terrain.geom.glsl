#version 400

layout (triangles) in;
layout (triangle_strip, max_vertices=3) out;

uniform mat4 mvp_matrix;
uniform float world_width;
uniform int which_heightmap;
uniform sampler2D tex_heightmap;
uniform sampler2D tex_heightmap_2;

out vec2 tex_coord;

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
		position.z = 20*height_sample.r;
		tex_coord = position.xy/world_width * 5;
		gl_Position = mvp_matrix * position;
		EmitVertex();
	}
	EndPrimitive();
}