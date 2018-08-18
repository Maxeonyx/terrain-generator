#version 400

uniform sampler2D tex_lava;

void main() {
    gl_FragColor = texture(tex_lava, gl_FragCoord.xy/(1250/5));
}
