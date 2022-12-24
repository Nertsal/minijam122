varying vec2 v_uv;

#ifdef VERTEX_SHADER
attribute vec2 a_pos;
void main() {
    v_uv = a_pos;
    gl_Position = vec4(a_pos * 2.0 - 1.0, 0.0, 1.0);
}
#endif

#ifdef FRAGMENT_SHADER
uniform sampler2D u_texture;
uniform vec2 u_texture_size;
void main() {
    vec4 pixel_color = texture2D(u_texture, v_uv);
    gl_FragColor = pixel_color;

    float outline_thickness = u_texture_size.y * 1.0 / 800.0;

    for (int x = -4; x <= 4; ++x) {
        for (int y = -4; y <= 4; ++y) {
            vec4 this_color = texture2D(u_texture, v_uv + vec2(x, y) / 4.0 * outline_thickness / u_texture_size);
            if (length(this_color - pixel_color) >= 0.15) {
                gl_FragColor = vec4(0.0, 0.0, 0.0, 1.0);
                return;
            }
        }
    }
}
#endif