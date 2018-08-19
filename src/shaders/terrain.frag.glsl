#version 400

uniform sampler2D tex_lava;

in vec2 tex_coord;

void main() {
    gl_FragColor = texture(tex_lava, tex_coord);
}
