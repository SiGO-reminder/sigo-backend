// TMAP walking API 응답 본문 JSON 예시.
// 원하는 데이터인 "totalTime"은 "pointType": "SP" (즉, 출발지)인 경우에만 포함된다.
// 따라서, 이 기준으로 검색해서 필요해 보이는 정보 몇 가지만 저장하고 나머지는 drop한다.
// {
//   "features": [
//       {
//         "geometry": {
//           "coordinates": [
//               127.02974088852942,
//               37.582870370000336
//           ],
//           "type": "Point"
//         },
//         "properties": {
//           "description": "보행자도로 을 따라 7m 이동",
//           "direction": "",
//           "facilityName": "",
//           "facilityType": "15",
//           "index": 0,
//           "intersectionName": "",
//           "name": "",
//           "nearPoiName": "",
//           "nearPoiX": "0.0",
//           "nearPoiY": "0.0",
//           "pointIndex": 0,
//           "pointType": "SP",
//           "totalDistance": 2556,
//           "totalTime": 2029,
//           "turnType": 200
//         },
//         "type": "Feature"
//     },
//     ...
//   ]
// }

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub struct TmapWalkingResponse {
    pub features: Vec<Feature>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Feature {
    pub properties: Properties,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Properties {
    #[serde(rename = "totalTime")]
    pub total_time: Option<u32>, // totalTime만 포함
}

impl TmapWalkingResponse {
    pub fn from_json(json_response: Value) -> Result<Self, serde_json::Error> {
        let tmap_walking_response: TmapWalkingResponse = serde_json::from_value(json_response)?;
        Ok(tmap_walking_response)
    }

    pub fn get_total_time(&self) -> Option<i64> {
        for feature in &self.features {
            if feature.properties.total_time.is_some() {
                return feature.properties.total_time.map(|t| t as i64);
            }
        }
        None
    }
}
