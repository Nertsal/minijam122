use super::*;

#[derive(geng::Assets)]
pub struct Assets {
    #[asset(load_with = "gltf_load::load_meshes(&geng, base_path.join(\"level.glb\"))")]
    pub level: Vec<Mesh>,
    #[asset(load_with = "gltf_load::load_meshes(&geng, base_path.join(\"player.glb\"))")]
    pub player: Vec<Mesh>,
    #[asset(load_with = "gltf_load::load_meshes(&geng, base_path.join(\"arrow.glb\"))")]
    pub arrow: Vec<Mesh>,
    #[asset(load_with = "gltf_load::load_meshes(&geng, base_path.join(\"club.glb\"))")]
    pub club: Vec<Mesh>,
    pub shaders: Shaders,
    pub sfx: SfxAssets,
}

#[derive(geng::Assets)]
pub struct SfxAssets {
    pub hit: geng::Sound,
    pub fall: geng::Sound,
    #[asset(range = "0..4", path = "bump*.wav")]
    pub bump: Vec<geng::Sound>,
}

#[derive(geng::Assets)]
pub struct Shaders {
    pub gltf: ugli::Program,
    pub gltf_outline: ugli::Program,
    pub postprocess: ugli::Program,
}
