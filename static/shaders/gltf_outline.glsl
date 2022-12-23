#ifdef VERTEX_SHADER

attribute vec3 a_pos;
attribute vec3 a_outline_normal;

uniform mat4 u_projection_matrix;
uniform mat4 u_view_matrix;
uniform mat4 u_model_matrix;

void main() {
    gl_Position = u_projection_matrix * u_view_matrix * u_model_matrix * vec4(a_pos + a_outline_normal * 0.03, 1.0);
}
#endif

#ifdef FRAGMENT_SHADER
void main() {
    gl_FragColor = vec4(0.0, 0.0, 0.0, 1.0);
}
#endif