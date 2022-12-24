use super::*;

impl Game {
    fn draw_impl(&self, framebuffer: &mut ugli::Framebuffer, program: &ugli::Program) {
        // Level
        draw_gltf(
            &self.assets.level,
            program,
            Mat4::identity(),
            framebuffer,
            &self.camera,
        );

        // Players
        let matrix =
            Mat4::translate(self.player.position) * Mat4::scale_uniform(self.player.radius);
        draw_gltf(
            &self.assets.player,
            program,
            matrix,
            framebuffer,
            &self.camera,
        );

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
                    draw_gltf(
                        &self.assets.arrow,
                        program,
                        matrix,
                        framebuffer,
                        &self.camera,
                    );
                    let matrix = Mat4::translate(
                        self.player.position
                            - direction.extend(Coord::ZERO)
                                * (self.player.radius + Coord::new(0.2)),
                    ) * Mat4::rotate_z(angle)
                        * Mat4::scale_uniform(Coord::new(0.2));
                    draw_gltf(
                        &self.assets.club,
                        program,
                        matrix,
                        framebuffer,
                        &self.camera,
                    );
                }
                Control::Power { direction, time } => {
                    let angle = direction.arg();
                    let power = Coord::new((1.0 - time.as_f32().cos()) * 2.5);
                    let matrix = Mat4::translate(self.player.position)
                        * Mat4::rotate_z(angle)
                        * Mat4::scale(vec3(0.2 + power.as_f32() * 0.1, 0.2, 0.2).map(Coord::new));
                    draw_gltf(
                        &self.assets.arrow,
                        program,
                        matrix,
                        framebuffer,
                        &self.camera,
                    );
                    let matrix = Mat4::translate(
                        self.player.position
                            - direction.extend(Coord::ZERO)
                                * (self.player.radius + Coord::new(0.2 + power.as_f32() * 0.2)),
                    ) * Mat4::rotate_z(angle)
                        * Mat4::scale_uniform(Coord::new(0.2));
                    draw_gltf(
                        &self.assets.club,
                        program,
                        matrix,
                        framebuffer,
                        &self.camera,
                    );
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
                    draw_gltf(
                        &self.assets.arrow,
                        program,
                        matrix,
                        framebuffer,
                        &self.camera,
                    );
                    let matrix = Mat4::translate(
                        self.player.position
                            - direction.extend(Coord::ZERO)
                                * (self.player.radius + Coord::new(0.2 + power.as_f32() * 0.2)),
                    ) * Mat4::rotate_z(angle)
                        * Mat4::scale_uniform(Coord::new(0.2));
                    draw_gltf(
                        &self.assets.club,
                        program,
                        matrix,
                        framebuffer,
                        &self.camera,
                    );
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
                    draw_gltf(
                        &self.assets.club,
                        program,
                        matrix,
                        framebuffer,
                        &self.camera,
                    );
                }
            }
        }
    }
    pub fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        self.framebuffer_size = framebuffer.size();

        if self.color_texture.borrow().size() != framebuffer.size() {
            self.color_texture = RefCell::new(ugli::Texture::new_uninitialized(
                self.geng.ugli(),
                framebuffer.size(),
            ));
            self.outline_texture = RefCell::new(ugli::Texture::new_uninitialized(
                self.geng.ugli(),
                framebuffer.size(),
            ));
            self.depth_buffer = RefCell::new(ugli::Renderbuffer::new(
                self.geng.ugli(),
                framebuffer.size(),
            ));
        }

        {
            let mut color_texture = self.color_texture.borrow_mut();
            let mut depth_buffer = self.depth_buffer.borrow_mut();
            let mut framebuffer = ugli::Framebuffer::new(
                self.geng.ugli(),
                ugli::ColorAttachment::Texture(&mut color_texture),
                ugli::DepthAttachment::Renderbuffer(&mut depth_buffer),
            );
            let framebuffer = &mut framebuffer;
            ugli::clear(
                framebuffer,
                Some(Rgba::new(0.8, 0.8, 1.0, 1.0)),
                Some(1.0),
                None,
            );
            self.draw_impl(framebuffer, &self.assets.shaders.gltf);
        }

        {
            let mut outline_texture = self.outline_texture.borrow_mut();
            let mut depth_buffer = self.depth_buffer.borrow_mut();
            let mut framebuffer = ugli::Framebuffer::new(
                self.geng.ugli(),
                ugli::ColorAttachment::Texture(&mut outline_texture),
                ugli::DepthAttachment::Renderbuffer(&mut depth_buffer),
            );
            let framebuffer = &mut framebuffer;
            ugli::clear(
                framebuffer,
                Some(Rgba::new(0.8, 0.8, 1.0, 1.0)),
                Some(1.0),
                None,
            );
            self.draw_impl(framebuffer, &self.assets.shaders.gltf_outline);
        }
        ugli::draw(
            framebuffer,
            &self.assets.shaders.postprocess,
            ugli::DrawMode::TriangleFan,
            &self.quad_geometry,
            ugli::uniforms! {
                u_outline_texture_size: self.outline_texture.borrow().size().map(|x| x as f32),
                u_outline_texture: &self.outline_texture.borrow(),
                u_color_texture: &self.color_texture.borrow(),
            },
            ugli::DrawParameters { ..default() },
        );

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
}
pub fn draw_gltf<'a>(
    gltf: impl IntoIterator<Item = &'a Mesh>,
    program: &ugli::Program,
    matrix: Mat4<Coord>,
    framebuffer: &'a mut ugli::Framebuffer,
    camera: &Camera,
) {
    for mesh in gltf.into_iter() {
        let matrix = matrix.map(|x| x.as_f32()); // * mesh.transform;
        ugli::draw(
            framebuffer,
            program,
            ugli::DrawMode::Triangles,
            &mesh.data,
            (
                mesh.material.uniforms(),
                ugli::uniforms! {
                    u_model_matrix: matrix,
                    u_eye_pos: camera.eye_pos(),
                    u_light_dir: vec3(1.0, -2.0, 5.0),
                    u_light_color: Rgba::WHITE,
                    u_ambient_light_color: Rgba::WHITE,
                    u_ambient_light_intensity: 0.8,
                },
                geng::camera3d_uniforms(camera, framebuffer.size().map(|x| x as f32)),
            ),
            ugli::DrawParameters {
                depth_func: Some(ugli::DepthFunc::Less),
                ..default()
            },
        );
    }
}
