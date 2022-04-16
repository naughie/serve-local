use axum::extract::Path;
use axum::routing::get;
use axum::Router;

use axum::http::{header, HeaderMap, StatusCode};

use serde::Deserialize;

use std::ffi::OsStr;

#[derive(Debug, Deserialize)]
struct Filename {
    fname: String,
}

async fn serve_file(Path(param): Path<Filename>) -> (StatusCode, HeaderMap, Vec<u8>) {
    use std::path::Path;

    let on_fail = || (StatusCode::NOT_FOUND, HeaderMap::new(), Vec::new());

    let fname = if param.fname.ends_with('/') {
        let mut fname = String::with_capacity(param.fname.len() + 11);
        fname.push('.');
        fname.push_str(&param.fname);
        fname.push_str("index.html");
        fname
    } else {
        let mut fname = String::with_capacity(param.fname.len() + 1);
        fname.push('.');
        fname.push_str(&param.fname);
        fname
    };
    eprintln!("Serve {}", &fname);
    let fname: &Path = fname.as_ref();

    if fname.exists() {
        if let Ok(body) = std::fs::read(fname) {
            (StatusCode::OK, headers(fname.extension(), &body), body)
        } else {
            on_fail()
        }
    } else {
        on_fail()
    }
}

fn headers(ext: Option<&OsStr>, bytes: &[u8]) -> HeaderMap {
    fn osstr(s: &str) -> Option<&OsStr> {
        Some(OsStr::new(s))
    }

    use header::HeaderValue;
    use header::{CONTENT_LENGTH, CONTENT_TYPE};

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_LENGTH, bytes.len().into());

    let mut insert = |ty| headers.insert(CONTENT_TYPE, HeaderValue::from_static(ty));

    if ext == osstr("html") {
        insert("text/html");
    } else if ext == osstr("css") {
        insert("text/css");
    } else if ext == osstr("js") {
        insert("application/javascript");
    } else if ext == osstr("txt") || ext == osstr("text") || ext == None {
        insert("text/plain");
    } else if ext == osstr("png") {
        insert("image/png");
    } else if ext == osstr("jpg") || ext == osstr("jpeg") {
        insert("image/jpeg");
    } else if ext == osstr("gif") {
        insert("image/gif");
    } else if ext == osstr("svg") {
        insert("image/svg+xml");
    } else {
        insert("application/octet-stream");
    }

    headers
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/*fname", get(serve_file));

    let mut addr = String::from("0.0.0.0:8080");
    for arg in std::env::args().skip(1) {
        addr = format!("0.0.0.0:{}", arg);
    }

    eprintln!("Listen on {}", &addr);

    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
