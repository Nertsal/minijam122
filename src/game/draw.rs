use super::*;

impl Game {
    pub fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        self.framebuffer_size = framebuffer.size();
        ugli::clear(framebuffer, Some(Rgba::BLACK), Some(1.0), None);

        // Level
        self.draw_gltf(&self.assets.level, Mat4::identity(), framebuffer);

        // Players
        let matrix =
            Mat4::translate(self.player.position) * Mat4::scale_uniform(self.player.radius);
        self.draw_gltf(&self.assets.player, matrix, framebuffer);

        // Control
        match &self.control {
            Control::Disabled => {}
            Control::Direction => {
                let direction = self.screen_pos_to_move_dir(self.geng.window().mouse_pos());
                let angle = direction.arg();
                let matrix = Mat4::translate(self.player.position)
                    * Mat4::rotate_z(angle)
                    * Mat4::scale_uniform(Coord::new(0.2));
                self.draw_gltf(&self.assets.arrow, matrix, framebuffer);
            }
            Control::Power { direction, time } => {
                let angle = direction.arg();
                let power = Coord::new((1.0 - time.as_f32().cos()) * 2.5);
                let matrix = Mat4::translate(self.player.position)
                    * Mat4::rotate_z(angle)
                    * Mat4::scale(vec3(0.2 + power.as_f32() * 0.1, 0.2, 0.2).map(Coord::new));
                self.draw_gltf(&self.assets.arrow, matrix, framebuffer);
            }
        }
    }

    fn draw_gltf<'a>(
        &'a self,
        gltf: impl IntoIterator<Item = &'a Mesh>,
        matrix: Mat4<Coord>,
        framebuffer: &'a mut ugli::Framebuffer,
    ) {
        for mesh in gltf.into_iter() {
            let matrix = matrix.map(|x| x.as_f32()); // * mesh.transform;
            ugli::draw(
                framebuffer,
                &self.assets.shaders.gltf,
                ugli::DrawMode::Triangles,
                &mesh.data,
                (
                    mesh.material.uniforms(),
                    ugli::uniforms! {
                        u_model_matrix: matrix,
                        u_eye_pos: self.camera.eye_pos(),
                        u_light_dir: vec3(1.0, -2.0, 5.0),
                        u_light_color: Rgba::WHITE,
                        u_ambient_light_color: Rgba::WHITE,
                        u_ambient_light_intensity: 0.8,
                    },
                    geng::camera3d_uniforms(&self.camera, framebuffer.size().map(|x| x as f32)),
                ),
                ugli::DrawParameters {
                    depth_func: Some(ugli::DepthFunc::Less),
                    ..default()
                },
            );
            ugli::draw(
                framebuffer,
                &self.assets.shaders.gltf_outline,
                ugli::DrawMode::Triangles,
                &mesh.data,
                (
                    ugli::uniforms! {
                        u_model_matrix: matrix,
                    },
                    geng::camera3d_uniforms(&self.camera, framebuffer.size().map(|x| x as f32)),
                ),
                ugli::DrawParameters {
                    depth_func: Some(ugli::DepthFunc::Less),
                    cull_face: Some(ugli::CullFace::Front),
                    blend_mode: Some(ugli::BlendMode::default()),
                    ..default()
                },
            );
        }
    }
}
