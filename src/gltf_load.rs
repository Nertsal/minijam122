use super::*;

#[derive(ugli::Vertex)]
pub struct Vertex {
    pub a_uv: Vec2<f32>,
    pub a_mr_uv: Vec2<f32>,
    pub a_pos: Vec3<f32>,
    pub a_normal: Vec3<f32>,
    pub a_color: Rgba<f32>,
}

pub struct Material {
    base_color_texture: ugli::Texture,
    base_color_factor: Rgba<f32>,
    metallic_roughness_texture: ugli::Texture,
    metallic_factor: f32,
    roughness_factor: f32,
    // TODO: normal texture
    // TODO: occlusion texture
    // TODO: emissive texture
}

impl Material {
    pub fn uniforms(&self) -> impl ugli::Uniforms + '_ {
        ugli::uniforms! {
            u_base_color_texture: &self.base_color_texture,
            u_base_color_factor: self.base_color_factor,
            u_metallic_roughness_texture: &self.metallic_roughness_texture,
            u_metallic_factor: self.metallic_factor,
            u_roughness_factor: self.roughness_factor,
        }
    }
}

pub struct Mesh {
    pub data: ugli::VertexBuffer<Vertex>,
    pub material: Material,
}

pub async fn load_meshes(
    geng: &Geng,
    path: impl AsRef<std::path::Path>,
) -> anyhow::Result<Vec<Mesh>> {
    let file = load_file(path.as_ref()).await?;
    let (document, buffers, images) = gltf::import_slice(&file).unwrap();
    let mut meshes = Vec::new();
    for mesh in document.meshes() {
        info!("{:?}", mesh.name());
        for primitive in mesh.primitives() {
            let reader = primitive.reader(|buffer| buffers.get(buffer.index()).map(|x| &**x));
            let positions: Vec<Vec3<f32>> = reader
                .read_positions()
                .expect("No positions for primitive mesh WAT")
                .map(|[x, y, z]| vec3(x, y, z))
                .collect();
            let normals: Vec<Vec3<f32>> = reader
                .read_normals()
                .expect("Missing normals, this is not supported yet")
                .map(|[x, y, z]| vec3(x, y, z))
                .collect();
            let colors: Option<Vec<Rgba<f32>>> = reader.read_colors(0).map(|colors| {
                colors
                    .into_rgba_f32()
                    .map(|[r, g, b, a]| Rgba::new(r, g, b, a))
                    .collect()
            });
            let indices = reader
                .read_indices()
                .expect("Absent indices not supported yet")
                .into_u32()
                .map(|x| x as usize);
            assert_eq!(primitive.mode(), gltf::mesh::Mode::Triangles);
            let data = ugli::VertexBuffer::new_static(
                geng.ugli(),
                indices
                    .map(|index| Vertex {
                        a_mr_uv: Vec2::ZERO, // TODO
                        a_uv: Vec2::ZERO,    // TODO
                        a_pos: positions[index],
                        a_normal: normals[index], // TODO: optional
                        a_color: colors.as_ref().map_or(Rgba::WHITE, |colors| colors[index]),
                    })
                    .collect(),
            );
            let material = {
                let material = primitive.material();
                let white_texture =
                    || ugli::Texture::new_with(geng.ugli(), vec2(1, 1), |_| Rgba::WHITE);
                Material {
                    base_color_texture: white_texture(), // TODO material.pbr_metallic_roughness().base_color_texture()
                    base_color_factor: {
                        let [r, g, b, a] = material.pbr_metallic_roughness().base_color_factor();
                        Rgba::new(r, g, b, a)
                    },
                    metallic_roughness_texture: white_texture(), // TODO
                    metallic_factor: material.pbr_metallic_roughness().metallic_factor(),
                    roughness_factor: material.pbr_metallic_roughness().roughness_factor(),
                }
            };
            meshes.push(Mesh { data, material });
        }
    }
    Ok(meshes)
}
