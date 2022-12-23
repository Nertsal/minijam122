use super::*;

#[derive(geng::Assets)]
pub struct Assets {
    #[asset(load_with = "gltf_load::load_meshes(&geng, base_path.join(\"level.glb\"))")]
    pub level: Vec<Mesh>,
    pub shaders: Shaders,
}

#[derive(geng::Assets)]
pub struct Shaders {
    pub gltf: ugli::Program,
}
