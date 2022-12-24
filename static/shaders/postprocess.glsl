varying vec2 v_uv;

#ifdef VERTEX_SHADER
attribute vec2 a_pos;
void main() {
    v_uv = a_pos;
    gl_Position = vec4(a_pos * 2.0 - 1.0, 0.0, 1.0);
}
#endif

#ifdef FRAGMENT_SHADER
uniform sampler2D u_outline_texture;
uniform sampler2D u_color_texture;
uniform vec2 u_outline_texture_size;
void main() {
    vec4 pixel_outline = texture2D(u_outline_texture, v_uv);
    gl_FragColor = texture2D(u_color_texture, v_uv);

    float outline_thickness = u_outline_texture_size.y * 1.0 / 800.0;

    vec4 sum = vec4(0.0);
    for (int x = -4; x <= 4; ++x) {
        for (int y = -4; y <= 4; ++y) {
            vec4 this_outline = texture2D(u_outline_texture, v_uv + vec2(x, y) / 4.0 * outline_thickness / u_outline_texture_size);
            sum += this_outline;
            if (dot(this_outline.xyz * 2.0 - 1.0, pixel_outline.xyz * 2.0 - 1.0) < 0.2
                || abs(this_outline.w - pixel_outline.w) >= 0.01) {
                gl_FragColor = vec4(0.0, 0.0, 0.0, 1.0);
                return;
            }
        }
    }
    sum /= 81.0;
}
#endif