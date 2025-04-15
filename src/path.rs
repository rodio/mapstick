use vello::{kurbo::BezPath, peniko::Color};

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

    pub fn bez_path(&self) -> &BezPath {
        &self.bez_path
    }
    pub fn color(&self) -> &Color {
        &self.color
    }
    pub fn path_type(&self) -> &PathType {
        &self.path_type
    }
}

pub enum PathType {
    StrokeLine,
    Fill,
}
