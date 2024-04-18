/// Generatedby Sideko (sideko.dev)
use crate::auth;
use crate::request_types::*;
use crate::result;
use crate::error_enums;
use crate::schemas::*;
use reqwest::blocking::Client as ReqwestClient;
use reqwest::blocking::RequestBuilder as ReqwestRequestBuilder;
#[allow(unused)]
use reqwest::blocking::multipart as reqwest_multipart;
use std::collections::BTreeMap;
#[derive(Clone, Debug)]
pub struct Client {
    pub base_url: String,
    auth: BTreeMap<String, auth::AuthProvider>,
}
impl Default for Client {
    fn default() -> Self {
        Self {
            base_url: "https://api.canva.com/rest/v1".to_string(),
            auth: BTreeMap::new(),
        }
    }
}
impl Client {
    /// Override the default base url
    pub fn with_base_url(mut self, base_url: &str) -> Self {
        self.base_url = base_url.into();
        self
    }
    /// Authentication  builder function to store bearer credentials in the client
    pub fn with_bearer_auth(mut self, val: &str) -> Self {
        self.auth
            .insert(
                "bearerAuth".to_string(),
                auth::AuthProvider::Bearer(val.to_string()),
            );
        self
    }
    fn builder_with_auth(
        &self,
        mut req_builder: ReqwestRequestBuilder,
        auth_names: &[&str],
    ) -> ReqwestRequestBuilder {
        for auth_name in auth_names {
            if let Some(provider) = self.auth.get(&auth_name.to_string()) {
                req_builder = provider.add_auth_blocking(req_builder);
            }
        }
        req_builder
    }
    pub fn list_all_designs(
        &self,
        request: ListAllDesignsRequest,
    ) -> result::Result<GetDesignsResponse, error_enums::ListAllDesignsErrors> {
        let endpoint = "/designs";
        let url = format!("{}{}", self.base_url, endpoint);
        let mut query_params: Vec<(&str, String)> = vec![];
        if let Some(continuation) = request.continuation {
            query_params.push(("continuation", format!("{}", & continuation)));
        }
        if let Some(ownership) = request.ownership {
            query_params.push(("ownership", format!("{}", & ownership)));
        }
        if let Some(query) = request.query {
            query_params.push(("query", format!("{}", & query)));
        }
        if let Some(sort_by) = request.sort_by {
            query_params.push(("sort_by", format!("{}", & sort_by)));
        }
        let unauthed_builder = ReqwestClient::default().get(&url).query(&query_params);
        let authed_builder = self.builder_with_auth(unauthed_builder, &["bearerAuth"]);
        let response = authed_builder.send().map_err(result::Error::Dispatch)?;
        let status_code = response.status().as_u16();
        match status_code {
            200 => {
                let response_text = response.text().unwrap_or_default();
                let data = serde_json::from_str::<GetDesignsResponse>(&response_text)
                    .map_err(|serde_err| result::Error::UnexpectedResponseBody {
                        status_code,
                        method: "GET".to_string(),
                        url: url.to_string(),
                        response_text,
                        expected_signature: "GetDesignsResponse".to_string(),
                        serde_err,
                    })?;
                Ok(data)
            }
            _ => {
                let expected_status_codes: Vec<String> = vec!["200".to_string(),];
                Err(result::Error::BlockingUnexpectedStatus {
                    status_code,
                    method: "".to_string(),
                    url: url.to_string(),
                    response,
                    expected_status_codes,
                })
            }
        }
    }
}
