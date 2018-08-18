#version 400

uniform sampler2D texture1;

void main() {
    gl_FragColor = texture(texture1, gl_FragCoord.xy/(1250/5));
}
