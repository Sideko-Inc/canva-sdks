use crate::schemas::*;
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct ListAllDesignsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continuation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ownership: Option<GetDesignsOwnershipEnum>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<GetDesignsSortByEnum>,
}
