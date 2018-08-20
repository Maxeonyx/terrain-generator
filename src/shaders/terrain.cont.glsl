#version 400

layout(vertices = 4) out;

uniform vec3 camera_position;

void main() {
	gl_out[gl_InvocationID].gl_Position = gl_in[gl_InvocationID].gl_Position;

	vec4 avg_position = (gl_in[0].gl_Position + gl_in[1].gl_Position + gl_in[2].gl_Position + gl_in[3].gl_Position) / 4;

	float dist = distance(avg_position.xyz, camera_position);

	// distance      level
	// 200           1
	// 100           1
	// 50            10
	// 0             20
	int level = int(max((100 - dist)/5, 1));

	gl_TessLevelOuter[0] = level;
	gl_TessLevelOuter[1] = level;
	gl_TessLevelOuter[2] = level;
	gl_TessLevelOuter[3] = level;

	gl_TessLevelInner[0] = level;
	gl_TessLevelInner[1] = level;
}