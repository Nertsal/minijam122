use super::*;

#[derive(Debug, Clone)]
pub struct Camera {
    pub fov: f32,
    pub pos: Vec3<f32>,
    pub distance: f32,
    pub rot_h: f32,
    pub rot_v: f32,
}

impl Camera {
    pub fn eye_pos(&self) -> Vec3<f32> {
        let v = vec2(self.distance, 0.0).rotate(self.rot_v);
        self.pos + vec3(0.0, -v.y, v.x)
    }
}

impl geng::AbstractCamera3d for Camera {
    fn view_matrix(&self) -> Mat4<f32> {
        Mat4::translate(vec3(0.0, 0.0, -self.distance))
            * Mat4::rotate_x(-self.rot_v)
            * Mat4::rotate_z(-self.rot_h)
            * Mat4::translate(-self.pos)
    }

    fn projection_matrix(&self, framebuffer_size: Vec2<f32>) -> Mat4<f32> {
        Mat4::perspective(
            self.fov,
            framebuffer_size.x / framebuffer_size.y,
            1.0,
            100.0,
        )
    }
}
