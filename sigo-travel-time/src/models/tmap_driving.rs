use serde::{Deserialize, Serialize};
use serde_json::Value;

// TMAP Driving API - 응답 본문 구조
// {
//   "type": "FeatureCollection",
//   "features": [
//     {
//       "type": "Feature",
//       "properties": {
//         "totalDistance": 428496,
//         "totalTime": 23477,
//         "totalFare": 22300,
//         "taxiFare": 443700
//       }
//     }
//   ]
// }

#[derive(Serialize, Deserialize, Debug)]
pub struct TmapDrivingResponse {
    pub response_type: String,
    pub features: Vec<Feature>,
}

impl TmapDrivingResponse {
    pub fn from_json(json_response: Value) -> Result<Self, serde_json::Error> {
        let tmap_driving_response: TmapDrivingResponse = serde_json::from_value(json_response)?;
        Ok(tmap_driving_response)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Feature {
    #[serde(rename = "type")]
    pub feature_type: String,
    pub properties: Properties,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Properties {
    #[serde(rename = "totalDistance")]
    pub total_distance: u32,
    #[serde(rename = "totalTime")]
    pub total_time: u32,
    #[serde(rename = "totalFare")]
    pub total_fare: u32,
    #[serde(rename = "taxiFare")]
    pub taxi_fare: u32,
}
