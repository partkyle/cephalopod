#![allow(unused_variables)]
extern crate actix;
extern crate actix_web;
extern crate env_logger;
extern crate futures;

#[macro_use]
extern crate serde_derive;
extern crate serde;

extern crate sha1;

use sha1::Sha1;

use actix_web::{
    dev, error, http, middleware, multipart, server, App, Error, FutureResponse, HttpMessage,
    HttpRequest, HttpResponse,
};

use futures::future;
use futures::{Future, Stream};

#[derive(Serialize)]
pub struct ShaFileResult {
    pub content_type: String,
    pub sha: String,
}

pub fn sha_file(
    field: multipart::Field<dev::Payload>,
) -> Box<Future<Item = ShaFileResult, Error = Error>> {
    let content_type = field.content_type().to_string();
    Box::new(
        field
            .fold(Sha1::new(), |mut sha, bytes| {
                sha.update(bytes.as_ref());
                if false {
                    return future::result(Err(error::MultipartError::NoContentType));
                }
                future::result(Ok(sha))
            })
            .map(|sha| ShaFileResult {
                content_type: content_type,
                sha: sha.digest().to_string(),
            })
            .map_err(|e| error::ErrorInternalServerError(e)),
    )
}

pub fn handle_multipart_item(
    item: multipart::MultipartItem<dev::Payload>,
) -> Box<Stream<Item = ShaFileResult, Error = Error>> {
    match item {
        multipart::MultipartItem::Field(field) => Box::new(sha_file(field).into_stream()),
        multipart::MultipartItem::Nested(mp) => Box::new(
            mp.map_err(error::ErrorInternalServerError)
                .map(handle_multipart_item)
                .flatten(),
        ),
    }
}

pub fn upload(req: HttpRequest) -> FutureResponse<HttpResponse> {
    Box::new(
        req.multipart()
            .map_err(error::ErrorInternalServerError)
            .map(handle_multipart_item)
            .flatten()
            .collect()
            .map(|sizes| HttpResponse::Ok().json(sizes))
            .map_err(|e| {
                println!("failed: {}", e);
                e
            }),
    )
}

fn index(_req: HttpRequest) -> Result<HttpResponse, error::Error> {
    let html = r#"<html>
        <head><title>Upload Test</title></head>
        <body>
            <form target="/" method="post" enctype="multipart/form-data">
                <input type="file" name="file"/>
                <input type="submit" value="Submit"></button>
            </form>
        </body>
    </html>"#;

    Ok(HttpResponse::Ok().body(html))
}

fn main() {
    ::std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let sys = actix::System::new("multipart-example");

    server::new(|| {
        App::new()
            .middleware(middleware::Logger::default())
            .resource("/", |r| {
                r.method(http::Method::GET).with(index);
                r.method(http::Method::POST).with(upload);
            })
    })
    .bind("127.0.0.1:8080")
    .unwrap()
    .start();

    println!("Starting http server: 127.0.0.1:8080");
    let _ = sys.run();
}
