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
        if !self.player.finished {
            match &self.control {
                Control::Disabled => {}
                Control::Direction => {
                    let direction = self.screen_pos_to_move_dir(self.geng.window().mouse_pos());
                    let angle = direction.arg();
                    let matrix = Mat4::translate(self.player.position)
                        * Mat4::rotate_z(angle)
                        * Mat4::scale_uniform(Coord::new(0.2));
                    self.draw_gltf(&self.assets.arrow, matrix, framebuffer);
                    let matrix = Mat4::translate(
                        self.player.position
                            - direction.extend(Coord::ZERO)
                                * (self.player.radius + Coord::new(0.2)),
                    ) * Mat4::rotate_z(angle)
                        * Mat4::scale_uniform(Coord::new(0.2));
                    self.draw_gltf(&self.assets.club, matrix, framebuffer);
                }
                Control::Power { direction, time } => {
                    let angle = direction.arg();
                    let power = Coord::new((1.0 - time.as_f32().cos()) * 2.5);
                    let matrix = Mat4::translate(self.player.position)
                        * Mat4::rotate_z(angle)
                        * Mat4::scale(vec3(0.2 + power.as_f32() * 0.1, 0.2, 0.2).map(Coord::new));
                    self.draw_gltf(&self.assets.arrow, matrix, framebuffer);
                    let matrix = Mat4::translate(
                        self.player.position
                            - direction.extend(Coord::ZERO)
                                * (self.player.radius + Coord::new(0.2 + power.as_f32() * 0.2)),
                    ) * Mat4::rotate_z(angle)
                        * Mat4::scale_uniform(Coord::new(0.2));
                    self.draw_gltf(&self.assets.club, matrix, framebuffer);
                }
                Control::Precision {
                    direction,
                    power,
                    time,
                } => {
                    let angle = (*time * Coord::new(3.0)).sin() * Coord::new(f32::PI / 12.0);
                    let direction = direction.rotate(angle);
                    let angle = direction.arg();
                    let power = *power * Coord::new(2.5 / 7.0);
                    let matrix = Mat4::translate(self.player.position)
                        * Mat4::rotate_z(angle)
                        * Mat4::scale(vec3(0.2 + power.as_f32() * 0.1, 0.2, 0.2).map(Coord::new));
                    self.draw_gltf(&self.assets.arrow, matrix, framebuffer);
                    let matrix = Mat4::translate(
                        self.player.position
                            - direction.extend(Coord::ZERO)
                                * (self.player.radius + Coord::new(0.2 + power.as_f32() * 0.2)),
                    ) * Mat4::rotate_z(angle)
                        * Mat4::scale_uniform(Coord::new(0.2));
                    self.draw_gltf(&self.assets.club, matrix, framebuffer);
                }
                Control::Hitting { time, hit } => {
                    let direction = hit.xy().normalize_or_zero();
                    let angle = direction.arg();
                    let power = hit.len() * Coord::new(2.5 / 7.0);
                    let far = Coord::new(0.2 + power.as_f32() * 0.2);
                    let b = 2.0;
                    let t = time.as_f32();
                    let t = -(t - 1.0) * (b * t + 1.0);
                    let matrix = Mat4::translate(
                        self.player.position
                            - direction.extend(Coord::ZERO)
                                * (self.player.radius + far * Coord::new(t)),
                    ) * Mat4::rotate_z(angle)
                        * Mat4::scale_uniform(Coord::new(0.2));
                    self.draw_gltf(&self.assets.club, matrix, framebuffer);
                }
            }
        }

        // UI
        let camera = geng::Camera2d {
            center: Vec2::ZERO,
            rotation: 0.0,
            fov: 10.0,
        };
        let framebuffer_size = framebuffer.size().map(|x| x as f32);
        let camera_aabb = AABB::point(camera.center).extend_symmetric(
            vec2(
                camera.fov * framebuffer_size.x / framebuffer_size.y,
                camera.fov,
            ) / 2.0,
        );

        if self.player.finished {
            self.geng.default_font().draw_with_outline(
                framebuffer,
                &camera,
                "YOU FINISHED!",
                camera_aabb.center() + vec2(0.0, 2.0),
                geng::TextAlign::CENTER,
                1.0,
                Rgba::BLACK,
                0.02,
                Rgba::WHITE,
            );
            self.geng.default_font().draw_with_outline(
                framebuffer,
                &camera,
                &format!("hits: {}\ndeaths: {}", self.player.hits, self.player.deaths),
                camera_aabb.center() + vec2(0.0, 0.0),
                geng::TextAlign::CENTER,
                1.0,
                Rgba::BLACK,
                0.02,
                Rgba::WHITE,
            );
        } else {
            self.geng.default_font().draw_with_outline(
                framebuffer,
                &camera,
                &format!("Hits: {}", self.player.hits),
                camera_aabb.top_left() + vec2(0.2, -0.5),
                geng::TextAlign::LEFT,
                0.5,
                Rgba::BLACK,
                0.01,
                Rgba::WHITE,
            );
            self.geng.default_font().draw_with_outline(
                framebuffer,
                &camera,
                &format!("Deaths: {}", self.player.deaths),
                camera_aabb.top_left() + vec2(0.2, -1.0),
                geng::TextAlign::LEFT,
                0.5,
                Rgba::BLACK,
                0.01,
                Rgba::WHITE,
            );
            self.geng.default_font().draw_with_outline(
                framebuffer,
                &camera,
                &format!("Fatigue delay: {:.1}s", self.player.fatigue),
                camera_aabb.top_left() + vec2(0.2, -1.5),
                geng::TextAlign::LEFT,
                0.5,
                Rgba::BLACK,
                0.01,
                Rgba::WHITE,
            );
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
            let mut draw_outline = |offset: f32| {
                ugli::draw(
                    framebuffer,
                    &self.assets.shaders.gltf_outline,
                    ugli::DrawMode::Triangles,
                    &mesh.data,
                    (
                        ugli::uniforms! {
                            u_model_matrix: matrix,
                            u_offset: offset,
                        },
                        geng::camera3d_uniforms(&self.camera, framebuffer.size().map(|x| x as f32)),
                    ),
                    ugli::DrawParameters {
                        depth_func: Some(ugli::DepthFunc::Less),
                        cull_face: Some(if offset > 0.0 {
                            ugli::CullFace::Front
                        } else {
                            ugli::CullFace::Back
                        }),
                        blend_mode: Some(ugli::BlendMode::default()),
                        ..default()
                    },
                );
            };
            draw_outline(0.03);
            draw_outline(-0.03);
        }
    }
}
