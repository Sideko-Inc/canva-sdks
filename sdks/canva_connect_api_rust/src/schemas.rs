pub struct BinaryResponse {
    pub content: bytes::Bytes,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct TeamUser {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct Thumbnail {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<String>,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct Design {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub doctype_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<TeamUser>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<Thumbnail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct GetDesignsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continuation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<Design>>,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum GetDesignsOwnershipEnum {
    #[default]
    #[serde(rename = "OWNED")]
    Owned,
    #[serde(rename = "SHARED")]
    Shared,
    #[serde(rename = "ANY")]
    Any,
}
impl std::fmt::Display for GetDesignsOwnershipEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            GetDesignsOwnershipEnum::Owned => "OWNED",
            GetDesignsOwnershipEnum::Shared => "SHARED",
            GetDesignsOwnershipEnum::Any => "ANY",
        };
        write!(f, "{}", str_val)
    }
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum GetDesignsSortByEnum {
    #[default]
    #[serde(rename = "RELEVANCE")]
    Relevance,
    #[serde(rename = "MODIFIED_DESCENDING")]
    ModifiedDescending,
    #[serde(rename = "MODIFIED_ASCENDING")]
    ModifiedAscending,
    #[serde(rename = "TITLE_DESCENDING")]
    TitleDescending,
    #[serde(rename = "TITLE_ASCENDING")]
    TitleAscending,
}
impl std::fmt::Display for GetDesignsSortByEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            GetDesignsSortByEnum::Relevance => "RELEVANCE",
            GetDesignsSortByEnum::ModifiedDescending => "MODIFIED_DESCENDING",
            GetDesignsSortByEnum::ModifiedAscending => "MODIFIED_ASCENDING",
            GetDesignsSortByEnum::TitleDescending => "TITLE_DESCENDING",
            GetDesignsSortByEnum::TitleAscending => "TITLE_ASCENDING",
        };
        write!(f, "{}", str_val)
    }
}
