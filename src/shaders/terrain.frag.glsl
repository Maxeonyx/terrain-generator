#version 400

const int N_TEX = 3;

uniform sampler2D tex_lava;
uniform sampler2D tex_lavarock;

// xy is texture coordinate. z is texture weight.
in vec3[N_TEX] tex_coord;

void main() {

    // take samples from each texture
    vec4 tex_sample[N_TEX];

    {
        tex_sample[0] = texture(tex_lava, tex_coord[0].xy);
        // Make lava a bit more orange
        tex_sample[0] += vec4(0, 0.08, 0, 0);
    }
    tex_sample[1] = texture(tex_lavarock, tex_coord[1].xy);
    tex_sample[2] = tex_sample[0].zyxw;

    // Add together the samples and multiply by the weights
    vec4 color = vec4(0.0);
    for (int i = 0; i < N_TEX; i ++) {
        color += tex_sample[i] * tex_coord[i].z;
    }
    gl_FragColor = color;
}
