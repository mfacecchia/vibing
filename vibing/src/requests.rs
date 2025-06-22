use crate::{
    creds::get_cred_use_defaults,
    env,
    error::{AppError, Result, build_generic_error},
    utils::verbose_print,
};
use reqwest::{
    Method, RequestBuilder,
    header::{HeaderMap, HeaderValue},
};
use serde::{de::DeserializeOwned, Serialize};

pub struct FetchOptions<T>
where
    T: Serialize,
{
    pub host: String,
    pub path: String,
    pub method: Method,
    pub headers: Option<HeaderMap<HeaderValue>>,
    pub authorization: Option<String>,
    pub data: Option<T>,
}

impl<T> FetchOptions<T>
where
    T: Serialize,
{
    pub fn new(
        host: String,
        path: String,
        method: Method,
        headers: Option<HeaderMap<HeaderValue>>,
        authorization: Option<String>,
        data: Option<T>,
    ) -> FetchOptions<T> {
        FetchOptions {
            host,
            path,
            method,
            headers,
            authorization,
            data,
        }
    }
}

pub async fn fetch_backend<T, U>(
    path: String,
    method: Method,
    data: Option<T>,
    verbose: bool,
) -> Result<U>
where
    T: Serialize,
    U: DeserializeOwned,
{
    let mut fetch_options = FetchOptions::new(
        env::get_env("BACKEND_URL"),
        path,
        method,
        None,
        Some(get_cred_use_defaults()?),
        data,
    );
    let res = fetch::<T, U>(&mut fetch_options, verbose).await?;
    Ok(res)
}

pub async fn fetch<T, U>(fetch_options: &mut FetchOptions<T>, verbose: bool) -> Result<U>
where
    T: Serialize,
    U: DeserializeOwned,
{
    verbose_print(verbose, "Building request...");
    let full_url = format!("{}{}", fetch_options.host.trim(), fetch_options.path.trim());
    let client = reqwest::Client::new();
    let mut req = client.request(fetch_options.method.clone(), full_url);
    if let Some(auth_token) = fetch_options.authorization.clone() {
        req = req.bearer_auth(auth_token);
    }
    if let Some(req_body) = &fetch_options.data {
        req = set_req_body(&req_body, &mut fetch_options.headers, req)?;
    }
    if let Some(req_headers) = fetch_options.headers.clone() {
        req = req.headers(req_headers);
    }
    let req = req.build()?;
    verbose_print(verbose, "Fetching data...");
    let res = client.execute(req).await?;
    println!("{}", res.status());
    verbose_print(verbose, "Done.\nDeserializing response...");
    if res.status().is_success() {
        let json_res = res.json::<U>().await?;
        verbose_print(verbose, "Done.");
        return Ok(json_res);
    }
    // TODO: Handle validation errors as well
    if res.status().is_client_error() && res.status().as_str().eq("401") {
        return Err(AppError::AuthorizationError(None));
    }
    Err(AppError::NetworkError(None))
}

/// Sets the request body based on the passed `Content-type` header.
///
/// Defaults to `application/json`
fn set_req_body<T>(
    data: &T,
    headers: &mut Option<HeaderMap<HeaderValue>>,
    mut req: RequestBuilder,
) -> Result<RequestBuilder>
where
    T: Serialize,
{
    if headers.is_none() {
        let mut header_map = HeaderMap::new();
        header_map.insert(
            "Content-type",
            HeaderValue::from_str("application/json")
                .map_err(|_| return AppError::Other(Box::new(build_generic_error(None))))?,
        );
        let _ = headers.insert(header_map);
    }
    let some_headers = headers.as_ref().unwrap();
    match some_headers.get("Content-type") {
        Some(req_content_type) => {
            let req_content_type_str = req_content_type
                .to_str()
                .map_err(|_| return AppError::Other(Box::new(build_generic_error(None))))?;
            if req_content_type_str.contains("json") {
                req = req.json(data);
                return Ok(req);
            }
            if req_content_type_str.contains("form-data") {
                req = req.form(data);
                return Ok(req);
            }
        }
        None => {
            req = req.json(data);
        }
    }
    Ok(req)
}
