#version 400

layout(vertices = 4) out;

uniform vec3 camera_position;

int level(float dist) {
	// distance      level
	// 200           1
	// 100           1
	// 50            10
	// 0             20
	return int(max((100 - dist)/5, 1));
}

void main() {
	gl_out[gl_InvocationID].gl_Position = gl_in[gl_InvocationID].gl_Position;
	
	// Calculate LOD for each edge of the patch
	int edge_level[4];
	{
		vec4 edge_pos = (gl_in[0].gl_Position + gl_in[1].gl_Position) / 2;
		float dist = distance(edge_pos.xyz, camera_position);
		edge_level[1] = level(dist);
	}
	{
		vec4 edge_pos = (gl_in[0].gl_Position + gl_in[2].gl_Position) / 2;
		float dist = distance(edge_pos.xyz, camera_position);
		edge_level[0] = level(dist);
	}
	{
		vec4 edge_pos = (gl_in[1].gl_Position + gl_in[3].gl_Position) / 2;
		float dist = distance(edge_pos.xyz, camera_position);
		edge_level[2] = level(dist);
	}
	{
		vec4 edge_pos = (gl_in[2].gl_Position + gl_in[3].gl_Position) / 2;
		float dist = distance(edge_pos.xyz, camera_position);
		edge_level[3] = level(dist);
	}

	// Calculate LOD for center of patch
	int avg_level;
	{
		vec4 avg_position = (gl_in[0].gl_Position + gl_in[1].gl_Position + gl_in[2].gl_Position + gl_in[3].gl_Position) / 4;
		float avg_dist = distance(avg_position.xyz, camera_position);
		avg_level = level(avg_dist);
	}

	// v = vertex
	// t = index in gl_TessLevelOuter
	//
	//          v3
	//          /\
	//         /  \
	//    t2  /    \  t3
	//       /      \
	//      /        \
	//  +y /  patch   \ +x
	//  v1 \          / v2
	//      \        /
	//       \      /
	//    t1  \    /  t0
	//         \  /
	//          \/
	//          v0
	//         (0,0)

	gl_TessLevelOuter[0] = edge_level[0];
	gl_TessLevelOuter[1] = edge_level[1];
	gl_TessLevelOuter[2] = edge_level[2];
	gl_TessLevelOuter[3] = edge_level[3];

	gl_TessLevelInner[0] = avg_level;
	gl_TessLevelInner[1] = avg_level;
}