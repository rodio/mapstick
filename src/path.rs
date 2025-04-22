use vello::{
    Scene,
    kurbo::{Affine, BezPath, Stroke},
    peniko::{self, Color},
};

use crate::layer_wrapper::LayerType;

pub struct Path {
    bez_path: BezPath,
    color: Color,
    path_type: PathType,
    layer_type: LayerType,
}

impl PartialEq for Path {
    fn eq(&self, other: &Self) -> bool {
        self.layer_type == other.layer_type
    }
}

impl Eq for Path {}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for Path {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.layer_type.cmp(&other.layer_type)
    }
}

impl Path {
    pub fn new(
        bez_path: BezPath,
        color: Color,
        path_type: PathType,
        layer_type: LayerType,
    ) -> Self {
        Self {
            bez_path,
            color,
            path_type,
            layer_type,
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

    pub fn layer_type(&self) -> &LayerType {
        &self.layer_type
    }
}

pub enum PathType {
    StrokeLine,
    Fill,
}
