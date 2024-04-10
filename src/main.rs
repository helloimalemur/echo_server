use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web::http::Method;
use actix_web::web::{Bytes, to};



async fn default_route(req_method: Method, http_request: HttpRequest, bytes: Bytes) -> impl Responder {
    let body = String::from_utf8(bytes.to_vec()).unwrap();
    let response = format!("Method: {}\n\nURL: {}\n\nHeaders: {:?}\n\nBody: {}\n\n Full Request: {:?}", req_method, http_request.path(), http_request.headers(), body, http_request);
    println!("{}", response.clone());
    HttpResponse::Ok().body(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting.. ");
    println!("Listening at 127.0.0.1:8080\n");
    HttpServer::new(|| {
        App::new()
            .default_service(to(default_route))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
