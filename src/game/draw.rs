use super::*;

impl Game {
    pub fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        ugli::clear(framebuffer, Some(Rgba::BLACK), Some(1.0), None);
        let framebuffer_size = framebuffer.size().map(|x| x as f32);

        // Level
        for mesh in &self.assets.level {
            ugli::draw(
                framebuffer,
                &self.assets.shaders.gltf,
                ugli::DrawMode::Triangles,
                &mesh.data,
                (
                    mesh.material.uniforms(),
                    ugli::uniforms! {
                        u_model_matrix: Mat4::identity(),
                        u_eye_pos: self.camera.eye_pos(),
                        u_light_dir: vec3(1.0, -2.0, 5.0),
                        u_light_color: Rgba::WHITE,
                        u_ambient_light_color: Rgba::WHITE,
                        u_ambient_light_intensity: 0.1,
                    },
                    geng::camera3d_uniforms(&self.camera, framebuffer_size),
                ),
                ugli::DrawParameters {
                    depth_func: Some(ugli::DepthFunc::Less),
                    ..default()
                },
            );
        }

        // Players
        for mesh in &self.assets.player {
            let matrix =
                Mat4::translate(self.player.position) * Mat4::scale_uniform(self.player.radius);
            ugli::draw(
                framebuffer,
                &self.assets.shaders.gltf,
                ugli::DrawMode::Triangles,
                &mesh.data,
                (
                    mesh.material.uniforms(),
                    ugli::uniforms! {
                        u_model_matrix: matrix.map(|x| x.as_f32()),
                        u_eye_pos: self.camera.eye_pos(),
                        u_light_dir: vec3(1.0, -2.0, 5.0),
                        u_light_color: Rgba::WHITE,
                        u_ambient_light_color: Rgba::WHITE,
                        u_ambient_light_intensity: 0.1,
                    },
                    geng::camera3d_uniforms(&self.camera, framebuffer_size),
                ),
                ugli::DrawParameters {
                    depth_func: Some(ugli::DepthFunc::Less),
                    ..default()
                },
            )
        }
    }
}
