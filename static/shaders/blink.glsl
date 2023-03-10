uniform float u_closed;

varying vec2 v_quad_pos;

#ifdef VERTEX_SHADER
attribute vec2 a_pos;

void main() {
    vec2 pos = a_pos * 2.0 - 1.0;
    v_quad_pos = pos / 1.3;
    v_quad_pos.y /= 1.0 - u_closed;
    gl_Position = vec4(pos, 0.0, 1.0);
}
#endif

#ifdef FRAGMENT_SHADER
void main() {
    float d = length(v_quad_pos) - 1.0;
    float alpha = d * 5.0;
    gl_FragColor = vec4(vec3(0.0), alpha);
}
#endif