varying vec2 v_uv;
varying vec2 v_mr_uv;
varying vec4 v_color;
varying vec3 v_normal;
varying vec3 v_world_pos;

vec3 vec4_to_3(vec4 v) {
    return v.xyz / v.w;
}

#ifdef VERTEX_SHADER

attribute vec2 a_uv;
attribute vec2 a_mr_uv;
attribute vec3 a_pos;
attribute vec3 a_normal;
attribute vec4 a_color;

uniform mat4 u_projection_matrix;
uniform mat4 u_view_matrix;
uniform mat4 u_model_matrix;

void main() {
    v_uv = a_uv;
    v_mr_uv = a_mr_uv;
    v_color = pow(a_color, vec4(1.0 / 2.2));
    v_world_pos = vec4_to_3(u_model_matrix * vec4(a_pos, 1.0));
    v_normal = normalize(vec3(u_model_matrix * vec4(a_normal, 0.0)));
    gl_Position = u_projection_matrix * u_view_matrix * vec4(v_world_pos, 1.0);
}
#endif

#ifdef FRAGMENT_SHADER
void main() {
    //  * 0.9 + v_color.xyz * 0.1
    gl_FragColor = vec4((normalize(v_normal) * 0.5 + 0.5), gl_FragCoord.z);
}
#endif
