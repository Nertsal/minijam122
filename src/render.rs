use super::*;

pub struct Render {
    geng: Geng,
    assets: Rc<Assets>,
    pub camera: Camera,
    framebuffer_size: Vec2<usize>,
    pub quad_geometry: ugli::VertexBuffer<draw_2d::Vertex>,
    outline_texture: RefCell<ugli::Texture>,
    color_texture: RefCell<ugli::Texture>,
    depth_buffer: RefCell<ugli::Renderbuffer<ugli::DepthComponent>>,
}

impl Render {
    pub fn new(geng: &Geng, assets: &Rc<Assets>) -> Self {
        Self {
            geng: geng.clone(),
            assets: assets.clone(),
            camera: Camera {
                fov: f32::PI / 5.0,
                pos: vec3(17.0, -10.0, 1.0),
                distance: 60.0,
                rot_h: 0.0,
                rot_v: f32::PI / 6.0,
            },
            framebuffer_size: vec2(1, 1),
            color_texture: RefCell::new(ugli::Texture::new_uninitialized(geng.ugli(), vec2(1, 1))),
            outline_texture: RefCell::new(ugli::Texture::new_uninitialized(
                geng.ugli(),
                vec2(1, 1),
            )),
            depth_buffer: RefCell::new(ugli::Renderbuffer::new(geng.ugli(), vec2(1, 1))),
            quad_geometry: ugli::VertexBuffer::new_static(
                geng.ugli(),
                vec![
                    draw_2d::Vertex {
                        a_pos: vec2(0.0, 0.0),
                    },
                    draw_2d::Vertex {
                        a_pos: vec2(1.0, 0.0),
                    },
                    draw_2d::Vertex {
                        a_pos: vec2(1.0, 1.0),
                    },
                    draw_2d::Vertex {
                        a_pos: vec2(0.0, 1.0),
                    },
                ],
            ),
        }
    }

    pub fn draw_world(
        &mut self,
        player: &Player,
        control: &Control,
        framebuffer: &mut ugli::Framebuffer,
    ) {
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
            self.draw_world_with(player, control, &self.assets.shaders.gltf, framebuffer);
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
            self.draw_world_with(
                player,
                control,
                &self.assets.shaders.gltf_outline,
                framebuffer,
            );
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
    }

    fn draw_world_with(
        &self,
        player: &Player,
        control: &Control,
        program: &ugli::Program,
        framebuffer: &mut ugli::Framebuffer,
    ) {
        // Level
        draw_gltf(
            &self.assets.level,
            program,
            Mat4::identity(),
            framebuffer,
            &self.camera,
        );

        // Players
        let matrix = Mat4::translate(player.position) * Mat4::scale_uniform(player.radius);
        draw_gltf(
            &self.assets.player,
            program,
            matrix,
            framebuffer,
            &self.camera,
        );

        // Control
        if !player.finished {
            match &control {
                Control::Disabled => {}
                Control::Direction => {
                    let direction =
                        self.screen_pos_to_move_dir(self.geng.window().mouse_pos(), player);
                    let angle = direction.arg();
                    let matrix = Mat4::translate(player.position)
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
                        player.position
                            - direction.extend(Coord::ZERO) * (player.radius + Coord::new(0.2)),
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
                    let matrix = Mat4::translate(player.position)
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
                        player.position
                            - direction.extend(Coord::ZERO)
                                * (player.radius + Coord::new(0.2 + power.as_f32() * 0.2)),
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
                    let matrix = Mat4::translate(player.position)
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
                        player.position
                            - direction.extend(Coord::ZERO)
                                * (player.radius + Coord::new(0.2 + power.as_f32() * 0.2)),
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
                        player.position
                            - direction.extend(Coord::ZERO) * (player.radius + far * Coord::new(t)),
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

    pub fn screen_pos_to_move_dir(&self, position: Vec2<f64>, player: &Player) -> Vec2<Coord> {
        let ray = self.camera.pixel_ray(
            self.framebuffer_size.map(|x| x as f32),
            position.map(|x| x as f32),
        );
        let t =
            util::intersect_ray_with_plane(vec3(0.0, 0.0, 1.0), -player.position.z.as_f32(), ray);
        let raycast = (ray.from + ray.dir * t).map(Coord::new);
        -(raycast.xy() - player.position.xy()).normalize_or_zero()
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
        let matrix = matrix.map(|x| x.as_f32());
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
