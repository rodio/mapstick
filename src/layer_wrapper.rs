use vello::peniko::Color;

use crate::{
    geometry::Geometry,
    tile::{Feature, GeomType, Layer},
};

pub struct LayerWrapper {
    layer: Layer,
    pub features: Vec<FeatureWrapper>,
}

impl LayerWrapper {
    pub fn new(layer: Layer) -> Self {
        let features = layer
            .features
            .iter()
            .map(|f| FeatureWrapper::new(f.clone()))
            .collect();
        Self { layer, features }
    }

    pub fn color(&self) -> Color {
        match self.layer.name.as_str() {
            "water" | "waterway" => Color::new([0.0, 0.702, 0.9294, 1.]),
            "landuse" => Color::new([0.1, 0.4, 0.3, 1.]),
            "landcover" => Color::new([0.1, 0.3, 0.3, 1.]),
            "transportation_name" => Color::new([0.9, 0.9, 0.9, 1.]),
            &_ => Color::new([0.6, 0.6, 0.6, 1.]),
        }
    }
}

pub struct FeatureWrapper {
    geometry: Geometry,
    feature: Feature,
}

impl FeatureWrapper {
    pub fn new(feature: Feature) -> Self {
        Self {
            geometry: Geometry::try_from(&feature.geometry).unwrap(),
            feature,
        }
    }

    pub fn ftype(&self) -> GeomType {
        self.feature.r#type()
    }

    pub fn geometry(&self) -> &Geometry {
        &self.geometry
    }
}
