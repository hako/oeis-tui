use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum OEISSearchResponse {
    Array(Vec<crate::api::Sequence>),
    Object(ResponseObject),
}

#[derive(Debug, Deserialize)]
pub struct ResponseObject {
    #[serde(default)]
    pub results: Vec<crate::api::Sequence>,
}

impl OEISSearchResponse {
    pub fn into_sequences(self) -> Vec<crate::api::Sequence> {
        match self {
            OEISSearchResponse::Array(list) => list,
            OEISSearchResponse::Object(obj) => obj.results,
        }
    }
}
