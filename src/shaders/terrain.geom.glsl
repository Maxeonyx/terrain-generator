#version 400

layout (triangles) in;
layout (triangle_strip, max_vertices=3) out;

uniform mat4 mvp_matrix;
uniform float world_width;
uniform sampler2D tex_heightmap;

void main() {
	int i;
	for (i = 0; i < gl_in.length(); i++) {
		vec4 position = gl_in[i].gl_Position;
		position.z = 20*texture(tex_heightmap, position.xy/world_width).r;
		gl_Position = mvp_matrix * position;
		EmitVertex();
	}
	EndPrimitive();
}