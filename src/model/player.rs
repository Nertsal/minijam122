use super::*;

#[derive(HasId)]
pub struct Player {
    pub id: Id,
    pub position: Vec3<Coord>,
    pub velocity: Vec3<Coord>,
    pub radius: Coord,
    pub color: Rgba<f32>,
    pub last_shot: Vec3<Coord>,
}
