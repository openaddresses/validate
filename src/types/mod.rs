pub mod context;
pub mod name;
pub mod polygon;

pub trait AsFeat {
    fn as_feat(self) -> geojson::Feature;
}
