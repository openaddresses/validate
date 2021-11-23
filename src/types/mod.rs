pub mod context;
pub mod name;
pub mod polygon;
pub mod point;

pub trait AsFeat {
    fn as_feat(self) -> geojson::Feature;
}
