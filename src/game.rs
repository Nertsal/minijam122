use super::*;

pub struct Game {
    geng: Geng,
    assets: Rc<Assets>,
    camera: Camera,
}

impl Game {
    pub fn new(geng: &Geng, assets: &Rc<Assets>) -> Self {
        Self {
            geng: geng.clone(),
            assets: assets.clone(),
            camera: Camera {
                fov: f32::PI / 3.0,
                pos: vec3(0.0, 0.0, 1.0),
                distance: 5.0,
                rot_h: 0.0,
                rot_v: f32::PI / 3.0,
            },
        }
    }
}

impl geng::State for Game {
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        ugli::clear(framebuffer, Some(Rgba::BLACK), Some(1.0), None);
        let framebuffer_size = framebuffer.size().map(|x| x as f32);
        for mesh in &self.assets.level {
            ugli::draw(
                framebuffer,
                &self.assets.shaders.gltf,
                ugli::DrawMode::Triangles,
                &mesh.data,
                (
                    mesh.material.uniforms(),
                    ugli::uniforms! {
                        u_model_matrix: Mat4::identity(), // TODO
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
    }
}
