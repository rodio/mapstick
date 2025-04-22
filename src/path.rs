use vello::{
    Scene,
    kurbo::{Affine, BezPath, Stroke},
    peniko::{self, Color},
};

pub struct Path {
    bez_path: BezPath,
    color: Color,
    path_type: PathType,
}

impl Path {
    pub fn new(bez_path: BezPath, color: Color, path_type: PathType) -> Self {
        Self {
            bez_path,
            color,
            path_type,
        }
    }

    pub fn draw(&self, scene: &mut Scene) {
        match self.path_type {
            PathType::StrokeLine => scene.stroke(
                &Stroke::new(6.0),
                Affine::IDENTITY,
                self.color,
                None,
                &self.bez_path,
            ),
            PathType::Fill => scene.fill(
                peniko::Fill::NonZero,
                Affine::IDENTITY,
                self.color,
                None,
                &self.bez_path,
            ),
        }
    }
}

pub enum PathType {
    StrokeLine,
    Fill,
}
