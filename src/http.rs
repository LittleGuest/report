use std::{collections::HashMap, str::FromStr, time::Duration};

use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue},
    Client, RequestBuilder,
};
use serde_json::Value;

use crate::ReportResult;

/// 设置headers、form或body参数、设置超时时间
pub struct HttpReqConf {
    url: String,
    headers: Option<HashMap<String, String>>,
    form: Option<HashMap<String, Value>>,
    body: Option<HashMap<String, Value>>,
}

pub fn get(conf: &HttpReqConf) -> ReportResult<String> {
    // let mut client = reqwest::Client::new()
    //     .get(&conf.url)
    //     .timeout(Duration::from_secs(10));
    //
    // if let Some(form) = &conf.form {
    //     client.form(form);
    // }
    //
    // if let Some(body) = &conf.body {
    //     client.json(&body);
    // }
    //
    // client.build();

    let mut client = hyper::Request::builder();
    client.uri(&conf.url);

    todo!()
}

// fn set_header(conf: &HttpReqConf, req: &mut RequestBuilder) -> ReportResult<()> {
//     if let Some(headers) = &conf.headers {
//         let mut header_map = HeaderMap::new();
//         for (k, v) in headers {
//             header_map.insert(HeaderName::from_str(k)?, HeaderValue::from_str(v)?);
//         }
//         req.headers(header_map);
//     }
//     Ok(())
// }
