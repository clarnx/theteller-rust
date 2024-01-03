use reqwest::header::HeaderMap;
use reqwest::Client as ReqwestClient;

pub trait CustomClientFunctions {
    fn set_headers(&self, request_headers: HeaderMap) -> ReqwestClient;
}
