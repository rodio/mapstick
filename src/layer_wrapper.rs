use core::panic;

use vello::peniko::Color;

use crate::{
    geometry::Geometry,
    tile::{Feature, GeomType, Layer},
};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub enum LayerType {
    Building,
    Boundary,
    Waterway,
    WaterName,
    Water,
    Transportation,
    TransportationName,
    Place,
    Housenumber,
    Poi,
    Landcover,
    Landuse,
}

pub struct LayerWrapper {
    layer_type: LayerType,

    pub features: Vec<FeatureWrapper>,
}

impl LayerWrapper {
    pub fn new(layer: Layer) -> Self {
        let features = layer
            .features
            .iter()
            .map(|f| FeatureWrapper::new(f.clone()))
            .collect();

        let layer_type = match layer.name.as_str() {
            "waterway" => LayerType::Waterway,
            "water" => LayerType::Water,
            "water_name" => LayerType::WaterName,
            "building" => LayerType::Building,
            "landuse" => LayerType::Landuse,
            "boundary" => LayerType::Boundary,
            "transportation" => LayerType::Transportation,
            "transportation_name" => LayerType::TransportationName,
            "place" => LayerType::Place,
            "housenumber" => LayerType::Housenumber,
            "poi" => LayerType::Poi,
            "landcover" => LayerType::Landcover,
            &_ => panic!("{}", layer.name),
        };
        Self {
            layer_type,
            features,
        }
    }

    pub fn color(&self) -> Color {
        let alpha = 0.8;
        match self.layer_type {
            LayerType::Waterway => Color::new([0.0, 0.902, 0.9294, alpha]),
            LayerType::WaterName => Color::new([0.0, 0.302, 0.3294, alpha]),
            LayerType::Water => Color::new([0.0, 0.7, 0.9, alpha]),
            LayerType::Landcover => Color::new([0.6, 0.9, 0.4, alpha]),
            LayerType::Landuse => Color::new([0.95, 0.95, 0.95, alpha]),
            LayerType::Building => Color::new([0.5, 0.5, 0.5, alpha]),
            LayerType::Boundary => Color::new([0.1, 0.9, 0.1, alpha]),
            LayerType::Place => Color::new([0.9, 0.1, 0.1, alpha]),
            _ => Color::new([0.6, 0.6, 0.6, alpha]),
        }
    }

    pub fn layer_type(&self) -> LayerType {
        self.layer_type.clone()
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
