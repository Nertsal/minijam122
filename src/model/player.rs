use super::*;

#[derive(HasId)]
pub struct Player {
    pub id: Id,
    pub position: Vec3<Coord>,
    pub velocity: Vec3<Coord>,
    pub radius: Coord,
    pub color: Rgba<f32>,
    pub last_shot: Vec3<Coord>,
    pub fatigue: R32,
    pub hits: usize,
    pub deaths: usize,
    pub finished: bool,
}

impl Player {
    pub fn new() -> Self {
        Self {
            id: Id(0),
            position: vec3(0.0, 0.0, 1.1).map(Coord::new),
            velocity: Vec3::ZERO,
            radius: Coord::new(0.1),
            color: Rgba::RED,
            last_shot: Vec3::ZERO,
            fatigue: R32::ZERO,
            hits: 0,
            deaths: 0,
            finished: false,
        }
    }
}

impl Default for Player {
    fn default() -> Self {
        Self::new()
    }
}
