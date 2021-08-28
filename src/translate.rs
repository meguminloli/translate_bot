use std::io::{Error, ErrorKind};

use hyper_tls::HttpsConnector;
use hyper::{Body, Client, Method, Request, body::HttpBody};
use percent_encoding::{NON_ALPHANUMERIC, utf8_percent_encode};
use serde::Deserialize;
use log::info;

#[derive(Deserialize, Debug)]
struct Response {
    translated_text: String,
    from_lang: String,
}

pub async fn check_language(message: &str, api_key: &str) -> Result<String, Box<dyn std::error::Error>> {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    let escaped = utf8_percent_encode(message, NON_ALPHANUMERIC);
    let url = format!("https://translo.p.rapidapi.com/translate?text={}&to=en", escaped);
    let body = r#"{\r
        \"key1\": \"value\",\r
        \"key2\": \"value\"\r
    }"#;
    
    let req = Request::builder()
        .method(Method::POST)
        .uri(url)
        .header("x-rapidapi-host", "translo.p.rapidapi.com")
        .header("Content-Type", "application/json")
        .header("x-rapidapi-key", api_key)
        .body(Body::from(body))?;
    let mut req = client.request(req).await?;
    let buf = match req.data().await {
        Some(data) => {
            let body = data?;
            String::from_utf8(body[..].to_owned())?
        }
        None => {
            info!("The fuck!");
            return Err(Box::new(Error::new(ErrorKind::Other, "Unknown error")));
        }
    };
    let val: Response = serde_json::from_str(&buf)?;
    if val.from_lang == "en" {
        return Err(Box::new(Error::new(ErrorKind::Other, "English text")));
    }
    info!("{:?}", val);
    let messages = format!("From {} to en:\n{}", val.from_lang, val.translated_text);
    Ok(messages)
}
