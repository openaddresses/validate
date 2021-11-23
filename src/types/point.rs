///
/// A representation of a single point
///
#[derive(Debug)]
pub struct Point {
    /// An optional identifier for the address
    pub id: Option<i64>,

    /// JSON representation of properties
    pub props: serde_json::Map<String, serde_json::Value>,

    /// Simple representation of Lng/Lat geometry
    pub geom: Vec<geojson::PointType>
}

impl Point {
    pub fn new(feat: geojson::GeoJson) -> Result<Self, String> {
        let feat = match feat {
            geojson::GeoJson::Feature(feat) => feat,
            _ => { return Err(String::from("Not a GeoJSON Feature")); }
        };

        let props = match feat.properties {
            Some(props) => props,
            None => { return Err(String::from("Feature has no properties")); }
        };

        let geom = match feat.geometry {
            Some(geom) => match geom.value {
                geojson::Value::Point(pt) => vec![pt],
                geojson::Value::MultiPoint(pts) => pts,
                _ => { return Err(String::from("Point must have (Multi)Point geometry")); }
            },
            None => { return Err(String::from("Point must have geometry")); }
        };

        Ok(Point {
            id: match feat.id {
                Some(geojson::feature::Id::Number(id)) => id.as_i64(),
                _ => None
            },
            props: props,
            geom: geom
        })
    }
}
